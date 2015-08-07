/*!
This module defines the various error types that can be produced by a failed conversion.
*/

use std::any::Any;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use misc::{Saturated, InvalidSentinel, SignedInfinity};

/**
A general error enumeration that subsumes all other conversion errors.

This exists primarily as a "catch-all" for reliably unifying various different kinds of conversion errors.
*/
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum GeneralError {
    /// Input underflowed the target type.
    Underflow,

    /// Input overflowed the target type.
    Overflow,

    /// Input was not representable in the target type.
    Unrepresentable,
}

impl Display for GeneralError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.description())
    }
}

impl Error for GeneralError {
    fn description(&self) -> &str {
        use self::GeneralError::*;
        match *self {
            Underflow => "conversion resulted in underflow",
            Overflow => "conversion resulted in overflow",
            Unrepresentable => "could not convert unrepresentable value",
        }
    }
}

impl From<NoError> for GeneralError {
    fn from(_: NoError) -> GeneralError {
        panic!("cannot convert NoError into GeneralError");
    }
}

impl<T> From<Unrepresentable<T>> for GeneralError {
    fn from(_: Unrepresentable<T>) -> GeneralError {
        GeneralError::Unrepresentable
    }
}

impl From<Underflow> for GeneralError {
    fn from(_: Underflow) -> GeneralError {
        GeneralError::Underflow
    }
}

impl From<Overflow> for GeneralError {
    fn from(_: Overflow) -> GeneralError {
        GeneralError::Overflow
    }
}

impl From<RangeError> for GeneralError {
    fn from(e: RangeError) -> GeneralError {
        use self::RangeError as R;
        use self::GeneralError as G;
        match e {
            R::Underflow => G::Underflow,
            R::Overflow => G::Overflow,
        }
    }
}

impl From<FloatError> for GeneralError {
    fn from(e: FloatError) -> GeneralError {
        use self::FloatError as F;
        use self::GeneralError as G;
        match e {
            F::Underflow => G::Underflow,
            F::Overflow => G::Overflow,
            F::NotANumber => G::Unrepresentable,
        }
    }
}

/// Indicates that it is not possible for the conversion to fail.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum NoError {}

impl Display for NoError {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unreachable!()
    }
}

impl Error for NoError {
    fn description(&self) -> &str {
        unreachable!()
    }
}

/// Indicates that the conversion failed because the value was not representable.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Unrepresentable<T>(pub T);

impl<T> Display for Unrepresentable<T>
where T: Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "could not convert unrepresentable value: {}", self.0)
    }
}

impl<T> Error for Unrepresentable<T>
where T: Debug + Display + Any {
    fn description(&self) -> &str {
        "could not convert unrepresentable value"
    }
}

/// Indicates that the conversion failed due to an underflow.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Underflow;

impl Display for Underflow {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.description())
    }
}

impl Error for Underflow {
    fn description(&self) -> &str {
        "conversion resulted in underflow"
    }
}

impl From<NoError> for Underflow {
    fn from(_: NoError) -> Underflow {
        panic!("cannot convert NoError into Underflow");
    }
}

/// Indicates that the conversion failed due to an overflow.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Overflow;

impl Display for Overflow {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.description())
    }
}

impl Error for Overflow {
    fn description(&self) -> &str {
        "conversion resulted in overflow"
    }
}

impl From<NoError> for Overflow {
    fn from(_: NoError) -> Overflow {
        panic!("cannot convert NoError into Overflow");
    }
}

/**
Indicates that a conversion from a floating point type failed.
*/
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum FloatError {
    /// Input underflowed the target type.
    Underflow,

    /// Input overflowed the target type.
    Overflow,

    /// Input was not-a-number, which the target type could not represent.
    NotANumber,
}

impl Display for FloatError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.description())
    }
}

impl Error for FloatError {
    fn description(&self) -> &str {
        use self::FloatError::*;
        match *self {
            Underflow => "conversion resulted in underflow",
            Overflow => "conversion resulted in overflow",
            NotANumber => "conversion target does not support not-a-number",
        }
    }
}

impl From<NoError> for FloatError {
    fn from(_: NoError) -> FloatError {
        panic!("cannot convert NoError into FloatError");
    }
}

impl From<Underflow> for FloatError {
    fn from(_: Underflow) -> FloatError {
        FloatError::Underflow
    }
}

impl From<Overflow> for FloatError {
    fn from(_: Overflow) -> FloatError {
        FloatError::Overflow
    }
}

impl From<RangeError> for FloatError {
    fn from(e: RangeError) -> FloatError {
        use self::RangeError as R;
        use self::FloatError as F;
        match e {
            R::Underflow => F::Underflow,
            R::Overflow => F::Overflow,
        }
    }
}

/**
Indicates that a conversion failed due to a range error.
*/
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum RangeError {
    /// Input underflowed the target type.
    Underflow,

    /// Input overflowed the target type.
    Overflow,
}

impl Display for RangeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.description())
    }
}

impl Error for RangeError {
    fn description(&self) -> &str {
        use self::RangeError::*;
        match *self {
            Underflow => "conversion resulted in underflow",
            Overflow => "conversion resulted in overflow",
        }
    }
}

impl From<NoError> for RangeError {
    fn from(_: NoError) -> RangeError {
        panic!("cannot convert NoError into RangeError");
    }
}

impl From<Underflow> for RangeError {
    fn from(_: Underflow) -> RangeError {
        RangeError::Underflow
    }
}

impl From<Overflow> for RangeError {
    fn from(_: Overflow) -> RangeError {
        RangeError::Overflow
    }
}

/**
Safely unwrap a `Result` that cannot contain an error.
*/
pub trait UnwrapOk<T> {
    /**
    Unwraps a `Result` without possibility of failing.

    Technically, this is not necessary; it's provided simply to make user code a little clearer.
    */
    fn unwrap_ok(self) -> T;
}

impl<T> UnwrapOk<T> for Result<T, NoError> {
    fn unwrap_ok(self) -> T {
        match self {
            Ok(v) => v,
            Err(..) => loop {},
        }
    }
}

/**
Unwrap a conversion by saturating to infinity.
*/
pub trait UnwrapOrInf {
    /// The result of unwrapping.
    type Output;

    /**
    Either unwraps the successfully converted value, or saturates to infinity in the "direction" of overflow/underflow.
    */
    fn unwrap_or_inf(self) -> Self::Output;
}

/**
Unwrap a conversion by replacing a failure with an invalid sentinel value.
*/
pub trait UnwrapOrInvalid {
    /// The result of unwrapping.
    type Output;

    /**
    Either unwraps the successfully converted value, or returns the output type's invalid sentinel value.
    */
    fn unwrap_or_invalid(self) -> Self::Output;
}

/**
Unwrap a conversion by saturating.
*/
pub trait UnwrapOrSaturate {
    /// The result of unwrapping.
    type Output;

    /**
    Either unwraps the successfully converted value, or saturates in the "direction" of overflow/underflow.
    */
    fn unwrap_or_saturate(self) -> Self::Output;
}

impl<T, E> UnwrapOrInf for Result<T, E>
where T: SignedInfinity, E: Into<RangeError> {
    type Output = T;
    fn unwrap_or_inf(self) -> T {
        use self::RangeError::*;
        match self.map_err(Into::into) {
            Ok(v) => v,
            Err(Underflow) => T::neg_infinity(),
            Err(Overflow) => T::pos_infinity(),
        }
    }
}

impl<T, E> UnwrapOrInvalid for Result<T, E>
where T: InvalidSentinel {
    type Output = T;
    fn unwrap_or_invalid(self) -> T {
        match self {
            Ok(v) => v,
            Err(..) => T::invalid_sentinel(),
        }
    }
}

impl<T, E> UnwrapOrSaturate for Result<T, E>
where T: Saturated, E: Into<RangeError> {
    type Output = T;
    fn unwrap_or_saturate(self) -> T {
        use self::RangeError::*;
        match self.map_err(Into::into) {
            Ok(v) => v,
            Err(Underflow) => T::saturated_min(),
            Err(Overflow) => T::saturated_max(),
        }
    }
}
