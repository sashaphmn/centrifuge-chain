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

#![cfg_attr(not(feature = "std"), no_std)]

use cfg_types::investments::Swap;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

pub mod impls;
pub mod types;

pub type SwapOf<T> = Swap<<T as Config>::Balance, <T as Config>::CurrencyId>;
pub type ForeignInvestmentInfoOf<T> = cfg_types::investments::ForeignInvestmentInfo<
	<T as frame_system::Config>::AccountId,
	<T as Config>::InvestmentId,
	crate::types::TokenSwapReason,
>;

#[frame_support::pallet]
pub mod pallet {
	use cfg_traits::{
		investments::{Investment as InvestmentT, InvestmentCollector, TrancheCurrency},
		StatusNotificationHook, TokenSwaps,
	};
	use cfg_types::investments::{
		CollectedAmount, ExecutedForeignCollectRedeem, ExecutedForeignDecrease,
	};
	use frame_support::{
		dispatch::HasCompact,
		pallet_prelude::{DispatchResultWithPostInfo, *},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AtLeast32BitUnsigned;
	use types::{InvestState, RedeemState};

	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: frame_system::WeightInfo;

		/// The source of truth for the balance of accounts
		type Balance: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;

		/// The currency type of transferrable tokens
		type CurrencyId: Parameter
			+ Member
			+ Copy
			+ MaybeSerializeDeserialize
			+ Ord
			+ TypeInfo
			+ MaxEncodedLen;

		/// The pool id type required for the investment identifier
		type PoolId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaxEncodedLen
			+ core::fmt::Debug;

		/// The tranche id type required for the investment identifier
		type TrancheId: Member
			+ Parameter
			+ Default
			+ Copy
			+ MaxEncodedLen
			+ TypeInfo
			+ From<[u8; 16]>;

		/// The investment identifying type required for the investment type
		type InvestmentId: TrancheCurrency<Self::PoolId, Self::TrancheId>
			+ Into<Self::CurrencyId>
			+ Clone
			+ Member
			+ Parameter
			+ Copy
			+ MaxEncodedLen;

		/// The internal investment type which handles the actual investment on
		/// top of the wrapper implementation of this Pallet
		type Investment: InvestmentT<
				Self::AccountId,
				Amount = Self::Balance,
				CurrencyId = Self::CurrencyId,
				Error = DispatchError,
				InvestmentId = Self::InvestmentId,
			> + InvestmentCollector<
				Self::AccountId,
				Error = DispatchError,
				InvestmentId = Self::InvestmentId,
				Result = CollectedAmount<Self::Balance>,
			>;

		/// The default sell price limit for token swaps which defines the
		/// lowest acceptable buy price.
		///
		/// NOTE: Can be removed once we implement a
		/// more sophisticated swap price discovery.
		// TODO(@review): Since we will only support stable coins from the
		// beginning, a global default value could be feasible or do we want to
		// have better granularity?
		#[pallet::constant]
		type DefaultTokenSwapSellPriceLimit: Get<Self::Balance>;

		/// The default minimum fulfillment amount for token swaps.
		///
		/// TODO(@review): Since we will only support stable coins from the
		/// beginning, a global default value could be feasible or do we want to
		/// have better granularity?
		///
		/// NOTE: Can be removed once we implement a more sophisticated swap
		/// price discovery.
		// TODO(@review): Since we will only support stable coins from the
		// beginning, a global default value could be feasible or do we want to
		// have better granularity?
		#[pallet::constant]
		type DefaultTokenMinFulfillmentAmount: Get<Self::Balance>;

		/// The token swap order identifying type
		type TokenSwapOrderId: Parameter
			+ Member
			+ Copy
			+ MaybeSerializeDeserialize
			+ Ord
			+ TypeInfo
			+ MaxEncodedLen;

		/// The type which exposes token swap order functionality such as
		/// placing and cancelling orders
		type TokenSwaps: TokenSwaps<
			Self::AccountId,
			CurrencyId = Self::CurrencyId,
			Balance = Self::Balance,
			OrderId = Self::TokenSwapOrderId,
		>;

		/// The hook type which acts upon a finalized investment decrement.
		type ExecutedDecreaseInvestHook: StatusNotificationHook<
			Id = cfg_types::investments::ForeignInvestmentInfo<
				Self::AccountId,
				Self::InvestmentId,
				(),
			>,
			Status = ExecutedForeignDecrease<Self::Balance, Self::CurrencyId>,
			Error = DispatchError,
		>;

		/// The hook type which acts upon a finalized redemption collection.
		type ExecutedCollectRedeemHook: StatusNotificationHook<
			Id = cfg_types::investments::ForeignInvestmentInfo<
				Self::AccountId,
				Self::InvestmentId,
				(),
			>,
			Status = ExecutedForeignCollectRedeem<Self::Balance, Self::CurrencyId>,
			Error = DispatchError,
		>;

		/// Type which provides a conversion from one currency amount to another
		/// currency amount.
		type CurrencyConverter: cfg_traits::SimpleCurrencyConversion<
			Balance = Self::Balance,
			Currency = Self::CurrencyId,
			Error = DispatchError,
		>;
	}

	/// Maps an investor and their `InvestmentId` to the corresponding
	/// `InvestState`.
	///
	/// NOTE: The lifetime of this storage starts with initializing a currency
	/// swap into the required pool currency and ends upon fully processing the
	/// investment after the potential swap. In case a swap is not required, the
	/// investment starts with `InvestState::InvestmentOngoing`.
	#[pallet::storage]
	pub(super) type InvestmentState<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::InvestmentId,
		InvestState<T::Balance, T::CurrencyId>,
	>;

	/// Maps an investor and their `InvestmentId` to the corresponding
	/// `RedeemState`.
	///
	/// NOTE: The lifetime of this storage starts with increasing a redemption
	/// if there exists a processed investment. It ends with transferring back
	/// the swapped return currency to the corresponding source domain from
	/// which the investment originated. The lifecycle must go through the
	/// following stages:
	/// 	1. Increase redemption --> Initialize storage
	/// 	2. Fully process pending redemption
	/// 	3. Collect redemption
	/// 	4. Trigger swap from pool to return currency
	/// 	5. Completely fulfill swap order
	/// 	6. Transfer back to source domain --> Kill storage entry
	#[pallet::storage]
	pub(super) type RedemptionState<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::InvestmentId,
		RedeemState<T::Balance, T::CurrencyId>,
	>;

	/// Maps `TokenSwapOrders` to `ForeignInvestmentInfo` to implicitly enable
	/// mapping to `InvestmentState` and `RedemptionState`.
	///
	/// NOTE: The storage is immediately killed when the swap order is
	/// completely fulfilled even if the corresponding investment and/or
	/// redemption might not be fully processed.
	#[pallet::storage]
	pub(super) type ForeignInvestmentInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::TokenSwapOrderId, ForeignInvestmentInfoOf<T>>;

	/// Maps an investor and their `InvestmentId` to the corresponding
	/// `TokenSwapOrderId`.
	///
	/// NOTE: The storage is immediately killed when the swap order is
	/// completely fulfilled even if the investment might not be fully
	/// processed.
	#[pallet::storage]
	pub(super) type TokenSwapOrderIds<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::InvestmentId,
		T::TokenSwapOrderId,
	>;

	/// Maps an investor and their `InvestmentId` to the amount of
	/// tranche tokens burned for the conversion into pool currency based on the
	/// fulfillment price(s) during collection.
	///
	/// NOTE: The lifetime of this storage starts with collecting a redemption
	/// in pool currency and ends with having swapped the entire amount to
	/// return currency which is assumed to be asynchronous.
	#[pallet::storage]
	pub(super) type CollectedRedemptionTrancheTokens<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::InvestmentId,
		T::Balance,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ForeignInvestmentUpdated {
			investor: T::AccountId,
			investment_id: T::InvestmentId,
			state: InvestState<T::Balance, T::CurrencyId>,
		},
		ForeignInvestmentCleared {
			investor: T::AccountId,
			investment_id: T::InvestmentId,
		},
		ForeignRedemptionUpdated {
			investor: T::AccountId,
			investment_id: T::InvestmentId,
			state: RedeemState<T::Balance, T::CurrencyId>,
		},
		ForeignRedemptionCleared {
			investor: T::AccountId,
			investment_id: T::InvestmentId,
		},
	}

	#[pallet::error]
	// TODO: Add more errors
	pub enum Error<T> {
		/// Failed to retrieve the `InvestmentInfo` from the given
		/// `TokenSwapOrderId`.
		///
		/// NOTE: We must ensure, this can practically never happen!
		InvestmentInfoNotFound,
		// TODO: Not used at the moment
		/// Failed to retrieve the `RedemptionInfo` from the given
		/// `TokenSwapOrderId`.
		///
		/// NOTE: We must ensure, this can practically never happen!
		RedemptionInfoNotFound,
		/// Failed to retrieve the `TokenSwapReason` from the given
		/// `TokenSwapOrderId`.
		///
		/// NOTE: We must ensure, this can practically never happen!
		TokenSwapReasonNotFound,
		/// Failed to retrieve the `TokenSwapReason` from the given
		/// `TokenSwapOrderId`.
		///
		/// NOTE: We must ensure, this can practically never happen!
		ForeignInvestmentInfoNotFound,
		// TODO: Not used at the moment
		/// Failed to determine whether the corresponding currency can be either
		/// used for payment or payout of an investment.
		///
		/// NOTE: We must ensure, this can practically never happen!
		InvalidInvestmentCurrency,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Attempts to transition an `InvestState` after an epoch execution:
		/// * If the state includes `InvestmentOngoing` and the unprocessed
		///   investment amount is zero, removes `InvestmentOngoing`.
		/// * If the state includes `InvestmentOngoing` and the unprocessed
		///   investment amount is positive, updates the `invest_amount` to the
		///   unprocessed one.
		///
		/// NOOP: If the unprocessed investment amount is zero or the state does
		/// not include `InvestmentOngoing`
		// TODO: weights/benchmark, numbers chosen by rough estimation
		#[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(5, 5).ref_time())]
		#[pallet::call_index(1)]
		pub fn nudge_invest_state(
			origin: OriginFor<T>,
			investor: T::AccountId,
			investment_id: T::InvestmentId,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			if let Some(invest_state) = InvestmentState::<T>::get(&investor, investment_id) {
				let amount_unprocessed_investment =
					T::Investment::investment(&investor, investment_id)?;
				let new_state = invest_state.transition(
					types::InvestTransition::EpochExecution(amount_unprocessed_investment),
				)?;
				Pallet::<T>::apply_invest_state_transition(&investor, investment_id, new_state)?;

				Ok(Some(T::DbWeight::get().reads_writes(5, 5)).into())
			} else {
				Ok(Some(T::DbWeight::get().reads(1)).into())
			}
		}

		/// Attempts to transition a `RedeemState` after an epoch execution:
		/// * If the inner state includes `Redeeming` and the unprocessed
		///   redemption amount is zero, removes `Redeeming`.
		/// * If the inner state includes `Redeeming` and the unprocessed
		///   redemption amount is positive, updates the amount to the
		///   unprocessed one.
		///
		/// NOOP: If the unprocessed redemption amount is zero or the inner
		/// state does not include `Redeeming`.
		// TODO: weights/benchmark, numbers chosen by rough estimation
		#[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(5, 5).ref_time())]
		#[pallet::call_index(2)]
		pub fn nudge_redeem_state(
			origin: OriginFor<T>,
			investor: T::AccountId,
			investment_id: T::InvestmentId,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			if let Some(redeem_state) = RedemptionState::<T>::get(&investor, investment_id) {
				let amount_unprocessed_redemption =
					T::Investment::redemption(&investor, investment_id)?;
				let new_state = redeem_state.transition(
					types::RedeemTransition::EpochExecution(amount_unprocessed_redemption),
				)?;
				Pallet::<T>::apply_redeem_state_transition(&investor, investment_id, new_state)?;

				Ok(Some(T::DbWeight::get().reads_writes(5, 5)).into())
			} else {
				Ok(Some(T::DbWeight::get().reads(1)).into())
			}
		}
	}
}
