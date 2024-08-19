// Copyright 2024 Centrifuge Foundation (centrifuge.io).
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

use super::*;

/// Replicating Deployer.sol function `_rely`
/// Source: https://github.com/centrifuge/liquidity-pools/blob/4d1d6f7cc5bf5022a83333697ee5040b620c4bdc/script/Deployer.sol#L75
pub fn rely<T: Runtime>(evm: &mut <RuntimeEnv<T> as EnvEvmExtension<T>>::EvmEnv) {
	/* NOTE: This rely is NOT needed as the LocalRouter is not permissioned
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ADAPTER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap(); */

	// Rely on pool manager
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ESCROW,
		"rely",
		Some(&[Token::Address(evm.deployed(names::POOL_MANAGER).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::LP_FACTORY,
		"rely",
		Some(&[Token::Address(evm.deployed(names::POOL_MANAGER).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::TRANCHE_FACTORY,
		"rely",
		Some(&[Token::Address(evm.deployed(names::POOL_MANAGER).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::RESTRICTION_MANAGER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::POOL_MANAGER).address())]),
	)
	.unwrap();

	// Rely on root
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROUTER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::POOL_MANAGER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::INVESTMENT_MANAGER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GATEWAY,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GAS_SERVICE,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ESCROW,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROUTER_ESCROW,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::TRANSFER_PROXY_FACTORY,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::LP_FACTORY,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::TRANCHE_FACTORY,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::RESTRICTION_MANAGER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	)
	.unwrap();

	// Rely on guardian
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROOT,
		"rely",
		Some(&[Token::Address(evm.deployed(names::GUARDIAN).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GATEWAY,
		"rely",
		Some(&[Token::Address(evm.deployed(names::GUARDIAN).address())]),
	)
	.unwrap();

	// Rely on gateway
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROOT,
		"rely",
		Some(&[Token::Address(evm.deployed(names::GATEWAY).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::INVESTMENT_MANAGER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::GATEWAY).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::POOL_MANAGER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::GATEWAY).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GAS_SERVICE,
		"rely",
		Some(&[Token::Address(evm.deployed(names::GATEWAY).address())]),
	)
	.unwrap();

	// Rely on others
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROUTER_ESCROW,
		"rely",
		Some(&[Token::Address(evm.deployed(names::ROUTER).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::INVESTMENT_MANAGER,
		"rely",
		Some(&[Token::Address(evm.deployed(names::LP_FACTORY).address())]),
	)
	.unwrap();
}

/// Replicating Deployer.sol function `_file`
/// Source: https://github.com/centrifuge/liquidity-pools/blob/4d1d6f7cc5bf5022a83333697ee5040b620c4bdc/script/Deployer.sol#L110
pub fn file<T: Runtime>(evm: &mut <RuntimeEnv<T> as EnvEvmExtension<T>>::EvmEnv) {
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::POOL_MANAGER,
		"file",
		Some(&[
			Token::FixedBytes("investmentManager".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::INVESTMENT_MANAGER).address()),
		]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::POOL_MANAGER,
		"file",
		Some(&[
			Token::FixedBytes("gasService".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::GAS_SERVICE).address()),
		]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::POOL_MANAGER,
		"file",
		Some(&[
			Token::FixedBytes("gateway".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::GATEWAY).address()),
		]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::INVESTMENT_MANAGER,
		"file",
		Some(&[
			Token::FixedBytes("poolManager".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::POOL_MANAGER).address()),
		]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::INVESTMENT_MANAGER,
		"file",
		Some(&[
			Token::FixedBytes("gateway".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::GATEWAY).address()),
		]),
	)
	.unwrap();

	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GATEWAY,
		"file",
		Some(&[
			Token::FixedBytes("payers".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::ROUTER).address()),
			Token::Bool(true),
		]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::TRANSFER_PROXY_FACTORY,
		"file",
		Some(&[
			Token::FixedBytes("poolManager".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::POOL_MANAGER).address()),
		]),
	)
	.unwrap();
}

/// Replicate Deployer.sol function `wire`
/// Source: https://github.com/centrifuge/liquidity-pools/blob/4d1d6f7cc5bf5022a83333697ee5040b620c4bdc/script/Deployer.sol#L119
pub fn wire<T: Runtime>(evm: &mut <RuntimeEnv<T> as EnvEvmExtension<T>>::EvmEnv) {
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GATEWAY,
		"file",
		Some(&[
			Token::FixedBytes("adapters".as_bytes().to_vec()),
			Token::Array(vec![Token::Address(evm.deployed(names::ADAPTER).address())]),
		]),
	)
	.unwrap();
}

/// Replicating Deployer.sol function `_endorse`
/// Source: https://github.com/centrifuge/liquidity-pools/blob/4d1d6f7cc5bf5022a83333697ee5040b620c4bdc/script/Deployer.sol#L70
pub fn endorse<T: Runtime>(evm: &mut <RuntimeEnv<T> as EnvEvmExtension<T>>::EvmEnv) {
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROOT,
		"endorse",
		Some(&[Token::Address(evm.deployed(names::ROUTER).address())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROOT,
		"endorse",
		Some(&[Token::Address(evm.deployed(names::ESCROW).address())]),
	)
	.unwrap();
}

/// Replicating Deployer.sol function `removeDeployerAccess`
/// Source: https://github.com/centrifuge/liquidity-pools/blob/4d1d6f7cc5bf5022a83333697ee5040b620c4bdc/script/Deployer.sol#L125
pub fn remove_deployer_access<T: Runtime>(evm: &mut <RuntimeEnv<T> as EnvEvmExtension<T>>::EvmEnv) {
	/* NOTE: This deny is NOT needed as the LocalRouter is not permissioned
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ADAPTER,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap(); */
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::LP_FACTORY,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::TRANCHE_FACTORY,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::RESTRICTION_MANAGER,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::TRANSFER_PROXY_FACTORY,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROOT,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::INVESTMENT_MANAGER,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::POOL_MANAGER,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ESCROW,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROUTER_ESCROW,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GATEWAY,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ROUTER,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::GAS_SERVICE,
		"deny",
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	)
	.unwrap();
}

/// Replicating LocalAdapter.s.sol function `run`
/// Source: https://github.com/centrifuge/liquidity-pools/blob/4d1d6f7cc5bf5022a83333697ee5040b620c4bdc/test/integration/LocalAdapter.s.sol#L19-L21
pub fn local_router<T: Runtime>(evm: &mut <RuntimeEnv<T> as EnvEvmExtension<T>>::EvmEnv) {
	wire::<T>(evm);
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ADAPTER,
		"file",
		Some(&[
			Token::FixedBytes("gateway".as_bytes().to_vec()),
			Token::Address(evm.deployed(names::GATEWAY).address()),
		]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ADAPTER,
		"file",
		Some(&[
			Token::FixedBytes("sourceChain".as_bytes().to_vec()),
			Token::String(EVM_DOMAIN_STR.to_string()),
		]),
	)
	.unwrap();
	evm.call(
		Keyring::Alice,
		Default::default(),
		names::ADAPTER,
		"file",
		Some(&[
			Token::FixedBytes("sourceAddress".as_bytes().to_vec()),
			Token::String("0x1111111111111111111111111111111111111111".into()),
		]),
	)
	.unwrap();
}

/// Replicating Deployer.sol function `deploy`
/// Source: https://github.com/centrifuge/liquidity-pools/blob/4d1d6f7cc5bf5022a83333697ee5040b620c4bdc/script/Deployer.sol#L39
pub fn deployer_script<T: Runtime>(evm: &mut <RuntimeEnv<T> as EnvEvmExtension<T>>::EvmEnv) {
	evm.deploy(
		contracts::ESCROW,
		names::ESCROW,
		Keyring::Alice,
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	);
	evm.deploy(
		contracts::ESCROW,
		names::ROUTER_ESCROW,
		Keyring::Alice,
		Some(&[Token::Address(Keyring::Alice.in_eth())]),
	);
	evm.deploy(
		contracts::ROOT,
		names::ROOT,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ESCROW).address()),
			Token::Uint(U256::from(48 * SECONDS_PER_HOUR)),
			Token::Address(Keyring::Alice.in_eth()),
		]),
	);
	evm.deploy(
		contracts::LP_FACTORY,
		names::LP_FACTORY,
		Keyring::Alice,
		Some(&[Token::Address(evm.deployed(names::ROOT).address())]),
	);
	evm.deploy(
		contracts::RESTRICTION_MANAGER,
		names::RESTRICTION_MANAGER,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ROOT).address()),
			Token::Address(Keyring::Alice.in_eth()),
		]),
	);
	evm.deploy(
		contracts::TRANCHE_FACTORY,
		names::TRANCHE_FACTORY,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ROOT).address()),
			Token::Address(Keyring::Alice.in_eth()),
		]),
	);
	evm.deploy(
		contracts::INVESTMENT_MANAGER,
		names::INVESTMENT_MANAGER,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ROOT).address()),
			Token::Address(evm.deployed(names::ESCROW).address()),
		]),
	);
	evm.deploy(
		contracts::POOL_MANAGER,
		names::POOL_MANAGER,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ESCROW).address()),
			Token::Address(evm.deployed(names::LP_FACTORY).address()),
			Token::Address(evm.deployed(names::TRANCHE_FACTORY).address()),
		]),
	);
	evm.deploy(
		contracts::TRANSFER_PROXY_FACTORY,
		names::TRANSFER_PROXY_FACTORY,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ROOT).address()),
			Token::Address(Keyring::Alice.in_eth()),
		]),
	);
	evm.deploy(
		contracts::GAS_SERVICE,
		names::GAS_SERVICE,
		Keyring::Alice,
		Some(&[
			Token::Uint(Uint::from(gas::MSG_COST).into()),
			Token::Uint(Uint::from(gas::PROOF_COST).into()),
			Token::Uint(U128::from(gas::GAS_PRICE).into()),
			Token::Uint(U256::from(gas::TOKEN_PRICE).into()),
		]),
	);
	evm.deploy(
		contracts::GATEWAY,
		names::GATEWAY,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ROOT).address()),
			Token::Address(evm.deployed(names::POOL_MANAGER).address()),
			Token::Address(evm.deployed(names::INVESTMENT_MANAGER).address()),
			Token::Address(evm.deployed(names::GAS_SERVICE).address()),
		]),
	);
	evm.deploy(
		contracts::ROUTER,
		names::ROUTER,
		Keyring::Alice,
		Some(&[
			Token::Address(evm.deployed(names::ROUTER_ESCROW).address()),
			Token::Address(evm.deployed(names::GATEWAY).address()),
			Token::Address(evm.deployed(names::POOL_MANAGER).address()),
		]),
	);
	evm.deploy(
		contracts::GUARDIAN,
		names::GUARDIAN,
		Keyring::Alice,
		Some(&[
			// Based on https://github.com/centrifuge/liquidity-pools/blob/da4e46577712c762d069670077280112ea1c8ce8/test/integration/LocalAdapter.s.sol#L12-L14
			Token::Address(Keyring::Admin.in_eth()),
			Token::Address(evm.deployed(names::ROOT).address()),
			Token::Address(evm.deployed(names::GATEWAY).address()),
		]),
	);
}
