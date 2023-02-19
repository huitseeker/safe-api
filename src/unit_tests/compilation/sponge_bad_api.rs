use hybrid_array::{Array, ArrayOps};
use std::collections::VecDeque;
use typenum::{U2, U3, U4};

use extra_safe::{
    iopat,
    traits::{Absorb, Squeeze},
    Error, ExtraSponge, IOPattern, SpongeAPI, SpongeOp,
};

// Copied from crate::unit_tests::sponge_instance
// TODO: make this a common test utility
struct BasicSponge {
    elements: Vec<u8>,
    pattern: VecDeque<SpongeOp>,
}

impl BasicSponge {
    fn permute(&mut self, other_elems: &Vec<u8>) {
        self.elements
            .iter_mut()
            .zip(other_elems)
            .for_each(|(a, b)| {
                *a = *a ^ *b;
            });
    }
}

impl Default for BasicSponge {
    fn default() -> Self {
        BasicSponge {
            elements: Vec::new(),
            pattern: VecDeque::new(),
        }
    }
}

// This is a very simple implementation of SpongeAPI, which is used in the tests.
// It is not meant to be used in production. It is spectacularly not API-compliant
impl SpongeAPI for BasicSponge {
    type Acc = Vec<u8>;
    type Value = u8;

    fn start(&mut self, p: IOPattern, _: Option<u32>, acc: &mut Vec<u8>) {
        self.elements = acc.clone();
        self.pattern = p.0.into_iter().collect();
    }

    fn absorb(&mut self, length: u32, elements: &[u8], acc: &mut Vec<u8>) {
        assert_eq!(length as usize, elements.len());
        let word = self.pattern.pop_front().unwrap();
        assert_eq!(word, SpongeOp::Absorb(length));
        self.permute(acc);
        self.elements.extend_from_slice(elements);
    }

    fn squeeze(&mut self, length: u32, elements: &mut [u8], acc: &mut Vec<u8>) {
        assert_eq!(length as usize, elements.len());
        let word = self.pattern.pop_front().unwrap();
        assert_eq!(word, SpongeOp::Squeeze(length));
        self.permute(acc);
        for i in 0..length as usize {
            elements[i] = self.elements[i];
        }
    }

    fn finish(&mut self) -> Result<(), Error> {
        self.pattern
            .is_empty()
            .then(|| ())
            .ok_or(Error::ParameterUsageMismatch)
    }
}
// End of copied code

fn main() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let basic_sponge = BasicSponge::default();

    let extra_sponge: ExtraSponge<BasicSponge, iopat![Absorb<U4>, Squeeze<U3>]> =
        ExtraSponge::<BasicSponge, iopat![Absorb<U2>, Absorb<U2>, Squeeze<U3>]>::start(
            None,
            basic_sponge,
            &mut start_acc,
        );
    let five_array = [0u8; 5];
    // this fails because we are running out of API allowance
    let extra_sponge_2 =
        extra_sponge.absorb(Array::from_core_array(five_array), &mut Vec::default());
}
