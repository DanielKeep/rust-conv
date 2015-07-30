macro_rules! max_of {
    ($name:ident) => { ::std::$name::MAX };
}

macro_rules! min_of {
    ($name:ident) => { ::std::$name::MIN };
}

macro_rules! approx_blind {
    (($($attrs:tt)*), $src:ty, $dst:ty, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl ::ApproxFrom<$src, $scheme> for $dst {
                type Err = ::errors::NoError;
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_z_to_dmax {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl ::ApproxFrom<$src, $scheme> for $dst {
                type Err = ::errors::RangeError;
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::RangeError::Underflow);
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_to_dmax {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl ::ApproxFrom<$src, $scheme> for $dst {
                type Err = ::errors::Overflow;
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_dmin_to_dmax {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl ::ApproxFrom<$src, $scheme> for $dst {
                type Err = ::errors::RangeError;
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(min_of!($dst) as $src <= src) {
                        return Err(::errors::RangeError::Underflow);
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
    }
}

macro_rules! approx_z_up {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl ::ApproxFrom<$src, $scheme> for $dst {
                type Err = ::errors::Underflow;
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::Underflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_dmin_to_dmax_no_nan {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl ::ApproxFrom<$src, $scheme> for $dst {
                type Err = ::errors::FloatError;
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src.is_nan() {
                        return Err(::errors::FloatError::NotANumber);
                    }
                    if !(min_of!($dst) as $src <= src) {
                        return Err(::errors::FloatError::Underflow);
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::FloatError::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! num_conv {
    (@ $src:ty; $(,)*) => {};

    (@ $src:ty; #[32] $($tail:tt)*) => {
        num_conv! { @ $src; (#[cfg(target_pointer_width="32")]) $($tail)* }
    };

    (@ $src:ty; #[64] $($tail:tt)*) => {
        num_conv! { @ $src; (#[cfg(target_pointer_width="64")]) $($tail)* }
    };

    (@ $src:ty; e   $($tail:tt)*) => { num_conv! { @ $src; () e   $($tail)* } };
    (@ $src:ty; n+  $($tail:tt)*) => { num_conv! { @ $src; () n+  $($tail)* } };
    (@ $src:ty; n   $($tail:tt)*) => { num_conv! { @ $src; () n   $($tail)* } };
    (@ $src:ty; w+  $($tail:tt)*) => { num_conv! { @ $src; () w+  $($tail)* } };
    (@ $src:ty; w   $($tail:tt)*) => { num_conv! { @ $src; () w   $($tail)* } };
    (@ $src:ty; aW  $($tail:tt)*) => { num_conv! { @ $src; () aW  $($tail)* } };
    (@ $src:ty; nf  $($tail:tt)*) => { num_conv! { @ $src; () nf  $($tail)* } };
    (@ $src:ty; fan $($tail:tt)*) => { num_conv! { @ $src; () fan $($tail)* } };

    // Exact conversion
    (@ $src:ty; ($($attrs:tt)*) e $dst:ty, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::NoError;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    // Narrowing a signed type *into* an unsigned type where the destination type's maximum value is representable by the source type.
    (@ $src:ty; ($($attrs:tt)*) n+ $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_z_to_dmax! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::RangeError;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::RangeError::Underflow);
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    // Narrowing an unsigned type *into* a type where the destination type's maximum value is representable by the source type.
    (@ $src:ty; ($($attrs:tt)*) n- $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_to_dmax! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::Overflow;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    // Narrowing where the destination type's bounds are representable by the source type.
    (@ $src:ty; ($($attrs:tt)*) n $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_dmin_to_dmax! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::RangeError;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(min_of!($dst) as $src <= src) {
                        return Err(::errors::RangeError::Underflow);
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    // Widening a signed type *into* an unsigned type.
    (@ $src:ty; ($($attrs:tt)*) w+ $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_z_up! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::Underflow;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::Underflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    // Widening.
    (@ $src:ty; ($($attrs:tt)*) w $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::NoError;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    // Narrowing *into* a floating-point type where the conversion is only exact within a given range.
    (@ $src:ty; ($($attrs:tt)*) nf [+- $bound:expr] $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::RangeError;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(-$bound <= src) {
                        return Err(::errors::RangeError::Underflow);
                    }
                    if !(src <= $bound) {
                        return Err(::errors::RangeError::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    (@ $src:ty; ($($attrs:tt)*) nf [, $max:expr] $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::Overflow;
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(src <= $max) {
                        return Err(::errors::Overflow);
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src; $($tail)* }
    };

    // Approximately narrowing a floating point value *into* a type where the source value is constrained by the given range of values.
    (@ $src:ty; ($($attrs:tt)*) fan $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_dmin_to_dmax_no_nan! { ($($attrs)*), $src, $dst, ::DefaultApprox }
        }
        num_conv! { @ $src; $($tail)* }
    };

    ($src:ty; $($tail:tt)*) => {
        num_conv! { @ $src; $($tail)*, }
    };
}

mod lang_ints {
    num_conv! { i8;  w i16, w i32, w i64, w+u8, w+u16, w+u32, w+u64, w isize, w+usize }
    num_conv! { i16; n i8, w i32, w i64, n+u8, w+u16, w+u32, w+u64, w isize, w+usize }
    num_conv! { i32; n i8, n i16, w i64, n+u8, n+u16, w+u32, w+u64 }
    num_conv! { i64; n i8, n i16, n i32, n+u8, n+u16, n+u32, w+u64 }
    num_conv! { i32; #[32] e isize, #[64] w isize, w+usize }
    num_conv! { i64; #[32] n isize, #[64] e isize, n+usize }

    num_conv! { u8; n-i8, w i16, w i32, w i64, w u16, w u32, w u64, w isize, w usize }
    num_conv! { u16; n-i8, n-i16, w i32, w i64, n-u8, w u32, w u64, w isize, w usize }
    num_conv! { u32; n-i8, n-i16, n-i32, w i64, n-u8, n-u16, w u64 }
    num_conv! { u64; n-i8, n-i16, n-i32, n-i64, n-u8, n-u16, n-u32 }
    num_conv! { u32; #[32] n-isize, #[64] w isize, #[32] e usize, #[64] w usize }
    num_conv! { u64; n-isize, #[32] n-usize, #[64] e usize }

    num_conv! { isize; n i8, n i16, #[32] e i32, #[32] w i64, #[64] n i32, #[64] e i64 }
    num_conv! { isize; n+u8, n+u16, #[32] w+u32, #[32] w+u64, #[64] n+u32, #[64] w+u64 }
    num_conv! { isize; w+usize }

    num_conv! { usize; n-i8, n-i16, #[32] n-i32, #[32] w i64, #[64] n-i32, #[64] n-i64 }
    num_conv! { usize; n-u8, n-u16, #[32] e u32, #[32] w u64, #[64] n-u32, #[64] e u64 }
    num_conv! { usize; n-isize }
}

mod lang_floats {
    use {ApproxFrom, ApproxScheme};
    use ValueFrom;
    use errors::{NoError, RangeError};

    // f32 -> f64: strictly widening
    impl<Scheme> ApproxFrom<f32, Scheme> for f64
    where Scheme: ApproxScheme {
        type Err = NoError;
        fn approx_from(src: f32) -> Result<f64, Self::Err> {
            Ok(src as f64)
        }
    }

    impl ValueFrom<f32> for f64 {
        type Err = NoError;
        fn value_from(src: f32) -> Result<f64, Self::Err> {
            Ok(src as f64)
        }
    }

    // f64 -> f32: narrowing, approximate
    impl ApproxFrom<f64> for f32 {
        type Err = RangeError;
        fn approx_from(src: f64) -> Result<f32, Self::Err> {
            if !src.is_finite() {
                return Ok(src as f32);
            }
            if !(::std::f32::MIN as f64 <= src) {
                return Err(RangeError::Underflow);
            }
            if !(src <= ::std::f32::MAX as f64) {
                return Err(RangeError::Overflow);
            }
            Ok(src as f32)
        }
    }
}

mod lang_int_to_float {
    num_conv! { i8;  w f32, w f64 }
    num_conv! { i16; w f32, w f64 }
    num_conv! { i32; nf [+- 16_777_216] f32, w f64 }
    num_conv! { i64; nf [+- 16_777_216] f32, nf [+- 9_007_199_254_740_992] f64 }

    num_conv! { u8;  w f32, w f64 }
    num_conv! { u16; w f32, w f64 }
    num_conv! { u32; nf [, 16_777_216] f32, w f64 }
    num_conv! { u64; nf [, 16_777_216] f32, nf [, 9_007_199_254_740_992] f64 }
}

mod lang_float_to_int {
    num_conv! { f32; fan i8, fan i16, fan i32, fan i64 }
    num_conv! { f32; fan u8, fan u16, fan u32, fan u64 }
    num_conv! { f32; fan isize, fan usize }

    num_conv! { f64; fan i8, fan i16, fan i32, fan i64 }
    num_conv! { f64; fan u8, fan u16, fan u32, fan u64 }
    num_conv! { f64; fan isize, fan usize }
}
