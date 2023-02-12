use core::ops::{Add, Sub};
use std::marker::PhantomData;
pub use typenum;
use typenum::{Bit, Diff, Sum, UInt, Unsigned, U0};

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

// Merge operator impl (optional)
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

// Head zero elimination
impl<L: Normalize> Normalize for Cons<Squeeze<U0>, L> {
    type Output = Norm<L>;
}

impl<L: Normalize> Normalize for Cons<Absorb<U0>, L> {
    type Output = Norm<L>;
}

// Base cases
impl<U: Unsigned, B: Bit> Normalize for Cons<Absorb<UInt<U, B>>, Nil> {
    type Output = Cons<Absorb<UInt<U, B>>, Nil>;
}
impl<U: Unsigned, B: Bit> Normalize for Cons<Squeeze<UInt<U, B>>, Nil> {
    type Output = Cons<Squeeze<UInt<U, B>>, Nil>;
}

// Non-head-zero recursive cases: concatenation
impl<U: Unsigned, B: Bit, M: Unsigned, T: List> Normalize
    for Cons<Absorb<UInt<U, B>>, Cons<Absorb<M>, T>>
where
    UInt<U, B>: Add<M>, // present for all values in practice
    Cons<Absorb<Sum<UInt<U, B>, M>>, T>: Normalize,
{
    type Output = Norm<Cons<Absorb<Sum<UInt<U, B>, M>>, T>>;
}

impl<U: Unsigned, B: Bit, M: Unsigned, T: List> Normalize
    for Cons<Squeeze<UInt<U, B>>, Cons<Squeeze<M>, T>>
where
    UInt<U, B>: Add<M>, // present for all reasonable values in practice
    Cons<Squeeze<Sum<UInt<U, B>, M>>, T>: Normalize,
{
    type Output = Norm<Cons<Squeeze<Sum<UInt<U, B>, M>>, T>>;
}

// Non-head-zero recursive cases: no concatenation
// This requires introspection into the UInt / UTerm type
impl<U: Unsigned, B: Bit, U2: Unsigned, B2: Bit, T: List> Normalize
    for Cons<Squeeze<UInt<U, B>>, Cons<Absorb<UInt<U2, B2>>, T>>
where
    Cons<Absorb<UInt<U2, B2>>, T>: Normalize,
{
    type Output = Cons<Squeeze<UInt<U, B>>, Norm<Cons<Absorb<UInt<U2, B2>>, T>>>;
}

impl<U: Unsigned, B: Bit, U2: Unsigned, B2: Bit, T: List> Normalize
    for Cons<Absorb<UInt<U, B>>, Cons<Squeeze<UInt<U2, B2>>, T>>
where
    Cons<Squeeze<UInt<U2, B2>>, T>: Normalize,
{
    type Output = Cons<Absorb<UInt<U, B>>, Norm<Cons<Squeeze<UInt<U2, B2>>, T>>>;
}

// and in case the head is zero
impl<U: Unsigned, B: Bit, T: List> Normalize for Cons<Squeeze<UInt<U, B>>, Cons<Absorb<U0>, T>>
where
    Cons<Squeeze<UInt<U, B>>, T>: Normalize,
{
    type Output = Norm<Cons<Squeeze<UInt<U, B>>, T>>;
}

impl<U: Unsigned, B: Bit, T: List> Normalize for Cons<Absorb<UInt<U, B>>, Cons<Squeeze<U0>, T>>
where
    Cons<Absorb<UInt<U, B>>, T>: Normalize,
{
    type Output = Norm<Cons<Absorb<UInt<U, B>>, T>>;
}

// Emptying an IOPattern
trait Consume<Op: IOWord> {
    type Output: List;
}

// Convenience trait for projection
#[allow(dead_code)]
type Use<T, U> = <T as Consume<U>>::Output;

// We unfold the type-level cases of the recurrence

// If the consumer is larger than the head pattern, we get to something
// impossible, because we assume this is only called on normalized lists

// If we get to U0, we end
impl<N, T: List> Consume<Absorb<U0>> for Cons<Absorb<N>, T>
where
    N: Unsigned,
{
    type Output = Self;
}

impl<N, T: List> Consume<Squeeze<U0>> for Cons<Squeeze<N>, T>
where
    N: Unsigned,
{
    type Output = Self;
}

// Otherwise, we simplify
impl<U, B, N, T> Consume<Absorb<UInt<U, B>>> for Cons<Absorb<N>, T>
where
    U: Unsigned,
    B: Bit,
    N: Unsigned,
    T: List,
    N: Sub<UInt<U, B>>, // present for N >= UInt<U, B>
    Cons<Absorb<Diff<N, UInt<U, B>>>, T>: Normalize,
{
    type Output = Norm<Cons<Absorb<Diff<N, UInt<U, B>>>, T>>;
}

impl<U, B, N, T> Consume<Squeeze<UInt<U, B>>> for Cons<Squeeze<N>, T>
where
    U: Unsigned,
    B: Bit,
    N: Unsigned,
    T: List,
    N: Sub<UInt<U, B>>, // present for N >= UInt<U, B>
    Cons<Squeeze<Diff<N, UInt<U, B>>>, T>: Normalize,
{
    type Output = Norm<Cons<Squeeze<Diff<N, UInt<U, B>>>, T>>;
}

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
        // Sum cases
        assert_type_eq!(
            Norm<Cons<Absorb<U2>, Cons<Absorb<U3>, Nil>>>,
            Cons<Absorb<U5>, Nil>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U2>, Cons<Squeeze<U3>, Nil>>>,
            Cons<Squeeze<U5>, Nil>
        );
        // recursion cases
        assert_type_eq!(
            Norm<Cons<Squeeze<U2>, Cons<Absorb<U3>, Nil>>>,
            Cons<Squeeze<U2>, Cons<Absorb<U3>, Nil>>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U2>, Cons<Squeeze<U3>, Cons<Absorb<U1>, Nil>>>>,
            Cons<Squeeze<U5>, Cons<Absorb<U1>, Nil>>
        );
        // zero elision at the head
        assert_type_eq!(
            Norm<Cons<Absorb<U0>, Cons<Absorb<U3>, Nil>>>,
            Cons<Absorb<U3>, Nil>
        );
        assert_type_eq!(
            Norm<Cons<Absorb<U0>, Cons<Squeeze<U3>, Nil>>>,
            Cons<Squeeze<U3>, Nil>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U0>, Cons<Squeeze<U3>, Nil>>>,
            Cons<Squeeze<U3>, Nil>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U0>, Cons<Absorb<U3>, Nil>>>,
            Cons<Absorb<U3>, Nil>
        );
        // zero elision in recursive cases
        assert_type_eq!(
            Norm<Cons<Absorb<U3>, Cons<Squeeze<U0>, Cons<Absorb<U1>, Nil>>>>,
            Cons<Absorb<U4>, Nil>
        );
        assert_type_eq!(
            Norm<Cons<Squeeze<U3>, Cons<Absorb<U0>, Cons<Squeeze<U1>, Nil>>>>,
            Cons<Squeeze<U4>, Nil>
        );
    }

    #[test]
    fn uses() {
        // Substraction
        assert_type_eq!(
            Use<Cons<Absorb<U5>, Nil>, Absorb<U2>>,
            Cons<Absorb<U3>, Nil>
        );
        assert_type_eq!(
            Use<Cons<Absorb<U5>, Cons<Squeeze<U2>, Nil>>, Absorb<U2>>,
            Cons<Absorb<U3>, Cons<Squeeze<U2>, Nil>>
        );

        assert_type_eq!(
            Use<Cons<Squeeze<U5>, Nil>, Squeeze<U2>>,
            Cons<Squeeze<U3>, Nil>
        );
        assert_type_eq!(
            Use<Cons<Squeeze<U5>, Cons<Absorb<U2>, Nil>>, Squeeze<U2>>,
            Cons<Squeeze<U3>, Cons<Absorb<U2>, Nil>>
        );

        // Zero-simplification
        assert_type_eq!(Use<Cons<Absorb<U5>, Nil>, Absorb<U5>>, Nil);
        assert_type_eq!(
            Use<Cons<Absorb<U5>, Nil>, Absorb<U0>>,
            Cons<Absorb<U5>, Nil>
        );
        assert_type_eq!(
            Use<Cons<Squeeze<U5>, Nil>, Squeeze<U0>>,
            Cons<Squeeze<U5>, Nil>
        );
        assert_type_eq!(
            Use<Cons<Absorb<U3>, Cons<Squeeze<U2>, Nil>>, Absorb<U3>>,
            Cons<Squeeze<U2>, Nil>
        );

        // This doesn't work: you have to call on (head-)normalized lists
        // initially
        /*
        assert_type_eq!(
            Use<Cons<Absorb<U5>, Cons<Absorb<U1>, Nil>>, Absorb<U6>>,
            Nil
        );
        */
    }
}
