//! Benchmarking setup for pallet-sidetree

use super::*;

#[allow(unused)]
use crate::Pallet as SidetreePallet;
use frame_benchmarking::{
	benchmarks, impl_benchmark_test_suite, sp_std::vec::Vec, whitelisted_caller,
};
use frame_system::RawOrigin;

benchmarks! {
	anchor_hash {
		let s in 0 .. 100;
		let hash = Vec::<u8>::new();
		let anchor = Anchor {
			hash,
			operations: s,
		};
		// The caller account is whitelisted for DB reads/write by the benchmarking macro.
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), anchor)
}

impl_benchmark_test_suite!(SidetreePallet, crate::mock::new_test_ext(), crate::mock::Test);
