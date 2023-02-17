use safe_api::iopat;
use safe_api::traits::{Absorb, Norm};
use typenum::assert_type_eq;
use typenum::{U2, U3, U4};

fn main() {
    assert_type_eq!(Norm<iopat![Absorb<U2>, Absorb<U3>]>, iopat![Absorb<U4>]);
}
