pub use typenum;

use std::marker::PhantomData;
use typenum::{Bit, B0, B1};
use typenum::{Same, Sum, Unsigned};

pub struct IOWord<B: Bit, N: Unsigned>(PhantomData<(B, N)>);

pub type Absorb<N> = IOWord<B0, N>;
pub type Squeeze<N> = IOWord<B1, N>;

pub type Merge<BA, BB, NA, NB> = IOWord<<BA as Same<BB>>::Output, Sum<NA, NB>>;

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::assert_type_eq;
    use typenum::{U1, U2, U3, U4, U5};

    #[test]
    fn sums() {
        assert_type_eq!(Merge<B1, B1, U2, U3>, IOWord<B1, U5>);
        assert_type_eq!(Merge<B0, B0, U1, U3>, IOWord<B0, U4>);
    }
}
