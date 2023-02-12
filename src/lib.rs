pub use typenum;

use std::marker::PhantomData;
use typenum::{Bit, B0, B1};
use typenum::{Same, Sum, Unsigned};

/// Our two alternatives for the IOPattern, i.e. these are IOWords
/// Note the PhantomData avoids allocating actual data
pub struct Absorb<N: Unsigned>(PhantomData(N));
pub struct Squeeze<N: Unsigned>(PhantomData(N));

/// Our trait for common treatment of both patterns
// TODO: make a sealed trait
trait IOWord {};

impl<N: Unsigned> IOWord for Absorb<N> {};
impl<N: Unsigned> IOWord for Squeeze<N> {};

/// Our merge operator for same-type words
// TODO: make a sealed trait
trait Merge<Other: IOWord>: IOWord {
    type Output: IOWord;
}

// Convenience alias for projection
type Mer<T, U> = <T as Merge<U>>::Output;

// Merge operator impl
impl<Absorb<N>> Merge<Absorb<M>> for T 
where 
    N: Unsigned,
    M: Unsigned,
{
    type Output = Absorb<Sum<N, M>>;
}

impl<Squeeze<N>> Merge<Squeeze<M>> for T 
where 
    N: Unsigned,
    M: Unsigned,
{
    type Output = Squeeze<Sum<N, M>>;
}

// type-level HList
trait List {}
impl<Item, Next: List> List for Cons<Item, Next> {}
impl List for Nil {}

struct Cons<Item, Next: List> {
    _phantom: PhantomData<(Item, Next)>
}
struct Nil;

// an IOPattern is a List of IOWords .. (TODO: does this need elaboration?)

// Normalizing an IOPattern with Merge
type Normalize {
    type Output: List;
}

// TODO: recursion !?

// Emptying an IOPattern
type Consume<Op: IOWord> {
    type Output: List;
}

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
