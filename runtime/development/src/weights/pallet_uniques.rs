
//! Autogenerated weights for `pallet_uniques`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_uniques
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_uniques.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::WeightInfo for WeightInfo<T> {
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281`
		//  Estimated: `2657`
		// Minimum execution time: 37_099 nanoseconds.
		Weight::from_parts(38_051_000, 2657)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `2657`
		// Minimum execution time: 22_001 nanoseconds.
		Weight::from_parts(23_043_000, 2657)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1001 w:1000)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:0 w:1000)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(605), added: 3080, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:0 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(299), added: 2774, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(331), added: 2806, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1000)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `543 + n * (121 ±0) + m * (69 ±0) + a * (346 ±0)`
		//  Estimated: `5270 + n * (2613 ±0)`
		// Minimum execution time: 3_092_729 nanoseconds.
		Weight::from_parts(3_157_930_000, 5270)
			// Standard Error: 34_786
			.saturating_add(Weight::from_parts(13_415_711, 0).saturating_mul(n.into()))
			// Standard Error: 34_786
			.saturating_add(Weight::from_parts(79_883, 0).saturating_mul(m.into()))
			// Standard Error: 34_786
			.saturating_add(Weight::from_parts(754_538, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_proof_size(2613).saturating_mul(n.into()))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `7773`
		// Minimum execution time: 44_463 nanoseconds.
		Weight::from_parts(45_435_000, 7773)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `5270`
		// Minimum execution time: 45_465 nanoseconds.
		Weight::from_parts(46_346_000, 5270)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `5270`
		// Minimum execution time: 36_708 nanoseconds.
		Weight::from_parts(37_470_000, 5270)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:5000 w:5000)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `852 + i * (121 ±0)`
		//  Estimated: `2657 + i * (2613 ±0)`
		// Minimum execution time: 20_338 nanoseconds.
		Weight::from_parts(20_619_000, 2657)
			// Standard Error: 17_567
			.saturating_add(Weight::from_parts(20_039_440, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_proof_size(2613).saturating_mul(i.into()))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `5270`
		// Minimum execution time: 25_197 nanoseconds.
		Weight::from_parts(25_948_000, 5270)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `5270`
		// Minimum execution time: 24_816 nanoseconds.
		Weight::from_parts(25_767_000, 5270)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn freeze_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `2657`
		// Minimum execution time: 18_244 nanoseconds.
		Weight::from_parts(19_156_000, 2657)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn thaw_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `2657`
		// Minimum execution time: 18_525 nanoseconds.
		Weight::from_parts(18_986_000, 2657)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	/// Proof: Uniques OwnershipAcceptance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:2)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `5188`
		// Minimum execution time: 29_224 nanoseconds.
		Weight::from_parts(29_855_000, 5188)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `2657`
		// Minimum execution time: 19_226 nanoseconds.
		Weight::from_parts(19_677_000, 2657)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	fn force_item_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `2657`
		// Minimum execution time: 22_912 nanoseconds.
		Weight::from_parts(23_554_000, 2657)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(331), added: 2806, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1 w:1)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(605), added: 3080, mode: MaxEncodedLen)
	fn set_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `838`
		//  Estimated: `8543`
		// Minimum execution time: 52_367 nanoseconds.
		Weight::from_parts(53_359_000, 8543)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(331), added: 2806, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1 w:1)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(605), added: 3080, mode: MaxEncodedLen)
	fn clear_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1501`
		//  Estimated: `8543`
		// Minimum execution time: 50_173 nanoseconds.
		Weight::from_parts(51_286_000, 8543)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(331), added: 2806, mode: MaxEncodedLen)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `451`
		//  Estimated: `5463`
		// Minimum execution time: 37_420 nanoseconds.
		Weight::from_parts(38_081_000, 5463)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(331), added: 2806, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `838`
		//  Estimated: `5463`
		// Minimum execution time: 38_371 nanoseconds.
		Weight::from_parts(39_043_000, 5463)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:1 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(299), added: 2774, mode: MaxEncodedLen)
	fn set_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `5431`
		// Minimum execution time: 36_998 nanoseconds.
		Weight::from_parts(37_570_000, 5431)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:1 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(299), added: 2774, mode: MaxEncodedLen)
	fn clear_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `740`
		//  Estimated: `5431`
		// Minimum execution time: 35_075 nanoseconds.
		Weight::from_parts(35_977_000, 5431)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `5270`
		// Minimum execution time: 25_527 nanoseconds.
		Weight::from_parts(26_119_000, 5270)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `5270`
		// Minimum execution time: 25_638 nanoseconds.
		Weight::from_parts(25_979_000, 5270)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	/// Proof: Uniques OwnershipAcceptance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	fn set_accept_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `2531`
		// Minimum execution time: 20_518 nanoseconds.
		Weight::from_parts(20_939_000, 2531)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn set_collection_max_supply() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `5160`
		// Minimum execution time: 21_870 nanoseconds.
		Weight::from_parts(22_752_000, 5160)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:0)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	fn set_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `374`
		//  Estimated: `2613`
		// Minimum execution time: 21_751 nanoseconds.
		Weight::from_parts(22_431_000, 2613)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:1 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	fn buy_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `739`
		//  Estimated: `7850`
		// Minimum execution time: 51_215 nanoseconds.
		Weight::from_parts(52_518_000, 7850)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
