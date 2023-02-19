use extra_safe::iopat;
use extra_safe::traits::{Absorb, Squeeze, Use};
use typenum::assert_type_eq;
use typenum::{U1, U3, U6};

fn main() {
    // Running out of Use allowance!
    assert_type_eq!(
        Use<iopat![Absorb<U3>, Squeeze<U1>, Absorb<U1>], Absorb<U6>>,
        iopat![Squeeze<U1>, Absorb<U1>]
    );
}
