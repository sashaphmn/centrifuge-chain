
//! Autogenerated weights for `pallet_pool_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_pool_registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_pool_registry.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_pool_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_registry::WeightInfo for WeightInfo<T> {
	/// Storage: PoolRegistry Pools (r:1 w:1)
	/// Proof: PoolRegistry Pools (max_values: None, max_size: Some(25), added: 2500, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:6 w:5)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolSystem AccountDeposit (r:1 w:1)
	/// Proof: PoolSystem AccountDeposit (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Permissions PermissionCount (r:1 w:1)
	/// Proof: Permissions PermissionCount (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	/// Storage: Permissions Permission (r:1 w:1)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:0 w:1)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(5126), added: 7601, mode: MaxEncodedLen)
	/// Storage: PoolSystem PoolDeposit (r:0 w:1)
	/// Proof: PoolSystem PoolDeposit (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn register(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `437`
		//  Estimated: `4278 + n * (2475 ±0)`
		// Minimum execution time: 149_881_000 picoseconds.
		Weight::from_parts(136_284_629, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			// Standard Error: 34_107
			.saturating_add(Weight::from_parts(16_777_625, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:0)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:0)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: PoolSystem ScheduledUpdate (r:0 w:1)
	/// Proof: PoolSystem ScheduledUpdate (max_values: None, max_size: Some(1019), added: 3494, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn update_no_execution(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `754 + n * (133 ±0)`
		//  Estimated: `4278 + n * (2531 ±0)`
		// Minimum execution time: 49_412_000 picoseconds.
		Weight::from_parts(48_190_517, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			// Standard Error: 10_676
			.saturating_add(Weight::from_parts(2_491_327, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:0)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:0)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:1)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolSystem ScheduledUpdate (r:0 w:1)
	/// Proof: PoolSystem ScheduledUpdate (max_values: None, max_size: Some(1019), added: 3494, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn update_and_execute(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `814 + n * (167 ±0)`
		//  Estimated: `6754 + n * (2531 ±0)`
		// Minimum execution time: 85_119_000 picoseconds.
		Weight::from_parts(78_288_077, 0)
			.saturating_add(Weight::from_parts(0, 6754))
			// Standard Error: 26_121
			.saturating_add(Weight::from_parts(8_990_520, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:0)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: PoolSystem ScheduledUpdate (r:1 w:1)
	/// Proof: PoolSystem ScheduledUpdate (max_values: None, max_size: Some(1019), added: 3494, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:0)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:1)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[1, 5]`.
	fn execute_update(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `804 + n * (194 ±0)`
		//  Estimated: `6744 + n * (2531 ±0)`
		// Minimum execution time: 73_097_000 picoseconds.
		Weight::from_parts(66_028_083, 0)
			.saturating_add(Weight::from_parts(0, 6744))
			// Standard Error: 27_568
			.saturating_add(Weight::from_parts(9_266_911, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolRegistry PoolMetadata (r:0 w:1)
	/// Proof: PoolRegistry PoolMetadata (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 46]`.
	fn set_metadata(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `3693`
		// Minimum execution time: 22_342_000 picoseconds.
		Weight::from_parts(23_445_719, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			// Standard Error: 687
			.saturating_add(Weight::from_parts(5_135, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
