mod traits;
use traits::{Normalize, Norm, List};
use hybrid_array::Array;

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
