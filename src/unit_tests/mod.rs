use hybrid_array::{Array, ArrayOps};
use typenum::{U2, U3, U5};

use crate::{
    iopat,
    traits::{Absorb, Nil, Squeeze},
    ExtraSponge,
};

mod sponge_instance;
use sponge_instance::BasicSponge;

#[test]
fn illegal_api_uses() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/unit_tests/compilation/*.rs");
}

// THis works because we empty the sponge before dropping it
#[test]
fn test_extrasponge_instance() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let basic_sponge = BasicSponge::default();

    let extra_sponge: ExtraSponge<BasicSponge, iopat![Absorb<U5>, Squeeze<U3>]> =
        ExtraSponge::<BasicSponge, iopat![Absorb<U2>, Absorb<U3>, Squeeze<U3>]>::start(
            None,
            basic_sponge,
            &mut start_acc,
        );

    let five_array = [0u8; 5];
    let extra_sponge_2 =
        extra_sponge.absorb(Array::from_core_array(five_array), &mut Vec::default());
    let three_array_out = [0u8; 3];
    let mut three_harray_out = Array::from_core_array(three_array_out);
    let _extra_sponge_3: ExtraSponge<BasicSponge, Nil> =
        extra_sponge_2.squeeze(&mut three_harray_out, &mut Vec::default());
}

// This panics, because we drop it without emptying it
#[should_panic]
#[test]
fn test_extrasponge_instance_drop() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let basic_sponge = BasicSponge::default();

    let _extra_sponge: ExtraSponge<BasicSponge, iopat![Absorb<U5>, Squeeze<U3>]> =
        ExtraSponge::<BasicSponge, iopat![Absorb<U2>, Absorb<U3>, Squeeze<U3>]>::start(
            None,
            basic_sponge,
            &mut start_acc,
        );
}
