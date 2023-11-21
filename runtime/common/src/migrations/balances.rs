// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::TypeInfo, storage::unhashed, traits::OnRuntimeUpgrade, weights::Weight, RuntimeDebug,
	StoragePrefixedMap,
};
use frame_system::AccountInfo;
use sp_arithmetic::traits::Zero;
use sp_core::crypto::AccountId32;
pub use sp_core::sr25519;
use sp_runtime::DispatchError;
use sp_std::{prelude::Vec, vec};

/// All balance information for an account.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct OldAccountData<Balance> {
	/// Non-reserved part of the balance. There may still be restrictions on
	/// this, but it is the total pool what may in principle be transferred,
	/// reserved and used for tipping.
	///
	/// This is the only balance that matters in terms of most operations on
	/// tokens. It alone is used to determine the balance when in the contract
	/// execution environment.
	pub free: Balance,
	/// Balance which is reserved and may not be used at all.
	///
	/// This can still get slashed, but gets slashed last of all.
	///
	/// This balance is a 'reserve' balance that other subsystems use in order
	/// to set aside tokens that are still 'owned' by the account holder, but
	/// which are suspendable. This includes named reserve and unnamed reserve.
	pub reserved: Balance,
	/// The amount that `free` may not drop below when withdrawing for *anything
	/// except transaction fee payment*.
	pub misc_frozen: Balance,
	/// The amount that `free` may not drop below when withdrawing specifically
	/// for transaction fee payment.
	pub fee_frozen: Balance,
}

pub type OldAccountInfo<Index, Balance> = AccountInfo<Index, OldAccountData<Balance>>;

pub type NewAccountData<Balance> = pallet_balances::AccountData<Balance>;

pub type NewAccountInfo<Index, Balance> = AccountInfo<Index, NewAccountData<Balance>>;

pub struct Migration<T: pallet_balances::Config + frame_system::Config>(
	sp_std::marker::PhantomData<T>,
);

impl<T> OnRuntimeUpgrade for Migration<T>
where
	T: frame_system::Config<AccountId = AccountId32, Index = u32>
		+ pallet_balances::Config<Balance = u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, DispatchError> {
		let account_prefix = frame_system::Account::<T>::final_prefix();

		let mut total_count = 0;
		let mut total_reserved = 0;
		let mut total_frozen = 0;

		let mut previous_key = account_prefix.to_vec();

		while let Some(next) = sp_io::storage::next_key(&previous_key) {
			previous_key = next;

			log::info!(
				"Balances Migration - Processing account key - {}",
				hex::encode(previous_key.clone())
			);

			// The difference between the old and new account data structures
			// is the last field of the struct, which is:
			//
			// Old - `free_frozen` - T::Balance
			// New - `flags` - u128
			//
			// During this check, we confirm that both the old and the new can
			// be successfully decoded given the raw data found in storage, and
			// we add specific checks for each version:
			//
			// Old - confirm that `fee_frozen` is zero, as it shouldn't have
			// been used so far
			// New - confirm that `flags` does not have the
			// new logic flag set
			match unhashed::get::<OldAccountInfo<T::Index, T::Balance>>(&previous_key) {
				Some(old) => {
					if !old.data.reserved.is_zero() {
						total_reserved += 1;
					}

					if !old.data.fee_frozen.is_zero() || !old.data.misc_frozen.is_zero() {
						total_frozen += 1;
					}
				}
				None => log::error!("Balances Migration - Error decoding old data"),
			};

			match unhashed::get::<NewAccountInfo<T::Index, T::Balance>>(&previous_key) {
				Some(new) => {
					if new.data.flags.is_new_logic() {
						log::warn!(
							"Balances Migration - New account data with new logic flag enabled"
						)
					}
				}
				None => log::error!("Balances Migration - Error decoding new data"),
			};

			total_count += 1;
		}

		log::info!("Balances Migration - Total accounts - {}", total_count);
		log::info!(
			"Balances Migration - Total accounts with frozen balances - {}",
			total_frozen
		);
		log::info!(
			"Balances Migration - Total accounts with reserved balances - {}",
			total_reserved
		);

		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> Weight {
		// !!! REMOVE TEST ACCOUNT DATA !!!
		//
		// TEST ACCOUNT DATA - START
		let test_account_data = get_test_account_data::<T>();

		for (account_id, account_data) in test_account_data {
			log::info!(
				"Balances Migration - Processing account id - {}",
				hex::encode(account_id.clone())
			);

			let storage_key = frame_system::Account::<T>::hashed_key_for(account_id);

			unhashed::put(storage_key.as_slice(), account_data.encode().as_slice())
		}

		// TEST ACCOUNT DATA - END

		// WE CAN NOT MIGRATE. THIS CODE IS JUST FOR CHECKING IF WE NEED ANYTHING
		// BESIDES THE LAZY MIGRATION FROM PARITY
		// See: https://kflabs.slack.com/archives/C05TBFRBL15/p1699956615421249
		Weight::from_parts(0, 0)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_: Vec<u8>) -> Result<(), DispatchError> {
		Ok(())
	}
}

fn get_test_account_data<T>() -> Vec<(T::AccountId, OldAccountInfo<T::Index, T::Balance>)>
where
	T: frame_system::Config<AccountId = AccountId32, Index = u32>
		+ pallet_balances::Config<Balance = u128>,
{
	vec![
		(
			[1u8; 32].into(),
			OldAccountInfo::<T::Index, T::Balance> {
				nonce: 0,
				consumers: 0,
				providers: 0,
				sufficients: 0,
				data: OldAccountData::<T::Balance> {
					free: 1_000_000_000_000,
					reserved: 0,
					misc_frozen: 0,
					fee_frozen: 0,
				},
			},
		),
		(
			[2u8; 32].into(),
			OldAccountInfo::<T::Index, T::Balance> {
				nonce: 0,
				consumers: 0,
				providers: 0,
				sufficients: 0,
				data: OldAccountData::<T::Balance> {
					free: 1_000_000_000_000,
					reserved: 1_000_000_000_000,
					misc_frozen: 0,
					fee_frozen: 0,
				},
			},
		),
		(
			[3u8; 32].into(),
			OldAccountInfo::<T::Index, T::Balance> {
				nonce: 0,
				consumers: 0,
				providers: 0,
				sufficients: 0,
				data: OldAccountData::<T::Balance> {
					free: 1_000_000_000_000,
					reserved: 1_000_000_000_000,
					misc_frozen: 1_000_000_000_000,
					fee_frozen: 0,
				},
			},
		),
		(
			[4u8; 32].into(),
			OldAccountInfo::<T::Index, T::Balance> {
				nonce: 0,
				consumers: 0,
				providers: 0,
				sufficients: 0,
				data: OldAccountData::<T::Balance> {
					free: 1_000_000_000_000,
					reserved: 1_000_000_000_000,
					misc_frozen: 1_000_000_000_000,
					fee_frozen: 1_000_000_000_000,
				},
			},
		),
		(
			[5u8; 32].into(),
			OldAccountInfo::<T::Index, T::Balance> {
				nonce: 0,
				consumers: 0,
				providers: 0,
				sufficients: 0,
				data: OldAccountData::<T::Balance> {
					free: 1_000_000_000_000,
					reserved: 1_000_000_000_000,
					misc_frozen: 1_000_000_000_000,
					// IS_NEW_LOGIC flag value in the new account data structure
					fee_frozen: 0x80000000_00000000_00000000_00000000u128,
				},
			},
		),
		(
			[6u8; 32].into(),
			OldAccountInfo::<T::Index, T::Balance> {
				nonce: 1,
				consumers: 2,
				providers: 3,
				sufficients: 4,
				data: OldAccountData::<T::Balance> {
					free: 1_000_000_000_000,
					reserved: 1_000_000_000_000,
					misc_frozen: 1_000_000_000_000,
					// IS_NEW_LOGIC flag value in the new account data structure
					fee_frozen: 0x80000000_00000000_00000000_00000000u128,
				},
			},
		),
	]
}
