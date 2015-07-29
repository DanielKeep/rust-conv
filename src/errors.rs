/*!
This module defines the various error types that can be produced by a failed conversion.
*/

use misc::{Saturated, InvalidSentinel, SignedInfinity};

/// Indicates that it is not possible for the conversion to fail.
pub struct NoError;

/// Indicates that the conversion failed due to an underflow.
pub struct Underflow;

/// Indicates that the conversion failed due to an overflow.
pub struct Overflow;

/**
Indicates that a conversion from a floating point type failed.
*/
pub enum FloatError {
    /// Input underflowed the target type.
    Underflow,

    /// Input overflowed the target type.
    Overflow,

    /// Input was not-a-number, which the target type could not represent.
    NotANumber,
}

/**
Indicates that a conversion failed due to a range error.
*/
pub enum RangeError {
    /// Input underflowed the target type.
    Underflow,

    /// Input overflowed the target type.
    Overflow,
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
