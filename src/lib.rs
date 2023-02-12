pub use typenum;

use std::marker::PhantomData;
use typenum::{Bit, B0, B1};
use typenum::{Same, Sum, Unsigned};

/// Our two alternatives for the IOPattern, i.e. these are IOWords
pub struct Absorb<N: Unsigned>(PhantomData(N));
pub struct Squeeze<N: Unsigned>(PhantomData(N));

/// Our trait for common treatment of both patterns
// TODO: make a sealed trait
trait IOWord {
    type Direction: Bit;
    type Length: Unsigned;
};

// Convenience aliases for projections
type Len<T> = <T as IOWord>::Length;
type Dir<T> = <T as IOWord>::Direction;

impl<N: Unsigned> IOWord for Absorb<N> {
    type Direction = B1; // arbitrary, but opposite <Squeeze<N> as IOWord>::Direction
    type Length = N;
};

impl<N: Unsigned> IOWord for Squeeze<N> {
    type Direction = B0;
    type Length = N;
};

/// Our merge operator for same-type words
// TODO: make a sealed trait
trait MergeLength<Other: IOWord>: IOWord {
    type Output: Unsigned;
}

// Convenience alias for projection
type MergedLen<T, U> = <T as Merge<U>>::Output;

// Merge operator impl
impl<T: IOWord> Merge<Other> for T 
where 
    Other: IOWord,
    Dir<T> : Same<Dir<Other>>
{
    type Output = Sum<Len<T>, Len<Other>>;
}

// type Dual, used for alternation (TODO: Figure out if maybe we'd use B0/B1 instead)
trait ComputeDual {
    type Output;
}
type Dual<S> = <S as ComputeDual>::Output;


// type-level HList
trait List {}
impl<Item, Next: List> List for Cons<Item, Next> {}
impl List for Nil {}

struct Cons<Item, Next: List> {
    _phantom: PhantomData<(Item, Next)>
}
struct Nil;

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
