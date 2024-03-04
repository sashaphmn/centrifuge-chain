
//! Autogenerated weights for `pallet_transfer_allowlist`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_transfer_allowlist
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_transfer_allowlist.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_transfer_allowlist`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_transfer_allowlist::WeightInfo for WeightInfo<T> {
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn add_transfer_allowance_no_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `603`
		//  Estimated: `3674`
		// Minimum execution time: 86_341_000 picoseconds.
		Weight::from_parts(88_195_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn add_transfer_allowance_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `697`
		//  Estimated: `3674`
		// Minimum execution time: 89_687_000 picoseconds.
		Weight::from_parts(91_340_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn add_allowance_delay_no_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3557`
		// Minimum execution time: 20_117_000 picoseconds.
		Weight::from_parts(20_528_000, 0)
			.saturating_add(Weight::from_parts(0, 3557))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn add_allowance_delay_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `370`
		//  Estimated: `3557`
		// Minimum execution time: 22_362_000 picoseconds.
		Weight::from_parts(23_043_000, 0)
			.saturating_add(Weight::from_parts(0, 3557))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn toggle_allowance_delay_once_future_modifiable() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `3557`
		// Minimum execution time: 22_643_000 picoseconds.
		Weight::from_parts(23_384_000, 0)
			.saturating_add(Weight::from_parts(0, 3557))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn update_allowance_delay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `344`
		//  Estimated: `3557`
		// Minimum execution time: 22_542_000 picoseconds.
		Weight::from_parts(23_444_000, 0)
			.saturating_add(Weight::from_parts(0, 3557))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn purge_allowance_delay_no_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `344`
		//  Estimated: `3557`
		// Minimum execution time: 22_542_000 picoseconds.
		Weight::from_parts(23_103_000, 0)
			.saturating_add(Weight::from_parts(0, 3557))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn purge_allowance_delay_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378`
		//  Estimated: `3557`
		// Minimum execution time: 23_254_000 picoseconds.
		Weight::from_parts(23_964_000, 0)
			.saturating_add(Weight::from_parts(0, 3557))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:0)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn remove_transfer_allowance_delay_present() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `473`
		//  Estimated: `3597`
		// Minimum execution time: 34_294_000 picoseconds.
		Weight::from_parts(34_895_000, 0)
			.saturating_add(Weight::from_parts(0, 3597))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:0)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn remove_transfer_allowance_no_delay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `469`
		//  Estimated: `3597`
		// Minimum execution time: 34_254_000 picoseconds.
		Weight::from_parts(34_976_000, 0)
			.saturating_add(Weight::from_parts(0, 3597))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn purge_transfer_allowance_no_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `879`
		//  Estimated: `3674`
		// Minimum execution time: 81_201_000 picoseconds.
		Weight::from_parts(82_454_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	fn purge_transfer_allowance_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `918`
		//  Estimated: `3674`
		// Minimum execution time: 80_410_000 picoseconds.
		Weight::from_parts(81_341_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
