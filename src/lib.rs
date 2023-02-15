mod traits;
use traits::{Normalize, Norm, List};
use hybrid_array::{Array, ArraySize};

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

// This is a generic NewType
pub struct ExtraSponge<A, I>{
    api: A,
    acc: A::Acc,
    current_pattern: I
}

impl<A: SpongeAPI, I: Normalize> {
    
    fn start(self, domain_separator: Option<u32>) -> Self<A, Norm<I>> = {
        // TODO add the constraint to link `Norm<I>` to argument p of self.api.start
        // this is in the style of e.g. typenum::Uint::to_u32()
        todo!()
    }
}

impl<A: SpongeAPI, U: ArraySize<A::Value>, I: Consume<Absorb<U>>> {

    fn absorb(self, harray: Array<A::Value, U>) -> Self<A, Use<Norm<I>, Absorb<U>>> {
        // TODO: just call A
        todo!()
    }

}

impl<A: SpongeAPI, U: ArraySize<A::Value>, I: Conseume<Squeeze<U>>> {
    fn squeeze(self, &mut harray: Array<A::Value, U>) -> Self<A, Use<Norm<I>, Squeeze<U>> {
        // TODO: just call A
        todo!()
    }
}

impl<A: SpongeAPI, I: Normalize> Drop for ExtraSponge<A, I> {
    fn drop(&mut self) {
        // TODO: blow up unless I == Nil
        todo!()
    }
}