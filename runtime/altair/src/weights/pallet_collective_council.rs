
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
    /// Storage: Council Members (r:1 w:1)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Proposals (r:1 w:0)
    /// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Voting (r:100 w:100)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Prime (r:0 w:1)
    /// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
    /// The range of component `m` is `[0, 100]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `p` is `[0, 100]`.
    fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
        //  Estimated: `15762 + m * (1967 ±23) + p * (4332 ±23)`
        // Minimum execution time: 23_504_000 picoseconds.
        Weight::from_parts(24_045_000, 0)
            .saturating_add(Weight::from_parts(0, 15762))
            // Standard Error: 62_373
            .saturating_add(Weight::from_parts(4_726_718, 0).saturating_mul(m.into()))
            // Standard Error: 62_373
            .saturating_add(Weight::from_parts(8_779_554, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
            .saturating_add(Weight::from_parts(0, 1967).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 4332).saturating_mul(p.into()))
    }
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    fn execute(b: u32, m: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103 + m * (32 ±0)`
        //  Estimated: `1589 + m * (32 ±0)`
        // Minimum execution time: 24_235_000 picoseconds.
        Weight::from_parts(23_250_984, 0)
            .saturating_add(Weight::from_parts(0, 1589))
            // Standard Error: 36
            .saturating_add(Weight::from_parts(1_948, 0).saturating_mul(b.into()))
            // Standard Error: 371
            .saturating_add(Weight::from_parts(18_660, 0).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
    }
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council ProposalOf (r:1 w:0)
    /// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    fn propose_execute(b: u32, m: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103 + m * (32 ±0)`
        //  Estimated: `3569 + m * (32 ±0)`
        // Minimum execution time: 28_623_000 picoseconds.
        Weight::from_parts(27_427_537, 0)
            .saturating_add(Weight::from_parts(0, 3569))
            // Standard Error: 44
            .saturating_add(Weight::from_parts(1_964, 0).saturating_mul(b.into()))
            // Standard Error: 459
            .saturating_add(Weight::from_parts(27_792, 0).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
    }
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council ProposalOf (r:1 w:1)
    /// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Proposals (r:1 w:1)
    /// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council ProposalCount (r:1 w:1)
    /// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Voting (r:0 w:1)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[2, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `393 + m * (32 ±0) + p * (36 ±0)`
        //  Estimated: `3785 + m * (33 ±0) + p * (36 ±0)`
        // Minimum execution time: 37_019_000 picoseconds.
        Weight::from_parts(36_618_582, 0)
            .saturating_add(Weight::from_parts(0, 3785))
            // Standard Error: 94
            .saturating_add(Weight::from_parts(2_958, 0).saturating_mul(b.into()))
            // Standard Error: 986
            .saturating_add(Weight::from_parts(25_992, 0).saturating_mul(m.into()))
            // Standard Error: 973
            .saturating_add(Weight::from_parts(241_104, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
            .saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Voting (r:1 w:1)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// The range of component `m` is `[5, 100]`.
    fn vote(m: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `842 + m * (64 ±0)`
        //  Estimated: `4306 + m * (64 ±0)`
        // Minimum execution time: 31_509_000 picoseconds.
        Weight::from_parts(32_380_933, 0)
            .saturating_add(Weight::from_parts(0, 4306))
            // Standard Error: 670
            .saturating_add(Weight::from_parts(42_211, 0).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
            .saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
    }
    /// Storage: Council Voting (r:1 w:1)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Proposals (r:1 w:1)
    /// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council ProposalOf (r:0 w:1)
    /// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `431 + m * (64 ±0) + p * (36 ±0)`
        //  Estimated: `3876 + m * (65 ±0) + p * (36 ±0)`
        // Minimum execution time: 40_866_000 picoseconds.
        Weight::from_parts(40_128_300, 0)
            .saturating_add(Weight::from_parts(0, 3876))
            // Standard Error: 900
            .saturating_add(Weight::from_parts(35_686, 0).saturating_mul(m.into()))
            // Standard Error: 878
            .saturating_add(Weight::from_parts(232_805, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: Council Voting (r:1 w:1)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council ProposalOf (r:1 w:1)
    /// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Proposals (r:1 w:1)
    /// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `733 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
        //  Estimated: `4050 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
        // Minimum execution time: 58_508_000 picoseconds.
        Weight::from_parts(58_282_994, 0)
            .saturating_add(Weight::from_parts(0, 4050))
            // Standard Error: 190
            .saturating_add(Weight::from_parts(2_776, 0).saturating_mul(b.into()))
            // Standard Error: 2_014
            .saturating_add(Weight::from_parts(14_102, 0).saturating_mul(m.into()))
            // Standard Error: 1_963
            .saturating_add(Weight::from_parts(284_218, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
            .saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
    }
    /// Storage: Council Voting (r:1 w:1)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Prime (r:1 w:0)
    /// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Proposals (r:1 w:1)
    /// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council ProposalOf (r:0 w:1)
    /// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_disapproved(m: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + m * (64 ±0) + p * (36 ±0)`
        //  Estimated: `3896 + m * (65 ±0) + p * (36 ±0)`
        // Minimum execution time: 44_563_000 picoseconds.
        Weight::from_parts(44_023_999, 0)
            .saturating_add(Weight::from_parts(0, 3896))
            // Standard Error: 807
            .saturating_add(Weight::from_parts(39_142, 0).saturating_mul(m.into()))
            // Standard Error: 787
            .saturating_add(Weight::from_parts(229_511, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: Council Voting (r:1 w:1)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Members (r:1 w:0)
    /// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Prime (r:1 w:0)
    /// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council ProposalOf (r:1 w:1)
    /// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council Proposals (r:1 w:1)
    /// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `753 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
        //  Estimated: `4070 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
        // Minimum execution time: 62_106_000 picoseconds.
        Weight::from_parts(62_287_266, 0)
            .saturating_add(Weight::from_parts(0, 4070))
            // Standard Error: 141
            .saturating_add(Weight::from_parts(2_748, 0).saturating_mul(b.into()))
            // Standard Error: 1_497
            .saturating_add(Weight::from_parts(32_032, 0).saturating_mul(m.into()))
            // Standard Error: 1_459
            .saturating_add(Weight::from_parts(275_044, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
            .saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
    }
    /// Storage: Council Proposals (r:1 w:1)
    /// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Council Voting (r:0 w:1)
    /// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: Council ProposalOf (r:0 w:1)
    /// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
    /// The range of component `p` is `[1, 100]`.
    fn disapprove_proposal(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `260 + p * (32 ±0)`
        //  Estimated: `1745 + p * (32 ±0)`
        // Minimum execution time: 23_244_000 picoseconds.
        Weight::from_parts(24_758_733, 0)
            .saturating_add(Weight::from_parts(0, 1745))
            // Standard Error: 752
            .saturating_add(Weight::from_parts(216_365, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
    }
}
