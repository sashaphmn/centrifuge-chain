// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge Chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! # Interest Accrual Pallet
//!
//! A pallet for calculating interest accrual on debt.

#![cfg_attr(not(feature = "std"), no_std)]

use std::cmp::Ordering;
use cfg_traits::{
	adjustments::Adjustment,
	interest::{
		FullPeriod, Interest, InterestAccrual, InterestAccrualProvider, InterestModel,
		InterestPayment, InterestRate,
	},
	time::{Period, Seconds, TimeAsSecs},
};
use frame_support::ensure;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_arithmetic::traits::{One, Zero};
use sp_runtime::{
	traits::{
		ensure_pow, AtLeast32BitUnsigned, CheckedAdd, CheckedSub, EnsureAdd, EnsureAddAssign,
		EnsureDiv, EnsureFixedPointNumber, EnsureInto, EnsureMul, EnsureSub, EnsureSubAssign,
		Saturating,
	},
	DispatchError, DispatchResult, FixedPointNumber, FixedPointOperand,
};

pub mod weights;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, TypeInfo, Debug, MaxEncodedLen)]
struct LastUpdated {
	at: Seconds,
	start_next_period: Seconds,
}

impl LastUpdated {
	pub fn new(at: Seconds, start_next_period: Seconds) -> Self {
		Self { at, start_next_period }
	}

	pub fn at(&self) -> Seconds {
		self.at
	}

	pub fn next(&self) -> Seconds {
		self.start_next_period
	}
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, TypeInfo, Debug, MaxEncodedLen)]
pub struct ActiveInterestModel<T: Config> {
	model: InterestModel<T::Rate>,
	activation: Seconds,
	last_updated: LastUpdated,
	deactivation: Option<Seconds>,
	notional: T::Balance,
	interest: T::Balance,
}

#[frame_support::pallet]
pub mod pallet {
	use cfg_types::fixed_point::FixedPointNumberExtension;
	use frame_support::pallet_prelude::*;
	use sp_runtime::traits::checked_pow;

	use super::*;
	use crate::weights::WeightInfo;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(3);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Balance: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaxEncodedLen
			+ FixedPointOperand
			+ From<u64>
			+ From<u128>
			+ TypeInfo
			+ TryInto<u64>;

		/// A fixed-point number which represents an interest rate.
		type Rate: Member
			+ Parameter
			+ Default
			+ core::fmt::Debug
			+ Copy
			+ TypeInfo
			+ FixedPointNumber<Inner = Self::Balance>
			+ MaxEncodedLen
		// TODO: REMOVE
			+ num_traits::CheckedNeg
			+ TryFrom<u128>
			+ Into<u128>;

		type Time: TimeAsSecs;

		type Weights: WeightInfo;
	}

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Emits when the accrual deactivation is in time before the last
		/// update
		AccrualDeactivationInThePast,
		/// Emits when T::Rate is not cnstructable from amount of passed periods
		/// (i.e. from the given u64)
		RateCreationFailed,
	}

	impl<T: Config> Pallet<T> {
		pub fn calculate_interest(
			notional: T::Balance,
			model: &InterestModel<T::Rate>,
			last_updated: LastUpdated,
			at: Seconds,
		) -> Result<Interest<T::Balance>, DispatchError> {
			let rate = model.rate_per_schedule()?;

			let periods = match model.compounding {
				None => model.base()?.with_ref(last_updated.next()).periods_passed(last_updated.at(), at)?,
				Some(compounding) => compounding.with_ref(last_updated.next()).periods_passed(last_updated.at(), at)?,
			};

			let interest = Interest::new(
				periods.try_map_front(|p| {
					Ok::<_, DispatchError>(InterestPayment::new(
						p.from(),
						p.to(),
						p.part().mul_floor(Self::interest_inner(rate, notional, 1)?),
					))
				})?,
				periods.try_map_full(|p| {
					Ok::<_, DispatchError>(FullPeriod::new(
						InterestPayment::new(
							p.from(),
							p.to(),
							Self::interest_inner(rate, notional, p.passed())?,
						),
						p.passed(),
					))
				})?,
				periods.try_map_back(|p| {
					Ok::<_, DispatchError>(InterestPayment::new(
						p.from(),
						p.to(),
						p.part().mul_floor(Self::interest_inner(rate, notional, 1)?),
					))
				})?,
			);

			Ok(interest)
		}

		fn interest_inner(
			rate: T::Rate,
			notional: T::Balance,
			periods: impl TryInto<usize>,
		) -> Result<T::Balance, DispatchError> {
			let rate = rate.ensure_add(T::Rate::one())?;
			let periods = periods.try_into().map_err(|_| "Usize must be u64. qed.")?;

			ensure_pow(rate, periods)?
				.ensure_mul_int(notional)?
				.ensure_sub(notional)
				.map_err(Into::into)
		}
	}
}

impl<T: Config> InterestAccrualProvider<T::Rate, T::Balance> for Pallet<T> {
	type InterestAccrual = ActiveInterestModel<T>;

	fn reference(
		model: impl Into<InterestModel<T::Rate>>,
		at: Seconds,
		start: Seconds,
	) -> Result<Self::InterestAccrual, DispatchError> {
		// TODO: Validate model
		// - is base multiple of period for example

		let model = model.into();
		Ok(ActiveInterestModel {
			model,
			activation: at,
			last_updated: LastUpdated::new(at, model.base()?.start()
			deactivation: None,
			notional: T::Balance::zero(),
			interest: T::Balance::zero(),
		})
	}

	fn unreference(_model: Self::InterestAccrual) -> Result<(), DispatchError> {
		Ok(())
	}
}

impl<T: Config> InterestAccrual<T::Rate, T::Balance> for ActiveInterestModel<T> {
	fn update<F: FnOnce(Interest<T::Balance>) -> Result<(T::Balance, Seconds), DispatchError>>(
		&mut self,
		at: Seconds,
		result: F,
	) -> DispatchResult {
		let interest =
			Pallet::<T>::calculate_interest(self.notional, &self.model, self.last_updated, at)?;

		let (interest, last_updated) = result(interest)?;

		let start_next_period = match last_updated.cmp(&self.last_updated.next()) {
			Ordering::Less | Ordering::Equal=> {self.last_updated.next()}
			Ordering::Greater => {todo!("Find actual next start period")}
		};

		self.last_updated = LastUpdated::new(last_updated, start_next_period);
		self.interest.ensure_add_assign(interest)?;

		Ok(())
	}

	fn notional(&self) -> Result<T::Balance, DispatchError> {
		Ok(self.notional)
	}

	fn interest(&self) -> Result<T::Balance, DispatchError> {
		Ok(self.interest)
	}

	fn debt(&self) -> Result<T::Balance, DispatchError> {
		self.notional.ensure_add(self.interest).map_err(Into::into)
	}

	fn adjust_notional(&mut self, adjustment: Adjustment<T::Balance>) -> Result<(), DispatchError> {
		match adjustment {
			Adjustment::Increase(amount) => {
				self.notional.ensure_add_assign(amount)?;
			}
			Adjustment::Decrease(amount) => {
				self.notional.ensure_sub_assign(amount)?;
			}
		}

		Ok(())
	}

	fn adjust_interest(&mut self, adjustment: Adjustment<T::Balance>) -> Result<(), DispatchError> {
		match adjustment {
			Adjustment::Increase(amount) => {
				self.interest.ensure_add_assign(amount)?;
			}
			Adjustment::Decrease(amount) => {
				self.interest.ensure_sub_assign(amount)?;
			}
		}

		Ok(())
	}

	fn adjust_rate(
		&mut self,
		adjustment: Adjustment<InterestRate<T::Rate>>,
	) -> Result<(), DispatchError> {
		todo!("Adjusting rate is not implemented yet")
	}

	fn adjust_compounding(&mut self, adjustment: Option<Period>) -> Result<(), DispatchError> {
		self.model.compounding = adjustment;

		Ok(())
	}

	fn stop_accrual_at(&mut self, deactivation: Seconds) -> Result<(), DispatchError> {
		ensure!(
			deactivation >= self.last_updated,
			Error::<T>::AccrualDeactivationInThePast
		);

		self.deactivation = Some(deactivation);

		Ok(())
	}

	fn accrued_updated(&self) -> Seconds {
		self.last_updated.at
	}

	fn accrued_since(&self) -> Seconds {
		self.activation
	}

	fn accrued_till(&self) -> Option<Seconds> {
		self.deactivation
	}
}
