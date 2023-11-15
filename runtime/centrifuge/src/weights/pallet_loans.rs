
//! Autogenerated weights for `pallet_loans`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_loans
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_loans.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_loans`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_loans::WeightInfo for WeightInfo<T> {
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Loans LastLoanId (r:1 w:1)
	/// Proof: Loans LastLoanId (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:0 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1229`
		//  Estimated: `4278`
		// Minimum execution time: 80_359_000 picoseconds.
		Weight::from_parts(81_582_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Loans CreatedLoan (r:1 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn borrow(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38314 + n * (340 ±0)`
		//  Estimated: `329491 + n * (340 ±0)`
		// Minimum execution time: 262_477_000 picoseconds.
		Weight::from_parts(276_849_068, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 63_357
			.saturating_add(Weight::from_parts(769_258, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(Weight::from_parts(0, 340).saturating_mul(n.into()))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn repay(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38467 + n * (340 ±0)`
		//  Estimated: `329491 + n * (340 ±0)`
		// Minimum execution time: 194_982_000 picoseconds.
		Weight::from_parts(204_652_794, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 60_262
			.saturating_add(Weight::from_parts(432_312, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_parts(0, 340).saturating_mul(n.into()))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:1 w:0)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(5126), added: 7601, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41174 + n * (340 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 362_864_000 picoseconds.
		Weight::from_parts(382_725_601, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 71_814
			.saturating_add(Weight::from_parts(522_955, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:1 w:0)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(5126), added: 7601, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn admin_write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41425 + n * (340 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 380_787_000 picoseconds.
		Weight::from_parts(401_596_520, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 83_274
			.saturating_add(Weight::from_parts(492_555, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn propose_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `971 + n * (316 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 48_340_000 picoseconds.
		Weight::from_parts(52_931_289, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 35_662
			.saturating_add(Weight::from_parts(376_459, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn apply_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37511 + n * (340 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 103_231_000 picoseconds.
		Weight::from_parts(109_347_888, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 41_439
			.saturating_add(Weight::from_parts(478_536, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Loans CreatedLoan (r:1 w:0)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Loans ClosedLoan (r:0 w:1)
	/// Proof: Loans ClosedLoan (max_values: None, max_size: Some(264), added: 2739, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn close(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37337 + n * (373 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 147_103_000 picoseconds.
		Weight::from_parts(159_337_470, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 54_505
			.saturating_add(Weight::from_parts(514_776, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	fn propose_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `512`
		//  Estimated: `4278`
		// Minimum execution time: 104_615_000 picoseconds.
		Weight::from_parts(105_727_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:0 w:1)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(5126), added: 7601, mode: MaxEncodedLen)
	fn apply_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4887`
		//  Estimated: `8649`
		// Minimum execution time: 111_958_000 picoseconds.
		Weight::from_parts(114_302_000, 0)
			.saturating_add(Weight::from_parts(0, 8649))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: PriceCollector Collection (r:1 w:0)
	/// Proof: PriceCollector Collection (max_values: None, max_size: Some(37026), added: 39501, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:0 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn update_portfolio_valuation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36964 + n * (316 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 87_242_000 picoseconds.
		Weight::from_parts(81_382_100, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 35_691
			.saturating_add(Weight::from_parts(10_473_477, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:1 w:0)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// The range of component `n` is `[2, 8]`.
	fn propose_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37177 + n * (340 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 289_698_000 picoseconds.
		Weight::from_parts(305_495_941, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 108_259
			.saturating_add(Weight::from_parts(937_603, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(326026), added: 328501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:1 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// The range of component `n` is `[2, 8]`.
	fn apply_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37839 + n * (340 ±0)`
		//  Estimated: `329491`
		// Minimum execution time: 298_934_000 picoseconds.
		Weight::from_parts(311_624_277, 0)
			.saturating_add(Weight::from_parts(0, 329491))
			// Standard Error: 107_989
			.saturating_add(Weight::from_parts(1_184_249, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
