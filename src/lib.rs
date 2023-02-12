pub use typenum;

use core::ops::Add;
use std::marker::PhantomData;
use typenum::{Sum, Unsigned};

/// Our two alternatives for the IOPattern, i.e. these are IOWords
/// Note the phantom type avoids allocating actual data
pub struct Absorb<N>(PhantomData<N>);
pub struct Squeeze<N>(PhantomData<N>);

/// Our trait for common treatment of both patterns
// TODO: make a sealed trait
trait IOWord {}

impl<N: Unsigned> IOWord for Absorb<N> {}
impl<N: Unsigned> IOWord for Squeeze<N> {}

/// Our merge operator for same-type words
// TODO: make a sealed trait
trait Merge<Other: IOWord>: IOWord {
    type Output;
}

// Convenience alias for projection
#[allow(dead_code)]
type Mer<T, U> = <T as Merge<U>>::Output;

// Merge operator impl
impl<N, M> Merge<Absorb<M>> for Absorb<N>
where
    N: Unsigned,
    M: Unsigned,
    N: Add<M>, // present for all reasonable values in practice
{
    type Output = Absorb<Sum<N, M>>;
}

impl<N, M> Merge<Squeeze<M>> for Squeeze<N>
where
    N: Unsigned,
    M: Unsigned,
    N: Add<M>, // present for all reasonable values in practice
{
    type Output = Squeeze<Sum<N, M>>;
}

// type-level HList, specializable to IOWord
// using  a sealed trait
trait List {}
impl<Item, Next: List> List for Cons<Item, Next> {}
impl List for Nil {}

struct Cons<Item, Next: List> {
    _phantom: PhantomData<(Item, Next)>,
}
struct Nil;

// an IOPattern is a List of IOWords .. (TODO: does this need elaboration?)

// Normalizing an IOPattern with Merge
trait Normalize: List {
    type Output: List;
}

// Convenience trait for projection
type Norm<T> = <T as Normalize>::Output;

// We unfold the type-level cases of the recurrence
impl Normalize for Nil {
    type Output = Nil;
}

impl<Item> Normalize for Cons<Item, Nil>
where
    Item: IOWord,
{
    type Output = Self;
}

impl<N: Unsigned, M: Unsigned, T: List> Normalize for Cons<Absorb<N>, Cons<Absorb<M>, T>>
where
    N: Add<M>, // present for all reasonable values in practice
    Cons<Absorb<Sum<N, M>>, T>: Normalize,
{
    type Output = Norm<Cons<Absorb<Sum<N, M>>, T>>;
}

impl<N: Unsigned, M: Unsigned, T: List> Normalize for Cons<Squeeze<N>, Cons<Squeeze<M>, T>>
where
    N: Add<M>, // present for all reasonable values in practice
    Cons<Squeeze<Sum<N, M>>, T>: Normalize,
{
    type Output = Norm<Cons<Squeeze<Sum<N, M>>, T>>;
}

impl<N: Unsigned, M: Unsigned, T: List> Normalize for Cons<Squeeze<N>, Cons<Absorb<M>, T>>
where
    Cons<Absorb<M>, T>: Normalize,
{
    type Output = Cons<Squeeze<N>, Norm<Cons<Absorb<M>, T>>>;
}

impl<N: Unsigned, M: Unsigned, T: List> Normalize for Cons<Absorb<N>, Cons<Squeeze<M>, T>>
where
    Cons<Squeeze<M>, T>: Normalize,
{
    type Output = Cons<Absorb<N>, Norm<Cons<Squeeze<M>, T>>>;
}

// Emptying an IOPattern
trait Consume<Op: IOWord> {
    type Output: List;
}

// Convenience trait for projection
type Use<T, U> = <T as Consume<U>>::Output;

// We unfold the type-level cases of the recurrence

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::assert_type_eq;
    use typenum::{U1, U2, U3, U4, U5};

    #[test]
    fn merges() {
        assert_type_eq!(Mer<Absorb<U2>, Absorb<U3>>, Absorb<U5>);
        assert_type_eq!(Mer<Squeeze<U1>, Squeeze<U4>>, Squeeze<U5>);
    }

    #[test]
    fn normalizes() {
        assert_type_eq!(
            Norm<Cons<Absorb<U2>, Cons<Absorb<U3>, Nil>>>,
            Cons<Absorb<U5>, Nil>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U2>, Cons<Squeeze<U3>, Nil>>>,
            Cons<Squeeze<U5>, Nil>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U2>, Cons<Absorb<U3>, Nil>>>,
            Cons<Squeeze<U2>, Cons<Absorb<U3>, Nil>>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U2>, Cons<Squeeze<U3>, Cons<Absorb<U1>, Nil>>>>,
            Cons<Squeeze<U5>, Cons<Absorb<U1>, Nil>>
        );
    }
}
