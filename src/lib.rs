mod traits;
use hybrid_array::{Array, ArraySize};
use traits::{Absorb, Consume, Norm, Normalize, Squeeze, Use};

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
    fn squeeze(&mut self, length: u32, acc: &mut Self::Acc) -> Vec<Self::Value>;
    fn finish(&mut self, _: &mut Self::Acc) -> Result<(), Error>;
}

// This is a slightly extended generic NewType
pub struct ExtraSponge<A: SpongeAPI, I> {
    api: A,
    acc: A::Acc,
    current_pattern: I,
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
    fn absorb<U>(self, harray: Array<A::Value, U>) -> ExtraSponge<A, Use<I, Absorb<U>>>
    where
        U: ArraySize<A::Value>,
        I: Consume<Absorb<U>>,
    {
        // TODO: just call A::absorb
        todo!()
    }
}

impl<A: SpongeAPI, I: Normalize> ExtraSponge<A, I> {
    fn squeeze<U>(self, harray: &mut Array<A::Value, U>) -> ExtraSponge<A, Use<I, Squeeze<U>>>
    where
        U: ArraySize<A::Value>,
        I: Consume<Squeeze<U>>,
    {
        // TODO: just call A::squeeze
        todo!()
    }
}

impl<A: SpongeAPI, I> Drop for ExtraSponge<A, I> {
    fn drop(&mut self) {
        // TODO: blow up unless I == Nil
        todo!()
    }
}
