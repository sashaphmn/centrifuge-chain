
//! Autogenerated weights for `pallet_pool_system`
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
// --pallet=pallet_pool_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_pool_system.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_pool_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_system::WeightInfo for WeightInfo<T> {
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	fn set_max_reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `657`
		//  Estimated: `5991`
		// Minimum execution time: 29_746 nanoseconds.
		Weight::from_parts(30_197_000, 5991)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:0)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:5 w:0)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:5)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:5)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:5 w:0)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: Investments ClearedInvestOrders (r:0 w:5)
	/// Proof: Investments ClearedInvestOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:0 w:5)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn close_epoch_no_orders(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841 + n * (133 ±0)`
		//  Estimated: `33561 + n * (20298 ±0)`
		// Minimum execution time: 121_947 nanoseconds.
		Weight::from_parts(50_340_346, 33561)
			// Standard Error: 43_332
			.saturating_add(Weight::from_parts(74_108_151, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((8_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(20298).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:1)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:5 w:0)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:5)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:5)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn close_epoch_no_execution(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1007 + n * (133 ±0)`
		//  Estimated: `33561 + n * (17694 ±0)`
		// Minimum execution time: 83_635 nanoseconds.
		Weight::from_parts(54_862_697, 33561)
			// Standard Error: 34_507
			.saturating_add(Weight::from_parts(31_508_266, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(17694).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:0)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:5 w:1)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:5)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:5)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:7 w:3)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Investments ClearedInvestOrders (r:0 w:5)
	/// Proof: Investments ClearedInvestOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:0 w:5)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn close_epoch_execute(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1525 + n * (133 ±0)`
		//  Estimated: `43975 + n * (20298 ±0)`
		// Minimum execution time: 209_289 nanoseconds.
		Weight::from_parts(138_046_977, 43975)
			// Standard Error: 59_696
			.saturating_add(Weight::from_parts(75_664_781, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((8_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(20298).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem EpochExecution (r:1 w:1)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn submit_solution(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493 + n * (249 ±0)`
		//  Estimated: `6533`
		// Minimum execution time: 30_777 nanoseconds.
		Weight::from_parts(31_524_649, 6533)
			// Standard Error: 13_304
			.saturating_add(Weight::from_parts(771_497, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem EpochExecution (r:1 w:1)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:7 w:3)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:1 w:1)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:0)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:0)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Investments ClearedInvestOrders (r:0 w:5)
	/// Proof: Investments ClearedInvestOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:0 w:5)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn execute_epoch(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1503 + n * (605 ±0)`
		//  Estimated: `19974 + n * (17774 ±0)`
		// Minimum execution time: 175_236 nanoseconds.
		Weight::from_parts(124_344_158, 19974)
			// Standard Error: 43_193
			.saturating_add(Weight::from_parts(54_899_238, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(8))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(17774).saturating_mul(n.into()))
	}
}
