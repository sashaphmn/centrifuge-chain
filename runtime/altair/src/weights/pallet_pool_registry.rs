
//! Autogenerated weights for `pallet_pool_registry`
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
// --pallet=pallet_pool_registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_pool_registry.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_pool_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_registry::WeightInfo for WeightInfo<T> {
	/// Storage: PoolRegistry Pools (r:1 w:1)
	/// Proof: PoolRegistry Pools (max_values: None, max_size: Some(25), added: 2500, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:6 w:5)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
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
		//  Measured:  `675`
		//  Estimated: `19807 + n * (2475 ±0)`
		// Minimum execution time: 153_997 nanoseconds.
		Weight::from_parts(141_907_904, 19807)
			// Standard Error: 45_067
			.saturating_add(Weight::from_parts(16_725_813, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(8))
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
		//  Measured:  `819 + n * (133 ±0)`
		//  Estimated: `9739 + n * (2531 ±0)`
		// Minimum execution time: 49_513 nanoseconds.
		Weight::from_parts(48_756_001, 9739)
			// Standard Error: 15_273
			.saturating_add(Weight::from_parts(2_395_540, 0).saturating_mul(n.into()))
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
		//  Measured:  `910 + n * (167 ±0)`
		//  Estimated: `15599 + n * (2699 ±0)`
		// Minimum execution time: 84_838 nanoseconds.
		Weight::from_parts(78_858_854, 15599)
			// Standard Error: 34_904
			.saturating_add(Weight::from_parts(8_462_777, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 2699).saturating_mul(n.into()))
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
		//  Measured:  `932 + n * (194 ±0)`
		//  Estimated: `16412 + n * (2725 ±0)`
		// Minimum execution time: 72_646 nanoseconds.
		Weight::from_parts(66_591_643, 16412)
			// Standard Error: 37_953
			.saturating_add(Weight::from_parts(8_553_255, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 2725).saturating_mul(n.into()))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolRegistry PoolMetadata (r:0 w:1)
	/// Proof: PoolRegistry PoolMetadata (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 46]`.
	fn set_metadata(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `2703`
		// Minimum execution time: 23_173 nanoseconds.
		Weight::from_parts(24_183_578, 2703)
			// Standard Error: 1_051
			.saturating_add(Weight::from_parts(4_207, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
