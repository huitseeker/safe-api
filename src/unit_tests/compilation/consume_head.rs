use extra_safe::iopat;
use extra_safe::traits::{Absorb, Nil, Squeeze, Use};
use typenum::assert_type_eq;
use typenum::{U0, U1, U5, U6};

fn main() {
    // This does not work because the `Use` trait assumes you are passing a pattern in head-normal form (i.e. you've triggered Norm on the list)
    assert_type_eq!(
        Use<iopat![Absorb<U5>, Squeeze<U0>, Absorb<U1>], Absorb<U6>>,
        Nil
    );
}
