/*!
This module defines the various error types that can be produced by a failed conversion.
*/

use misc::{Saturated, InvalidSentinel, SignedInfinity};

/// Indicates that it is not possible for the conversion to fail.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct NoError;

/// Indicates that the conversion failed due to an underflow.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Underflow;

impl From<NoError> for Underflow {
    fn from(_: NoError) -> Underflow {
        panic!("cannot convert NoError into Underflow");
    }
}

/// Indicates that the conversion failed due to an overflow.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Overflow;

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

impl<T> UnwrapOrInf for Result<T, RangeError>
where T: SignedInfinity {
    type Output = T;
    fn unwrap_or_inf(self) -> T {
        use self::RangeError::*;
        match self {
            Ok(v) => v,
            Err(Underflow) => T::neg_infinity(),
            Err(Overflow) => T::pos_infinity(),
        }
    }
}

impl<T> UnwrapOrInf for Result<T, Underflow>
where T: SignedInfinity {
    type Output = T;
    fn unwrap_or_inf(self) -> T {
        match self {
            Ok(v) => v,
            Err(Underflow) => T::neg_infinity(),
        }
    }
}

impl<T> UnwrapOrInf for Result<T, Overflow>
where T: SignedInfinity {
    type Output = T;
    fn unwrap_or_inf(self) -> T {
        match self {
            Ok(v) => v,
            Err(Overflow) => T::pos_infinity(),
        }
    }
}

impl<T> UnwrapOrInvalid for Result<T, RangeError>
where T: InvalidSentinel {
    type Output = T;
    fn unwrap_or_invalid(self) -> T {
        match self {
            Ok(v) => v,
            Err(_) => T::invalid_sentinel(),
        }
    }
}

impl<T> UnwrapOrInvalid for Result<T, Underflow>
where T: InvalidSentinel {
    type Output = T;
    fn unwrap_or_invalid(self) -> T {
        match self {
            Ok(v) => v,
            Err(_) => T::invalid_sentinel(),
        }
    }
}

impl<T> UnwrapOrInvalid for Result<T, Overflow>
where T: InvalidSentinel {
    type Output = T;
    fn unwrap_or_invalid(self) -> T {
        match self {
            Ok(v) => v,
            Err(_) => T::invalid_sentinel(),
        }
    }
}

impl<T> UnwrapOrSaturate for Result<T, RangeError>
where T: Saturated {
    type Output = T;
    fn unwrap_or_saturate(self) -> T {
        use self::RangeError::*;
        match self {
            Ok(v) => v,
            Err(Underflow) => T::saturated_min(),
            Err(Overflow) => T::saturated_max(),
        }
    }
}

impl<T> UnwrapOrSaturate for Result<T, Underflow>
where T: Saturated {
    type Output = T;
    fn unwrap_or_saturate(self) -> T {
        match self {
            Ok(v) => v,
            Err(Underflow) => T::saturated_min(),
        }
    }
}

impl<T> UnwrapOrSaturate for Result<T, Overflow>
where T: Saturated {
    type Output = T;
    fn unwrap_or_saturate(self) -> T {
        match self {
            Ok(v) => v,
            Err(Overflow) => T::saturated_max(),
        }
    }
}
