
//! Autogenerated weights for `pallet_migration_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_migration_manager
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_migration_manager.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_migration_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_migration_manager::WeightInfo for WeightInfo<T> {
	/// Storage: Migration Status (r:1 w:1)
	/// Proof: Migration Status (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28`
		//  Estimated: `496`
		// Minimum execution time: 17_462 nanoseconds.
		Weight::from_parts(18_124_000, 496)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Migration Status (r:1 w:1)
	/// Proof: Migration Status (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: System Account (r:0 w:100)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 100]`.
	fn migrate_system_account(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `496`
		// Minimum execution time: 19_276 nanoseconds.
		Weight::from_parts(18_344_777, 496)
			// Standard Error: 1_959
			.saturating_add(Weight::from_parts(1_120_674, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: Migration Status (r:1 w:1)
	/// Proof: Migration Status (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn migrate_balances_issuance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `496`
		// Minimum execution time: 19_406 nanoseconds.
		Weight::from_parts(19_827_000, 496)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Migration Status (r:1 w:1)
	/// Proof: Migration Status (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Vesting Vesting (r:10 w:10)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:10 w:10)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:10 w:10)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn migrate_vesting_vesting(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `363 + n * (461 ±0)`
		//  Estimated: `496 + n * (9009 ±0)`
		// Minimum execution time: 67_556 nanoseconds.
		Weight::from_parts(42_386_202, 496)
			// Standard Error: 84_499
			.saturating_add(Weight::from_parts(34_789_516, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 9009).saturating_mul(n.into()))
	}
	/// Storage: Migration Status (r:1 w:1)
	/// Proof: Migration Status (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Proxy Proxies (r:0 w:10)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn migrate_proxy_proxies(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `496`
		// Minimum execution time: 33_683 nanoseconds.
		Weight::from_parts(37_730_635, 496)
			// Standard Error: 74_688
			.saturating_add(Weight::from_parts(6_997_845, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
}
