
//! Autogenerated weights for `pallet_poe`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-06, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `lishengs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/production/solochain-template-node
// benchmark
// pallet
// --wasm-execution=compiled
// --pallet
// pallet_poe
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/poe/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_poe`.
pub trait WeightInfo {
	fn create_claim(c: u32, ) -> Weight;
	fn revoke_claim(c: u32, ) -> Weight;
	fn transfer_claim(c: u32, ) -> Weight;
}

/// Weights for `pallet_poe` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 10]`.
	fn create_claim(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3528`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(21_684_633, 3528)
			// Standard Error: 16_203
			.saturating_add(Weight::from_parts(8_792, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 10]`.
	fn revoke_claim(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + c * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_577_280, 3528)
			// Standard Error: 22_966
			.saturating_add(Weight::from_parts(2_357, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 10]`.
	fn transfer_claim(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + c * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_330_084, 3528)
			// Standard Error: 18_179
			.saturating_add(Weight::from_parts(19_686, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 10]`.
	fn create_claim(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3528`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(21_684_633, 3528)
			// Standard Error: 16_203
			.saturating_add(Weight::from_parts(8_792, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 10]`.
	fn revoke_claim(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + c * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_577_280, 3528)
			// Standard Error: 22_966
			.saturating_add(Weight::from_parts(2_357, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 10]`.
	fn transfer_claim(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + c * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_330_084, 3528)
			// Standard Error: 18_179
			.saturating_add(Weight::from_parts(19_686, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
