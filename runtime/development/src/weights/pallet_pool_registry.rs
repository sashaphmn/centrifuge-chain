//! Autogenerated weights for pallet_pools
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION
//! 4.0.0-dev DATE: 2022-06-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH
//! RANGE: `[]` EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN:
//! Some("development-local"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet_pools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/development/src/weights/pallet_pools.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weights for pallet_pools using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_registry::weights::WeightInfo for WeightInfo<T> {
	fn register(n: u32) -> Weight {
		Weight::from_ref_time(74_584_000) // Standard Error: 70_000
			.saturating_add(Weight::from_ref_time(536_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}

	fn update_no_execution(n: u32) -> Weight {
		Weight::from_ref_time(28_723_000) // Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(224_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn execute_update(n: u32) -> Weight {
		Weight::from_ref_time(45_439_000) // Standard Error: 64_000
			.saturating_add(Weight::from_ref_time(1_074_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn update_and_execute(n: u32) -> Weight {
		Weight::from_ref_time(47_712_000) // Standard Error: 28_000
			.saturating_add(Weight::from_ref_time(876_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn set_metadata(n: u32) -> Weight {
		Weight::from_ref_time(35_549_000) // Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(2_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
