
//! Autogenerated weights for `pallet_loans_ref`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_loans_ref
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_loans_ref.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_loans_ref`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_loans_ref::WeightInfo for WeightInfo<T> {
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: PoolSystem Pool (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Loans LastLoanId (r:1 w:1)
	// Storage: Loans CreatedLoan (r:0 w:1)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn create() -> Weight {
		// Minimum execution time: 105_202 nanoseconds.
		Weight::from_ref_time(106_302_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Loans CreatedLoan (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: InterestAccrual Rates (r:1 w:1)
	// Storage: InterestAccrual LastUpdated (r:1 w:0)
	// Storage: Loans PortfolioValuation (r:1 w:1)
	// Storage: Loans ActiveLoans (r:1 w:1)
	// Storage: PoolSystem Pool (r:1 w:1)
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	/// The range of component `n` is `[1, 299]`.
	fn borrow(n: u32, ) -> Weight {
		// Minimum execution time: 196_903 nanoseconds.
		Weight::from_ref_time(195_042_460)
			// Standard Error: 6_848
			.saturating_add(Weight::from_ref_time(1_026_277).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Loans PortfolioValuation (r:1 w:1)
	// Storage: Loans ActiveLoans (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: InterestAccrual Rates (r:1 w:0)
	// Storage: InterestAccrual LastUpdated (r:1 w:0)
	// Storage: PoolSystem Pool (r:1 w:1)
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	/// The range of component `n` is `[1, 299]`.
	fn repay(n: u32, ) -> Weight {
		// Minimum execution time: 166_603 nanoseconds.
		Weight::from_ref_time(162_750_552)
			// Standard Error: 6_810
			.saturating_add(Weight::from_ref_time(1_031_483).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Loans PortfolioValuation (r:1 w:1)
	// Storage: Loans ActiveLoans (r:1 w:1)
	// Storage: Loans WriteOffPolicy (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: InterestAccrual Rates (r:1 w:1)
	// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// The range of component `n` is `[1, 299]`.
	fn write_off(n: u32, ) -> Weight {
		// Minimum execution time: 263_205 nanoseconds.
		Weight::from_ref_time(256_972_263)
			// Standard Error: 6_444
			.saturating_add(Weight::from_ref_time(1_043_150).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: Loans PortfolioValuation (r:1 w:1)
	// Storage: Loans ActiveLoans (r:1 w:1)
	// Storage: Loans WriteOffPolicy (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: InterestAccrual Rates (r:1 w:1)
	// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// The range of component `n` is `[1, 299]`.
	fn admin_write_off(n: u32, ) -> Weight {
		// Minimum execution time: 283_205 nanoseconds.
		Weight::from_ref_time(276_944_904)
			// Standard Error: 7_164
			.saturating_add(Weight::from_ref_time(1_043_195).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Loans CreatedLoan (r:1 w:0)
	// Storage: Loans ActiveLoans (r:1 w:1)
	// Storage: InterestAccrual Rates (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Loans ClosedLoan (r:0 w:1)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// The range of component `n` is `[1, 299]`.
	fn close(n: u32, ) -> Weight {
		// Minimum execution time: 118_402 nanoseconds.
		Weight::from_ref_time(112_255_619)
			// Standard Error: 6_961
			.saturating_add(Weight::from_ref_time(991_554).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: PoolSystem Pool (r:1 w:0)
	// Storage: Loans WriteOffPolicy (r:0 w:1)
	fn update_write_off_policy() -> Weight {
		// Minimum execution time: 108_502 nanoseconds.
		Weight::from_ref_time(111_501_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PoolSystem Pool (r:1 w:0)
	// Storage: InterestAccrual Rates (r:1 w:0)
	// Storage: Loans ActiveLoans (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Loans PortfolioValuation (r:1 w:1)
	/// The range of component `n` is `[1, 300]`.
	fn update_portfolio_valuation(n: u32, ) -> Weight {
		// Minimum execution time: 85_902 nanoseconds.
		Weight::from_ref_time(84_278_313)
			// Standard Error: 3_513
			.saturating_add(Weight::from_ref_time(11_474_528).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
