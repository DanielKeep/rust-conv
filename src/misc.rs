/*!
This module defines some additional traits not *directly* tied to conversions.
*/

/**
This trait indicates that values of a type can be logically "saturated".

This is used by the `errors::UnwrapOrSaturate` extension trait.
*/
pub trait Saturated {
    /// Returns the type's saturated, maximum value.
    fn saturated_max() -> Self;

    /// Returns the type's saturated, minimum value.
    fn saturated_min() -> Self;
}

/**
This trait indicates that a type has an "invalid" sentinel value.

This is used by the `errors::UnwrapOrInvalid` extension trait.
*/
pub trait InvalidSentinel {
    /// Returns the type's "invalid" sentinel value.
    fn invalid_sentinel() -> Self;
}

/**
This trait indicates that a type has positive and negative "infinity" values.

This is used by the `errors::UnwrapOrInf` extension trait.
*/
pub trait SignedInfinity {
    /// Returns the type's positive infinity value.
    fn neg_infinity() -> Self;

    /// Returns the type's negative infinity value.
    fn pos_infinity() -> Self;
}
