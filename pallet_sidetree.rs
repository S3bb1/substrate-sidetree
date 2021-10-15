
//! Autogenerated weights for `pallet_sidetree`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-15, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/debug/node-template
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_sidetree
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_sidetree`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_sidetree::WeightInfo for WeightInfo<T> {
	// Storage: SidetreeModule TransactionNumber (r:1 w:1)
	// Storage: SidetreeModule AnchoredHashes (r:0 w:1)
	fn anchor_hash(s: u32, ) -> Weight {
		(564_003_000 as Weight)
			// Standard Error: 120_000
			.saturating_add((15_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
