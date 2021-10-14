use crate::{mock::*, *};
use frame_support::assert_ok;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		let hash = Vec::<u8>::new();
		let anchor = Anchor { hash, operations: 1 };
		// Dispatch a signed extrinsic.
		assert_ok!(SidetreeModule::anchor_hash(Origin::signed(1), anchor));
	});
}
