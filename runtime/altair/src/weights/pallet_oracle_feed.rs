
//! Autogenerated weights for `pallet_oracle_feed`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_oracle_feed
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_oracle_feed.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_oracle_feed`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_oracle_feed::WeightInfo for WeightInfo<T> {
	/// Storage: OraclePriceFeed FedValues (r:1 w:1)
	/// Proof: OraclePriceFeed FedValues (max_values: None, max_size: Some(711), added: 3186, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn feed_with_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `354`
		//  Estimated: `4176`
		// Minimum execution time: 68_549_000 picoseconds.
		Weight::from_parts(70_262_000, 0)
			.saturating_add(Weight::from_parts(0, 4176))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: OraclePriceFeed FedValues (r:1 w:1)
	/// Proof: OraclePriceFeed FedValues (max_values: None, max_size: Some(711), added: 3186, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn feed_without_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `4176`
		// Minimum execution time: 23_885_000 picoseconds.
		Weight::from_parts(24_597_000, 0)
			.saturating_add(Weight::from_parts(0, 4176))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
