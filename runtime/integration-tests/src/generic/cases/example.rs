use cfg_primitives::{AuraId, Balance, CFG};
use frame_support::{assert_ok, traits::Get};

use crate::{
	generic::{
		env::{self, Blocks, Config, Env},
		envs::runtime_env::RuntimeEnv,
		utils::genesis::Genesis,
	},
	utils::accounts::Keyring,
};

const TRANSFER: Balance = 1000 * CFG;
const FEES: Balance = 1 * CFG;

fn transfer_balance<T: Config>() {
	// Set up all GenesisConfig for your initial state
	let mut env = RuntimeEnv::<T>::from_genesis(
		Genesis::default()
			.add(pallet_aura::GenesisConfig::<T> {
				authorities: vec![AuraId::from(sp_core::sr25519::Public([0u8; 32]))],
			})
			.add(pallet_balances::GenesisConfig::<T> {
				balances: vec![(
					Keyring::Alice.to_account_id(),
					T::ExistentialDeposit::get() + FEES + TRANSFER,
				)],
			}),
	);

	assert_ok!(env.submit(
		Keyring::Alice,
		pallet_balances::Call::<T>::transfer {
			dest: Keyring::Bob.into(),
			value: TRANSFER,
		},
	));

	// Pass blocks to evolve the system
	// This call can be called several times in different test places
	// You can choose between evolve the runtime by time or by blocks
	// env.pass(Blocks::BySeconds(60));
	env.pass(Blocks::ByNumber(1));

	// Check the state
	// This call can be called several times in different test places
	env.state(|| {
		/*
		assert_eq!(
			pallet_balances::Pallet::<T>::free_balance(Keyring::Alice.to_account_id()),
			T::ExistentialDeposit::get() + FEES
		);
		*/
		assert_eq!(
			pallet_balances::Pallet::<T>::free_balance(Keyring::Bob.to_account_id()),
			TRANSFER
		);
	});
}

#[test]
fn test_transfer_balance() {
	transfer_balance::<development_runtime::Runtime>();
}
