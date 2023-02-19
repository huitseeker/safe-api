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
impl<Item: ToSpongeOp, T: ToIOPattern> ToIOPattern for Cons<Item, T> {
    fn to_iopattern() -> IOPattern {
        let mut v = T::to_iopattern().0;
        v.push(<Item as ToSpongeOp>::to_sponge_op());
        IOPattern(v)
    }
}

/// This is the SpongeAPI trait as you can find it in Neptune,
/// slightly modified so that the squeeze function takes an argument as a mutable slice
/// instead of returning a Vec.
pub trait SpongeAPI {
    type Acc;
    type Value;

    /// Optional `domain_separator` defaults to 0
    fn start(&mut self, p: IOPattern, domain_separator: Option<u32>, _: &mut Self::Acc);
    fn absorb(&mut self, length: u32, elements: &[Self::Value], acc: &mut Self::Acc);
    fn squeeze(
        &mut self,
        length: u32,
        elements: &mut [Self::Value],
        acc: &mut Self::Acc,
    ) -> Vec<Self::Value>;
    fn finish(&mut self, _: &mut Self::Acc) -> Result<(), Error>;
}

/// This is a slightly extended generic NewType wrapper around the original SpongeAPI.
/// It is decorated with the IOPattern I intended for this sponge instance.
#[derive(Debug)]
pub struct ExtraSponge<A: SpongeAPI, I: List> {
    api: A,
    acc: A::Acc,
    current_pattern: I,
}

impl<A: SpongeAPI, I: List> ExtraSponge<A, I> {
    pub fn new(api: A, acc: A::Acc) -> ExtraSponge<A, I> {
        ExtraSponge {
            api,
            acc,
            current_pattern: I::unit(),
        }
    }

    // This allows reinterpreting the type decorator of an ExtraSponge<A, I> into
    // an ExtraSponge<A, J> where J is another pattern.
    // Safety: this should stay private to ensure it is only used in the below.
    fn repattern<J: List>(self) -> ExtraSponge<A, J> {
        // Mandated by the existence of a Drop implementation which we cannot move out of
        // Safe since the only type that differs between source and destination is a Phantom
        let res =
            unsafe { std::mem::transmute_copy::<ExtraSponge<A, I>, ExtraSponge<A, J>>(&self) };
        // lets us bypass the drop logic,
        std::mem::forget(self);
        res
    }
}

impl<A: SpongeAPI, I: Normalize + ToIOPattern> ExtraSponge<A, I> {
    // Note this gives us the normalization of I!
    fn start(self, domain_separator: Option<u32>) -> ExtraSponge<A, Norm<I>> {
        // TODO add the constraint to link `Norm<I>` to argument p of self.api.start
        // this is in the style of e.g. typenum::Uint::to_u32()
        todo!()
    }
}

impl<A: SpongeAPI, I: Normalize> ExtraSponge<A, I> {
    fn absorb<U>(mut self, harray: Array<A::Value, U>) -> ExtraSponge<A, Use<I, Absorb<U>>>
    where
        U: ArraySize<A::Value>,
        I: Consume<Absorb<U>>,
    {
        self.api
            .absorb(U::to_u32(), &harray.as_slice(), &mut self.acc);
        self.repattern()
    }
}

impl<A: SpongeAPI, I: Normalize> ExtraSponge<A, I> {
    fn squeeze<U>(mut self, harray: &mut Array<A::Value, U>) -> ExtraSponge<A, Use<I, Squeeze<U>>>
    where
        U: ArraySize<A::Value>,
        I: Consume<Squeeze<U>>,
    {
        self.api
            .squeeze(U::to_u32(), harray.as_mut_slice(), &mut self.acc);
        self.repattern()
    }
}

impl<A: SpongeAPI, I: List> Drop for ExtraSponge<A, I> {
    fn drop(&mut self) {
        // TODO: blow up unless I == Nil
        todo!()
    }
}

#[cfg(test)]
mod unit_tests;
