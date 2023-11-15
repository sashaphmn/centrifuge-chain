
//! Autogenerated weights for `pallet_xcm`
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
// --pallet=pallet_xcm
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_xcm.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn send() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `3776`
		// Minimum execution time: 41_257_000 picoseconds.
		Weight::from_parts(42_038_000, 0)
			.saturating_add(Weight::from_parts(0, 3776))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry LocationToAssetId (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry LocationToAssetId (max_values: None, max_size: None, mode: Measured)
	fn teleport_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `3668`
		// Minimum execution time: 41_086_000 picoseconds.
		Weight::from_parts(42_299_000, 0)
			.saturating_add(Weight::from_parts(0, 3668))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry LocationToAssetId (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry LocationToAssetId (max_values: None, max_size: None, mode: Measured)
	fn reserve_transfer_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `3668`
		// Minimum execution time: 40_947_000 picoseconds.
		Weight::from_parts(41_607_000, 0)
			.saturating_add(Weight::from_parts(0, 3668))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:0 w:1)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn force_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_267_000 picoseconds.
		Weight::from_parts(15_068_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PolkadotXcm SafeXcmVersion (r:0 w:1)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	fn force_default_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_438_000 picoseconds.
		Weight::from_parts(4_668_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PolkadotXcm VersionNotifiers (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	/// Proof Skipped: PolkadotXcm QueryCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm Queries (r:0 w:1)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn force_subscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `3776`
		// Minimum execution time: 48_280_000 picoseconds.
		Weight::from_parts(49_592_000, 0)
			.saturating_add(Weight::from_parts(0, 3776))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: PolkadotXcm VersionNotifiers (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm Queries (r:0 w:1)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn force_unsubscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493`
		//  Estimated: `3958`
		// Minimum execution time: 48_901_000 picoseconds.
		Weight::from_parts(49_612_000, 0)
			.saturating_add(Weight::from_parts(0, 3958))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: PolkadotXcm XcmExecutionSuspended (r:0 w:1)
	/// Proof Skipped: PolkadotXcm XcmExecutionSuspended (max_values: Some(1), max_size: None, mode: Measured)
	fn force_suspension() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_549_000 picoseconds.
		Weight::from_parts(4_779_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:4 w:2)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn migrate_supported_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `11086`
		// Minimum execution time: 24_545_000 picoseconds.
		Weight::from_parts(25_237_000, 0)
			.saturating_add(Weight::from_parts(0, 11086))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PolkadotXcm VersionNotifiers (r:4 w:2)
	/// Proof Skipped: PolkadotXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notifiers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `200`
		//  Estimated: `11090`
		// Minimum execution time: 24_395_000 picoseconds.
		Weight::from_parts(25_247_000, 0)
			.saturating_add(Weight::from_parts(0, 11090))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:5 w:0)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn already_notified_target() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `13572`
		// Minimum execution time: 25_958_000 picoseconds.
		Weight::from_parts(26_619_000, 0)
			.saturating_add(Weight::from_parts(0, 13572))
			.saturating_add(T::DbWeight::get().reads(5))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:2 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn notify_current_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378`
		//  Estimated: `6318`
		// Minimum execution time: 45_615_000 picoseconds.
		Weight::from_parts(46_797_000, 0)
			.saturating_add(Weight::from_parts(0, 6318))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:3 w:0)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn notify_target_migration_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `8654`
		// Minimum execution time: 13_755_000 picoseconds.
		Weight::from_parts(14_176_000, 0)
			.saturating_add(Weight::from_parts(0, 8654))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notify_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `11097`
		// Minimum execution time: 24_986_000 picoseconds.
		Weight::from_parts(25_648_000, 0)
			.saturating_add(Weight::from_parts(0, 11097))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn migrate_and_notify_old_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `382`
		//  Estimated: `11272`
		// Minimum execution time: 54_191_000 picoseconds.
		Weight::from_parts(55_223_000, 0)
			.saturating_add(Weight::from_parts(0, 11272))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
