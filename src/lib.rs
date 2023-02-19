#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(rustdoc::broken_intra_doc_links)]
pub mod traits;

use hybrid_array::{Array, ArraySize};
use traits::{Absorb, Cons, Consume, IOWord, List, Nil, Norm, Normalize, Squeeze, Use};
use typenum::Unsigned;

#[derive(Debug)]
pub enum Error {
    ParameterUsageMismatch,
}

/// The SpongeWord type is lifted straight from the Neptune codebase.
/// See https://github.com/filecoin-project/neptune/blob/master/src/sponge/api.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpongeOp {
    Absorb(u32),
    Squeeze(u32),
}

/// Conversion from a type-level IOWord to a crate::SpongeOp
/// This is, morally speaking, an extension trait of the IOWord trait,
/// though Rust can of course not check exhaustivity.
pub trait ToSpongeOp: IOWord {
    fn to_sponge_op() -> SpongeOp;
}

impl<U: Unsigned> ToSpongeOp for Absorb<U> {
    fn to_sponge_op() -> SpongeOp {
        SpongeOp::Absorb(U::to_u32())
    }
}

impl<U: Unsigned> ToSpongeOp for Squeeze<U> {
    fn to_sponge_op() -> SpongeOp {
        SpongeOp::Squeeze(U::to_u32())
    }
}

#[derive(Clone, Debug)]
pub struct IOPattern(pub Vec<SpongeOp>);

// TODO : convert SpongeOp -> IOWord using macros

/// Conversion from a trait::List type-level IOPattern to a crate::IOpattern
/// This is morally an extension trait of the List trait, though Rust can of
/// course not check exhaustivity.
pub trait ToIOPattern {
    fn to_iopattern() -> IOPattern;
}

impl ToIOPattern for Nil {
    fn to_iopattern() -> IOPattern {
        IOPattern(vec![])
    }
}
impl<Item: ToSpongeOp, T: List + ToIOPattern> ToIOPattern for Cons<Item, T> {
    fn to_iopattern() -> IOPattern {
        // TODO: avoid the quadratic cost of prepending here
        let mut v = T::to_iopattern().0;
        v.push(<Item as ToSpongeOp>::to_sponge_op());
        IOPattern(v)
    }
}

/// This is the SpongeAPI trait as you can find it in Neptune,
/// see https://github.com/filecoin-project/neptune/blob/master/src/sponge/api.rs
/// Slightly modified so that the squeeze function takes an argument as a mutable slice
/// instead of returning a Vec.
pub trait SpongeAPI {
    type Acc;
    type Value;

    /// Optional `domain_separator` defaults to 0
    fn start(&mut self, p: IOPattern, domain_separator: Option<u32>, acc: &mut Self::Acc);
    fn absorb(&mut self, length: u32, elements: &[Self::Value], acc: &mut Self::Acc);
    // This differs from the original API in that it takes a mutable slice instead of returning a Vec.
    fn squeeze(&mut self, length: u32, elements: &mut [Self::Value], acc: &mut Self::Acc);
    // This differs from the original API in that if does not take a final Self::Acc argument.
    // It would not be impossible to do it without this change, but it would require depending on something other
    // than the Drop impelementation to detect the ExtraSponge going out of scope (e.g. MIRAI).
    fn finish(&mut self) -> Result<(), Error>;
}

/// This is a slightly extended generic NewType wrapper around the original SpongeAPI.
/// It is decorated with the IOPattern I intended for this sponge instance.
#[derive(Debug)]
pub struct ExtraSponge<A: SpongeAPI, I: List> {
    api: A,
    _current_pattern: I,
}

impl<A: SpongeAPI, I: List> ExtraSponge<A, I> {
    pub fn new(api: A) -> ExtraSponge<A, I> {
        ExtraSponge {
            api,
            _current_pattern: I::unit(),
        }
    }

    // This allows reinterpreting the type decorator of an ExtraSponge<A, I> into
    // an ExtraSponge<A, J> where J is another pattern.
    // Safety: this should stay private to ensure it is only used in the below.
    fn repattern<J: List>(self) -> ExtraSponge<A, J> {
        // Mandated by the existence of a Drop implementation which we cannot move out of.
        // Safe since the only type that differs between source and destination is a Phantom
        let res =
            unsafe { std::mem::transmute_copy::<ExtraSponge<A, I>, ExtraSponge<A, J>>(&self) };
        // This is really important, as it lets us bypass the drop logic, which would blow up in a
        // non-empty Sponge.
        std::mem::forget(self);
        res
    }
}

impl<A: SpongeAPI, I: Normalize> ExtraSponge<A, I>
where
    Norm<I>: ToIOPattern, // Satisfied in all cases
{
    // Create a sponge with the IOPatten given as a type parameter.
    // Note that we do not require this pattern to be normalized - instead the constructor will return
    // an ExtraSPonge with a normalized pattern.
    pub fn start(
        domain_separator: Option<u32>,
        api: A,
        acc: &mut A::Acc,
    ) -> ExtraSponge<A, Norm<I>> {
        // Note: we not directly creating the state on I but on its normalization, satifying the requirement
        // in subsequent calls to absorb and squeeze - the pattern, by then, will be in normalized form and these calls
        // will maintain it as such.
        let mut extra_sponge: ExtraSponge<A, Norm<I>> = ExtraSponge::new(api);
        extra_sponge
            .api
            .start(Norm::<I>::to_iopattern(), domain_separator, acc);
        extra_sponge
    }
}

impl<A: SpongeAPI, I: Normalize> ExtraSponge<A, I> {
    /// This pass-through function is used to absorb elements in the sponge.
    /// It calls the underlying API's absorb function, and then returns a new ExtraSponge
    /// but a successful method dispatch to this implementation gaurantees the call is coherent with
    /// the IOPattern.
    pub fn absorb<U>(
        mut self,
        harray: Array<A::Value, U>,
        acc: &mut A::Acc,
    ) -> ExtraSponge<A, Use<I, Absorb<U>>>
    where
        U: ArraySize<A::Value>,
        I: Consume<Absorb<U>>,
    {
        self.api.absorb(U::to_u32(), &harray.as_slice(), acc);
        self.repattern()
    }
}

impl<A: SpongeAPI, I: Normalize> ExtraSponge<A, I> {
    /// This pass-through function is used to squeeze elements out of the sponge.
    /// It calls the underlying API's squeeze function, and then returns a new ExtraSponge
    /// but a successful method dispatch to this implementation gaurantees the call is coherent with
    /// the IOPattern.
    pub fn squeeze<U>(
        mut self,
        harray: &mut Array<A::Value, U>,
        acc: &mut A::Acc,
    ) -> ExtraSponge<A, Use<I, Squeeze<U>>>
    where
        U: ArraySize<A::Value>,
        I: Consume<Squeeze<U>>,
    {
        self.api.squeeze(U::to_u32(), harray.as_mut_slice(), acc);
        self.repattern()
    }
}

/// This implementation of drop is called automatically when the ExtraSponge drops out of scope.
/// It checks that the IOPattern is empty by then, and if it is not, it panics. Otherwise, it calls finalize.
impl<A: SpongeAPI, I: List> Drop for ExtraSponge<A, I> {
    fn drop(&mut self) {
        if I::is_empty() {
            self.api
                .finish()
                .expect("SpongeAPI invariant violated: finish failed on an empty IO pattern");
        } else {
            panic!("SpongeAPI invariant violated: forgot to empty IO pattern before dropping it");
        }
    }
}

#[cfg(test)]
mod unit_tests;

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

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
}
