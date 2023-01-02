
//! Autogenerated weights for `peaq_pallet_transaction`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-29, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("agung"), DB CACHE: 1024

// Executed Command:
// ./target/release/peaq-node
// benchmark
// pallet
// --chain
// agung
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// peaq_pallet_transaction
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for peaq_pallet_transaction.
pub trait WeightInfo {
	fn service_requested() -> Weight;
	fn service_delivered() -> Weight;
}

/// Weight functions for `peaq_pallet_transaction`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn service_requested() -> Weight {
		Weight::from_ref_time(10_850_000_u64)
	}
	fn service_delivered() -> Weight {
		Weight::from_ref_time(11_512_000_u64)
	}
}
