
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("centrifuge-local")`, DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-local
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_elections_phragmen.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(209), added: 2684, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 5]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `397 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 39_854_000 picoseconds.
		Weight::from_parts(41_059_638, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 9_339
			.saturating_add(Weight::from_parts(122_202, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(209), added: 2684, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 5]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 55_103_000 picoseconds.
		Weight::from_parts(56_171_173, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 14_770
			.saturating_add(Weight::from_parts(204_563, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(209), added: 2684, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 5]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 55_143_000 picoseconds.
		Weight::from_parts(55_848_022, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 12_961
			.saturating_add(Weight::from_parts(304_036, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(209), added: 2684, mode: `MaxEncodedLen`)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `568`
		//  Estimated: `4764`
		// Minimum execution time: 56_095_000 picoseconds.
		Weight::from_parts(57_527_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 20]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1475 + c * (48 ±0)`
		//  Estimated: `2960 + c * (48 ±0)`
		// Minimum execution time: 40_225_000 picoseconds.
		Weight::from_parts(41_460_863, 0)
			.saturating_add(Weight::from_parts(0, 2960))
			// Standard Error: 2_567
			.saturating_add(Weight::from_parts(92_423, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 20]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `318 + c * (48 ±0)`
		//  Estimated: `1803 + c * (48 ±0)`
		// Minimum execution time: 35_306_000 picoseconds.
		Weight::from_parts(36_290_646, 0)
			.saturating_add(Weight::from_parts(0, 1803))
			// Standard Error: 2_574
			.saturating_add(Weight::from_parts(70_605, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1621`
		//  Estimated: `3106`
		// Minimum execution time: 50_805_000 picoseconds.
		Weight::from_parts(51_927_000, 0)
			.saturating_add(Weight::from_parts(0, 3106))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1023`
		//  Estimated: `2508`
		// Minimum execution time: 36_037_000 picoseconds.
		Weight::from_parts(36_938_000, 0)
			.saturating_add(Weight::from_parts(0, 2508))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 500_000_000_000 picoseconds.
		Weight::from_parts(500_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1724`
		//  Estimated: `6196`
		// Minimum execution time: 71_003_000 picoseconds.
		Weight::from_parts(72_324_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Elections::Voting` (r:51 w:50)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:50 w:50)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:50 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(209), added: 2684, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:50 w:50)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[50, 100]`.
	/// The range of component `d` is `[0, 50]`.
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + d * (494 ±0) + v * (62 ±0)`
		//  Estimated: `8032 + d * (3774 ±0) + v * (27 ±0)`
		// Minimum execution time: 5_440_000 picoseconds.
		Weight::from_parts(5_851_000, 0)
			.saturating_add(Weight::from_parts(0, 8032))
			// Standard Error: 7_490
			.saturating_add(Weight::from_parts(452_735, 0).saturating_mul(v.into()))
			// Standard Error: 16_345
			.saturating_add(Weight::from_parts(54_859_868, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(d.into())))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 27).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:101 w:0)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Elections::ElectionRounds` (r:1 w:1)
	/// Proof: `Elections::ElectionRounds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 20]`.
	/// The range of component `v` is `[1, 100]`.
	/// The range of component `e` is `[100, 500]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + e * (23 ±0) + v * (241 ±0)`
		//  Estimated: `9323 + c * (154 ±35) + e * (19 ±1) + v * (2526 ±7)`
		// Minimum execution time: 288_329_000 picoseconds.
		Weight::from_parts(291_985_000, 0)
			.saturating_add(Weight::from_parts(0, 9323))
			// Standard Error: 776_472
			.saturating_add(Weight::from_parts(1_654_140, 0).saturating_mul(c.into()))
			// Standard Error: 154_461
			.saturating_add(Weight::from_parts(8_253_388, 0).saturating_mul(v.into()))
			// Standard Error: 33_543
			.saturating_add(Weight::from_parts(253_004, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(Weight::from_parts(0, 154).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 19).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2526).saturating_mul(v.into()))
	}
}
