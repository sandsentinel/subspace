
//! Autogenerated weights for domain_pallet_executive
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `mac-mini.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/subspace-node
// domain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=domain-pallet-executive
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./domains/pallets/executive/src/weights.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::ParityDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for domain_pallet_executive.
pub trait WeightInfo {
	fn set_code() -> Weight;
}

/// Weights for domain_pallet_executive using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Digest` (r:1 w:1)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: UNKNOWN KEY `0x3a636f6465` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0x3a636f6465` (r:0 w:1)
	fn set_code() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1485`
		// Minimum execution time: 14_298_000_000 picoseconds.
		Weight::from_parts(14_475_000_000, 1485)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `System::Digest` (r:1 w:1)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: UNKNOWN KEY `0x3a636f6465` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0x3a636f6465` (r:0 w:1)
	fn set_code() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1485`
		// Minimum execution time: 14_298_000_000 picoseconds.
		Weight::from_parts(14_475_000_000, 1485)
			.saturating_add(ParityDbWeight::get().reads(1_u64))
			.saturating_add(ParityDbWeight::get().writes(2_u64))
	}
}
