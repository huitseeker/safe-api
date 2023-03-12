//! This module contains the traits for the IOPattern type, along with the implementation of type-level operations checking on their correct usage.
//! The IOPattern type is a type-level HList of IOWords, which are either Absorb or Squeeze.
//! The main operations are Normalize, which merges successive words of the same type, and Consume, which takes a word and an IOPattern, and checks whether
//! it is legal to use the operation of this word on the tip of the IOPattern.
//! This is explained in more detail in [the spec document][1].
//!
//! [1]: https://hackmd.io/bHgsH6mMStCVibM_wYvb2w#SAFE-Sponge-API-for-Field-Elements-%E2%80%93-A-Toolbox-for-ZK-Hash-Applications

use core::ops::{Add, Sub};
use std::marker::PhantomData;
pub use typenum;
use typenum::{Bit, Diff, Sum, UInt, Unsigned, U0};

// Our two alternatives for the IOPattern, i.e. these are IOWords
// Note the phantom type avoids allocating actual data.

/// The type-level Absorb operation
#[derive(Debug)]
pub struct Absorb<N>(PhantomData<N>);
/// The type-level Squeeze operation
#[derive(Debug)]
pub struct Squeeze<N>(PhantomData<N>);

/// Our trait for common treatment of both patterns
pub trait IOWord: private::Sealed {}

impl<N: Unsigned> IOWord for Absorb<N> {}
impl<N: Unsigned> IOWord for Squeeze<N> {}

/// Type-level HList, specialized to IOWord
/// using  a sealed trait
/// See e.g. `<https://hackage.haskell.org/package/heterolist>` (or frunk) for
/// what a HList is.
pub trait List: private::Sealed {
    /// This is an inhabitant of the List type corresponding to the
    /// Self type
    fn unit() -> Self;

    /// THis returns whether the list is empty
    fn is_empty() -> bool;
}

impl<Item: IOWord, Next: List> List for Cons<Item, Next> {
    fn unit() -> Self {
        Cons {
            _phantom: PhantomData,
        }
    }
    fn is_empty() -> bool {
        false
    }
}

impl List for Nil {
    fn unit() -> Self {
        Nil
    }
    fn is_empty() -> bool {
        true
    }
}

/// The concrete type constructor for our HList trait
#[derive(Debug)]
pub struct Cons<Item, Next: List> {
    _phantom: PhantomData<(Item, Next)>,
}

/// The concrete type for our empty HList representant
#[derive(Debug)]
pub struct Nil;

/// Convenience helper for creating an instance of List
#[macro_export]
macro_rules! iopat {
    () => { $crate::traits::Nil };
    (...$rest:ty) => { $rest };
    ($a:ty) => { $crate::iopat![$a,] };
    ($a:ty, $($tok:tt)*) => {
        $crate::traits::Cons<$a, $crate::iopat![$($tok)*]>
    };
}

// an IOPattern is a List of IOWords .. (TODO: does this need elaboration?)

/// Normalizing an IOPattern with merge operations applied recursively
pub trait Normalize: List {
    /// The output of the normalization
    type Output: List;
}

/// Convenience trait for projection of Normalize
pub type Norm<T> = <T as Normalize>::Output;

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

/// Emptying an IOPattern using an IOWord. This assumes that it is working
/// with a list in head-normal form (i.e. the first element cannot be merged
/// with the immediately following list). All lists that have been normalized
/// are in head-normal form.
pub trait Consume<Op: IOWord> {
    /// The output of the consumption
    type Output: List;
}

/// Convenience trait for projection of Consume
#[allow(dead_code)]
pub type Use<T, U> = <T as Consume<U>>::Output;

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

// Seal the traits so that the above defines admissible implementations of sealed traits
mod private {
    pub trait Sealed {}

    impl<N> Sealed for super::Absorb<N> {}
    impl<N> Sealed for super::Squeeze<N> {}

    impl Sealed for super::Nil {}
    impl<H, T: super::List> Sealed for super::Cons<H, T> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::assert_type_eq;
    use typenum::{U1, U2, U3, U4, U5, U6};

    #[test]
    fn normalizes() {
        // Sum cases
        assert_type_eq!(Norm<iopat![Absorb<U2>, Absorb<U3>]>, iopat![Absorb<U5>]);
        assert_type_eq!(Norm<iopat![Squeeze<U2>, Squeeze<U3>]>, iopat![Squeeze<U5>]);
        // recursion cases
        assert_type_eq!(
            Norm<iopat![Squeeze<U2>, Absorb<U3>]>,
            iopat![Squeeze<U2>, Absorb<U3>]
        );
        assert_type_eq!(
            Norm<iopat![Squeeze<U2>, Squeeze<U3>, Absorb<U1>]>,
            iopat![Squeeze<U5>, Absorb<U1>]
        );
        // zero elision at the head
        assert_type_eq!(Norm<iopat![Absorb<U0>, Absorb<U3>]>, iopat![Absorb<U3>]);
        assert_type_eq!(Norm<iopat![Absorb<U0>, Squeeze<U3>]>, iopat![Squeeze<U3>]);
        assert_type_eq!(Norm<iopat![Squeeze<U0>, Squeeze<U3>]>, iopat![Squeeze<U3>]);
        assert_type_eq!(Norm<iopat![Squeeze<U0>, Absorb<U3>]>, iopat![Absorb<U3>]);
        // zero elision in recursive cases
        assert_type_eq!(
            Norm<iopat![Absorb<U3>, Squeeze<U0>, Absorb<U1>]>,
            iopat![Absorb<U4>]
        );
        assert_type_eq!(
            Norm<iopat![Squeeze<U3>, Absorb<U0>, Squeeze<U1>]>,
            iopat![Squeeze<U4>]
        );
    }

    #[test]
    fn uses() {
        // Substraction
        assert_type_eq!(
            Use<iopat![Absorb<U5>], Absorb<U2>>,
            iopat![Absorb<U3>]
        );
        assert_type_eq!(
            Use<iopat![Absorb<U5>, Squeeze<U2>], Absorb<U2>>,
            iopat![Absorb<U3>, Squeeze<U2>]
        );

        assert_type_eq!(
            Use<iopat![Squeeze<U5>], Squeeze<U2>>,
            iopat![Squeeze<U3>]
        );
        assert_type_eq!(
            Use<iopat![Squeeze<U5>, Absorb<U2>], Squeeze<U2>>,
            iopat![Squeeze<U3>, Absorb<U2>]
        );

        // Zero-simplification
        assert_type_eq!(Use<iopat![Absorb<U5>], Absorb<U5>>, Nil);
        assert_type_eq!(
            Use<iopat![Absorb<U5>], Absorb<U0>>,
            iopat![Absorb<U5>]
        );
        assert_type_eq!(
            Use<iopat![Squeeze<U5>], Squeeze<U0>>,
            iopat![Squeeze<U5>]
        );
        assert_type_eq!(
            Use<iopat![Absorb<U3>, Squeeze<U2>], Absorb<U3>>,
            iopat![Squeeze<U2>]
        );

        // This doesn't work: you have to call on (head-)normalized lists
        // initially
        /*
        assert_type_eq!(
            Use<iopat![Absorb<U5>, Absorb<U1>], Absorb<U6>>,
            Nil
        );
        */

        // This, however, works
        assert_type_eq!(
            Use<Use<iopat![Squeeze<U3>, Absorb<U5>, Absorb<U1>], Squeeze<U3>>, Absorb<U6>>,
            Nil
        );
    }
}
