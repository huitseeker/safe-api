use std::collections::VecDeque;

use crate::{Error, IOPattern, SpongeAPI, SpongeOp};

#[allow(unreachable_pub)]
pub struct BasicSponge {
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

// This just tests our dummy type BasicSponge, to make sure it works as expected.
#[test]
fn test_start() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let _basic_sponge = BasicSponge::default().start(
        IOPattern(vec![SpongeOp::Absorb(1), SpongeOp::Squeeze(1)]),
        None,
        &mut start_acc,
    );
}

#[test]
fn test_pattern() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let mut basic_sponge = BasicSponge::default();
    basic_sponge.start(
        IOPattern(vec![SpongeOp::Absorb(1), SpongeOp::Squeeze(1)]),
        None,
        &mut start_acc,
    );
    basic_sponge.absorb(1, &[1], &mut Vec::default());
    let mut res = vec![0];
    basic_sponge.squeeze(1, &mut res, &mut Vec::default());
    basic_sponge.finish().unwrap()
}

#[test]
fn test_base_api_doesnt_protect_forgotten_finish() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let mut basic_sponge = BasicSponge::default();
    basic_sponge.start(
        IOPattern(vec![SpongeOp::Absorb(1), SpongeOp::Squeeze(1)]),
        None,
        &mut start_acc,
    );
    basic_sponge.absorb(1, &[1], &mut Vec::default());
    let mut res = vec![0];
    basic_sponge.squeeze(1, &mut res, &mut Vec::default());
}

#[should_panic]
#[test]
fn test_bad_pattern() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let mut basic_sponge = BasicSponge::default();
    basic_sponge.start(
        IOPattern(vec![SpongeOp::Absorb(1), SpongeOp::Squeeze(1)]),
        None,
        &mut start_acc,
    );
    basic_sponge.absorb(1, &[1], &mut Vec::default());
    let mut res = vec![0];
    basic_sponge.squeeze(1, &mut res, &mut Vec::default());
    basic_sponge.absorb(1, &[1], &mut Vec::default());
}

#[should_panic]
#[test]
fn test_bad_finish() {
    let mut start_acc: Vec<u8> = vec![1, 2, 3];
    let mut basic_sponge = BasicSponge::default();
    basic_sponge.start(
        IOPattern(vec![SpongeOp::Absorb(1), SpongeOp::Squeeze(1)]),
        None,
        &mut start_acc,
    );
    basic_sponge.absorb(1, &[1], &mut Vec::default());
    basic_sponge.finish().unwrap();
}
