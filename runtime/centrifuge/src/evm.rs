// Copyright 2023 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_primitives::{
	TwoThirdOfCouncil, BLOCK_STORAGE_LIMIT, MAXIMUM_BLOCK_WEIGHT, MAX_POV_SIZE,
	NORMAL_DISPATCH_RATIO,
};
use frame_support::{parameter_types, traits::FindAuthor, weights::Weight, ConsensusEngineId};
use pallet_ethereum::PostLogContent;
use pallet_evm::{EnsureAddressRoot, EnsureAddressTruncated};
use runtime_common::{
	account_conversion::AccountConverter,
	evm::{precompile::CentrifugePrecompiles, BaseFeeThreshold, WEIGHT_PER_GAS},
	origin::EnsureAccountOrRootOr,
};
use sp_core::{crypto::ByteArray, H160, U256};
use sp_runtime::Permill;
use sp_std::marker::PhantomData;

use crate::{Aura, LocationToAccountId};

// To create valid Ethereum-compatible blocks, we need a 20-byte
// "author" for the block. Since that author is purely informational,
// we do a simple truncation of the 32-byte Substrate author
pub struct FindAuthorTruncated<F>(PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(author_index) = F::find_author(digests) {
			let authority_id = Aura::authorities()[author_index as usize].clone();
			return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]));
		}
		None
	}
}

parameter_types! {
	pub BlockGasLimit: U256 = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
	pub PrecompilesValue: CentrifugePrecompiles<crate::Runtime> = CentrifugePrecompiles::<_>::new();
	pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
	pub GasLimitPovSizeRatio: u64 = {
		let block_gas_limit = BlockGasLimit::get().min(u64::MAX.into()).low_u64();
		block_gas_limit.saturating_div(MAX_POV_SIZE)
	};
	pub GasLimitStorageGrowthRatio: u64 =
		BlockGasLimit::get().min(u64::MAX.into()).low_u64().saturating_div(BLOCK_STORAGE_LIMIT);
}

impl pallet_evm::Config for crate::Runtime {
	type AddressMapping = AccountConverter<crate::Runtime, LocationToAccountId>;
	type BlockGasLimit = BlockGasLimit;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressRoot<crate::AccountId>;
	type ChainId = crate::EVMChainId;
	type Currency = crate::Balances;
	type FeeCalculator = crate::BaseFee;
	type FindAuthor = FindAuthorTruncated<Aura>;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type GasLimitStorageGrowthRatio = GasLimitStorageGrowthRatio;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type PrecompilesType = CentrifugePrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type RuntimeEvent = crate::RuntimeEvent;
	type Timestamp = crate::Timestamp;
	type WeightInfo = ();
	type WeightPerGas = WeightPerGas;
	type WithdrawOrigin = EnsureAddressTruncated;
}

impl pallet_evm_chain_id::Config for crate::Runtime {}

parameter_types! {
	pub DefaultBaseFeePerGas: U256 = U256::from(1_000_000_000);
	pub DefaultElasticity: Permill = Permill::from_parts(125_000);
}

impl pallet_base_fee::Config for crate::Runtime {
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type DefaultElasticity = DefaultElasticity;
	type RuntimeEvent = crate::RuntimeEvent;
	type Threshold = BaseFeeThreshold;
}

parameter_types! {
	pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
	//todo(nuno): revisit this
	pub const ExtraDataLength: u32 = 30;
}

impl pallet_ethereum::Config for crate::Runtime {
	type ExtraDataLength = ExtraDataLength;
	type PostLogContent = PostBlockAndTxnHashes;
	type RuntimeEvent = crate::RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
}

impl pallet_ethereum_transaction::Config for crate::Runtime {}

impl axelar_gateway_precompile::Config for crate::Runtime {
	type AdminOrigin =
		EnsureAccountOrRootOr<crate::liquidity_pools::LpAdminAccount, TwoThirdOfCouncil>;
	type RuntimeEvent = crate::RuntimeEvent;
	type WeightInfo = ();
}
