pub mod traits;

use hybrid_array::{Array, ArrayOps, ArraySize};
use traits::{Absorb, Consume, IOWord, List, Norm, Normalize, Squeeze, Use};

#[derive(Debug)]
pub enum Error {
    ParameterUsageMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpongeOp {
    Absorb(u32),
    Squeeze(u32),
}

#[derive(Clone, Debug)]
pub struct IOPattern(pub Vec<SpongeOp>);

// TODO : convert SpongeOp -> IOWord using macros

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

// This is a slightly extended generic NewType
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

impl<A: SpongeAPI, I: Normalize> ExtraSponge<A, I> {
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
        let values = self
            .api
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
