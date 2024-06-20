use cfg_traits::time::Millis;
use frame_support::{derive_impl, parameter_types, traits::Hooks};
use sp_io::TestExternalities;
use sp_runtime::BuildStorage;

use crate::*;

pub type Balance = u128;
pub type Rate = cfg_types::fixed_point::FixedU128<1_000_000_000_000>;

frame_support::construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		MockTime: cfg_mocks::pallet_mock_time,
		InterestAccrual: crate,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = frame_system::mocking::MockBlock<Runtime>;
}

impl cfg_mocks::pallet_mock_time::Config for Runtime {
	type Moment = Millis;
}

impl Config for Runtime {
	type Balance = Balance;
	type Rate = Rate;
	type RuntimeEvent = RuntimeEvent;
	type Time = MockTime;
	type Weights = ();
}

pub const SECONDS: u64 = 1000;
pub const START_DATE: u64 = 1640995200;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let storage = frame_system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap();

	let mut externalities = TestExternalities::new(storage);
	externalities.execute_with(|| {
		System::set_block_number(1);
		System::on_initialize(System::block_number());
		MockTime::mock_now(|| Millis::const_from(START_DATE * SECONDS));
	});
	externalities
}
