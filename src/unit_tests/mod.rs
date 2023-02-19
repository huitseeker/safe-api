use typenum::{U2, U3, U5};

use crate::{
    iopat,
    traits::{Absorb, Squeeze},
    ExtraSponge,
};

mod sponge_instance;
use sponge_instance::BasicSponge;

#[test]
fn illegal_api_uses() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/unit_tests/compilation/*.rs");
}

// This panics because we drop it without emptying it
#[should_panic]
#[test]
fn test_extrasponge_instance() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let basic_sponge = BasicSponge::default();

    let _extra_sponge: ExtraSponge<BasicSponge, iopat![Absorb<U5>, Squeeze<U3>]> =
        ExtraSponge::<BasicSponge, iopat![Absorb<U2>, Absorb<U3>, Squeeze<U3>]>::start(
            None,
            basic_sponge,
            &mut start_acc,
        );
}
