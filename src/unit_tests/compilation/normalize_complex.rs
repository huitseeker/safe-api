use extra_safe::iopat;
use extra_safe::traits::{Absorb, Norm, Squeeze};
use typenum::assert_type_eq;
use typenum::{U0, U2, U3, U6};

fn main() {
    assert_type_eq!(
        Norm<iopat![Absorb<U2>, Squeeze<U0>, Absorb<U3>, Absorb<U3>]>,
        iopat![Absorb<U2>, Absorb<U6>]
    );
}
