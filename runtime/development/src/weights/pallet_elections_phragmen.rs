
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development-local"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/development/src/weights/pallet_elections_phragmen.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 5]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `397 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 41_468_000 picoseconds.
		Weight::from_parts(42_560_017, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 9_642
			.saturating_add(Weight::from_parts(242_626, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 5]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 59_232_000 picoseconds.
		Weight::from_parts(60_716_949, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 18_221
			.saturating_add(Weight::from_parts(152_053, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 5]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 59_071_000 picoseconds.
		Weight::from_parts(60_124_534, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 15_245
			.saturating_add(Weight::from_parts(340_864, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `568`
		//  Estimated: `4764`
		// Minimum execution time: 62_237_000 picoseconds.
		Weight::from_parts(63_410_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `963 + c * (48 ±0)`
		//  Estimated: `2448 + c * (48 ±0)`
		// Minimum execution time: 44_684_000 picoseconds.
		Weight::from_parts(45_756_572, 0)
			.saturating_add(Weight::from_parts(0, 2448))
			// Standard Error: 2_274
			.saturating_add(Weight::from_parts(59_575, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `318 + c * (48 ±0)`
		//  Estimated: `1803 + c * (48 ±0)`
		// Minimum execution time: 40_296_000 picoseconds.
		Weight::from_parts(41_327_773, 0)
			.saturating_add(Weight::from_parts(0, 1803))
			// Standard Error: 2_230
			.saturating_add(Weight::from_parts(72_117, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1177`
		//  Estimated: `2662`
		// Minimum execution time: 56_436_000 picoseconds.
		Weight::from_parts(57_518_000, 0)
			.saturating_add(Weight::from_parts(0, 2662))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `639`
		//  Estimated: `2124`
		// Minimum execution time: 40_667_000 picoseconds.
		Weight::from_parts(41_187_000, 0)
			.saturating_add(Weight::from_parts(0, 2124))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 500_000_000_000 picoseconds.
		Weight::from_parts(500_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1177`
		//  Estimated: `3593`
		// Minimum execution time: 64_871_000 picoseconds.
		Weight::from_parts(65_784_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Elections Voting (r:101 w:100)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:100 w:100)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:100 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: System Account (r:100 w:100)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[50, 100]`.
	/// The range of component `d` is `[0, 50]`.
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1193 + v * (457 ±0)`
		//  Estimated: `4528 + d * (1 ±0) + v * (3774 ±0)`
		// Minimum execution time: 3_491_786_000 picoseconds.
		Weight::from_parts(62_147_002, 0)
			.saturating_add(Weight::from_parts(0, 4528))
			// Standard Error: 78_060
			.saturating_add(Weight::from_parts(69_439_573, 0).saturating_mul(v.into()))
			// Standard Error: 78_060
			.saturating_add(Weight::from_parts(3_241, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:101 w:0)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:10 w:10)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections ElectionRounds (r:1 w:1)
	/// Proof Skipped: Elections ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	/// The range of component `v` is `[1, 100]`.
	/// The range of component `e` is `[100, 500]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (199 ±0) + e * (24 ±0) + v * (248 ±0)`
		//  Estimated: `7737 + c * (1665 ±35) + e * (19 ±0) + v * (2536 ±2)`
		// Minimum execution time: 448_752_000 picoseconds.
		Weight::from_parts(450_275_000, 0)
			.saturating_add(Weight::from_parts(0, 7737))
			// Standard Error: 590_822
			.saturating_add(Weight::from_parts(3_268_250, 0).saturating_mul(c.into()))
			// Standard Error: 117_530
			.saturating_add(Weight::from_parts(6_433_607, 0).saturating_mul(v.into()))
			// Standard Error: 25_523
			.saturating_add(Weight::from_parts(110_406, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 1665).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 19).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2536).saturating_mul(v.into()))
	}
}
