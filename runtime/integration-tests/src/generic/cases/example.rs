use cfg_primitives::{Balance, CFG, SECONDS_PER_YEAR};
use cfg_traits::IntoSeconds;
use frame_support::traits::Get;
use sp_api::runtime_decl_for_core::CoreV4;

use crate::{
	generic::{
		config::Runtime,
		env::{Blocks, Env},
		envs::{
			fudge_env::{FudgeEnv, FudgeSupport},
			runtime_env::RuntimeEnv,
		},
		utils::genesis::Genesis,
	},
	utils::accounts::Keyring,
};

#[test_runtimes([development, altair, centrifuge])]
fn transfer_balance<T: Runtime>() {
	const TRANSFER: Balance = 1000 * CFG;
	const FOR_FEES: Balance = 1 * CFG;

	// Set up all GenesisConfig for your initial state
	// You can choose `RuntimeEnv` by `FudgeEnv` to make it working with fudge
	// environment.
	let mut env = RuntimeEnv::<T>::from_parachain_storage(
		Genesis::default()
			.add(pallet_balances::GenesisConfig::<T> {
				balances: vec![(
					Keyring::Alice.id(),
					T::ExistentialDeposit::get() + FOR_FEES + TRANSFER,
				)],
			})
			.storage(),
	);

	// Call an extrinsic that would be processed immediately
	let fee = env
		.submit_now(
			Keyring::Alice,
			pallet_balances::Call::transfer_allow_death {
				dest: Keyring::Bob.into(),
				value: TRANSFER,
			},
		)
		.unwrap();

	// Check for an even occurred in this block
	env.check_event(pallet_balances::Event::Transfer {
		from: Keyring::Alice.id(),
		to: Keyring::Bob.id(),
		amount: TRANSFER,
	})
	.unwrap();

	// Check the state
	env.parachain_state(|| {
		assert_eq!(
			pallet_balances::Pallet::<T>::free_balance(Keyring::Alice.id()),
			T::ExistentialDeposit::get() + FOR_FEES - fee,
		);
		assert_eq!(
			pallet_balances::Pallet::<T>::free_balance(Keyring::Bob.id()),
			TRANSFER
		);
	});

	// Pass blocks to evolve the system
	env.pass(Blocks::ByNumber(1));
}

// Identical to `transfer_balance()` test but using fudge.
#[test_runtimes([development, altair, centrifuge])]
fn fudge_transfer_balance<T: Runtime + FudgeSupport>() {
	const TRANSFER: Balance = 1000 * CFG;
	const FOR_FEES: Balance = 1 * CFG;

	let mut env = FudgeEnv::<T>::from_parachain_storage(
		Genesis::default()
			.add(pallet_balances::GenesisConfig::<T> {
				balances: vec![(
					Keyring::Alice.id(),
					T::ExistentialDeposit::get() + FOR_FEES + TRANSFER,
				)],
			})
			.storage(),
	);

	env.submit_later(
		Keyring::Alice,
		pallet_balances::Call::transfer_allow_death {
			dest: Keyring::Bob.into(),
			value: TRANSFER,
		},
	)
	.unwrap();

	// submit-later will only take effect if a block has passed
	env.pass(Blocks::ByNumber(1));

	// Check for an even occurred in this block
	env.check_event(pallet_balances::Event::Transfer {
		from: Keyring::Alice.id(),
		to: Keyring::Bob.id(),
		amount: TRANSFER,
	})
	.unwrap();

	// Look for the fee for the last transaction
	let fee = env
		.find_event(|e| match e {
			pallet_transaction_payment::Event::TransactionFeePaid { actual_fee, .. } => {
				Some(actual_fee)
			}
			_ => None,
		})
		.unwrap();

	// Check the state
	env.parachain_state(|| {
		assert_eq!(
			pallet_balances::Pallet::<T>::free_balance(Keyring::Alice.id()),
			T::ExistentialDeposit::get() + FOR_FEES - fee,
		);
		assert_eq!(
			pallet_balances::Pallet::<T>::free_balance(Keyring::Bob.id()),
			TRANSFER
		);
	});
}

#[test_runtimes(all)]
fn call_api<T: Runtime>() {
	let env = RuntimeEnv::<T>::default();

	env.parachain_state(|| {
		// If imported the trait: sp_api::runtime_decl_for_core::CoreV4,
		// you can easily do: T::Api::version()
		assert_eq!(
			T::Api::version(),
			<T as frame_system::Config>::Version::get()
		);
	})
}

#[test_runtimes(all)]
fn fudge_call_api<T: Runtime + FudgeSupport>() {
	let env = FudgeEnv::<T>::default();

	// Exclusive from fudge environment.
	// It uses a client to access the runtime api.
	env.with_api(|api, latest| {
		// We include the API we want to use
		use sp_api::Core;

		let result = api.version(latest).unwrap();

		assert_eq!(result, T::Api::version());
		assert_eq!(result, <T as frame_system::Config>::Version::get());
	})
}

#[test_runtimes(all)]
fn pass_time_one_block<T: Runtime>() {
	let mut env = RuntimeEnv::<T>::default();

	let before = env.parachain_state(|| pallet_timestamp::Pallet::<T>::get());

	// Not supported in fudge
	env.pass(Blocks::JumpBySeconds(SECONDS_PER_YEAR));

	let after = env.parachain_state(|| pallet_timestamp::Pallet::<T>::get());

	assert_eq!((after - before).into_seconds(), SECONDS_PER_YEAR)
}
