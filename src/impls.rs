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
                #[inline]
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
                type Err = ::errors::RangeError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::RangeError::NegOverflow(src));
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::PosOverflow(src));
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
                type Err = ::errors::PosOverflow<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::PosOverflow(src));
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
                type Err = ::errors::RangeError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(min_of!($dst) as $src <= src) {
                        return Err(::errors::RangeError::NegOverflow(src));
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::PosOverflow(src));
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
                type Err = ::errors::NegOverflow<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::NegOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_dmin_to_dmax_no_nan {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        approx_dmin_to_dmax_no_nan! { ($($attrs)*), $src, $dst, $scheme, approx: |s| s }
    };

    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty, approx: |$src_name:ident| $conv:expr) => {
        as_item! {
            $($attrs)*
            impl ::ApproxFrom<$src, $scheme> for $dst {
                type Err = ::errors::FloatError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src.is_nan() {
                        return Err(::errors::FloatError::NotANumber(src));
                    }
                    let approx = { let $src_name = src; $conv };
                    if !(min_of!($dst) as $src <= approx) {
                        return Err(::errors::FloatError::NegOverflow(src));
                    }
                    if !(approx <= max_of!($dst) as $src) {
                        return Err(::errors::FloatError::PosOverflow(src));
                    }
                    Ok(approx as $dst)
                }
            }
        }
    };
}

macro_rules! num_conv {
    (@ $src:ty=> $(,)*) => {};

    (@ $src:ty=> #[32] $($tail:tt)*) => {
        num_conv! { @ $src=> (#[cfg(target_pointer_width="32")]) $($tail)* }
    };

    (@ $src:ty=> #[64] $($tail:tt)*) => {
        num_conv! { @ $src=> (#[cfg(target_pointer_width="64")]) $($tail)* }
    };

    (@ $src:ty=> e   $($tail:tt)*) => { num_conv! { @ $src=> () e   $($tail)* } };
    (@ $src:ty=> n+  $($tail:tt)*) => { num_conv! { @ $src=> () n+  $($tail)* } };
    (@ $src:ty=> n   $($tail:tt)*) => { num_conv! { @ $src=> () n   $($tail)* } };
    (@ $src:ty=> w+  $($tail:tt)*) => { num_conv! { @ $src=> () w+  $($tail)* } };
    (@ $src:ty=> w   $($tail:tt)*) => { num_conv! { @ $src=> () w   $($tail)* } };
    (@ $src:ty=> aW  $($tail:tt)*) => { num_conv! { @ $src=> () aW  $($tail)* } };
    (@ $src:ty=> nf  $($tail:tt)*) => { num_conv! { @ $src=> () nf  $($tail)* } };
    (@ $src:ty=> fan $($tail:tt)*) => { num_conv! { @ $src=> () fan $($tail)* } };

    // Exact conversion
    (@ $src:ty=> ($($attrs:tt)*) e $dst:ty, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::NoError;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing a signed type *into* an unsigned type where the destination type's maximum value is representable by the source type.
    (@ $src:ty=> ($($attrs:tt)*) n+ $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_z_to_dmax! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::RangeError<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::RangeError::NegOverflow(src));
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing an unsigned type *into* a type where the destination type's maximum value is representable by the source type.
    (@ $src:ty=> ($($attrs:tt)*) n- $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_to_dmax! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::PosOverflow<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing where the destination type's bounds are representable by the source type.
    (@ $src:ty=> ($($attrs:tt)*) n $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_dmin_to_dmax! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::RangeError<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(min_of!($dst) as $src <= src) {
                        return Err(::errors::RangeError::NegOverflow(src));
                    }
                    if !(src <= max_of!($dst) as $src) {
                        return Err(::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Widening a signed type *into* an unsigned type.
    (@ $src:ty=> ($($attrs:tt)*) w+ $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_z_up! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::NegOverflow<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(0 <= src) {
                        return Err(::errors::NegOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Widening.
    (@ $src:ty=> ($($attrs:tt)*) w $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, ::Wrapping }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::NoError;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing *into* a floating-point type where the conversion is only exact within a given range.
    (@ $src:ty=> ($($attrs:tt)*) nf [+- $bound:expr] $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::RangeError<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(-$bound <= src) {
                        return Err(::errors::RangeError::NegOverflow(src));
                    }
                    if !(src <= $bound) {
                        return Err(::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    (@ $src:ty=> ($($attrs:tt)*) nf [, $max:expr] $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, ::DefaultApprox }

            $($attrs)*
            impl ::ValueFrom<$src> for $dst {
                type Err = ::errors::PosOverflow<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if !(src <= $max) {
                        return Err(::errors::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Approximately narrowing a floating point value *into* a type where the source value is constrained by the given range of values.
    (@ $src:ty=> ($($attrs:tt)*) fan $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_dmin_to_dmax_no_nan! { ($($attrs)*), $src, $dst, ::DefaultApprox }
            approx_dmin_to_dmax_no_nan! { ($($attrs)*), $src, $dst, ::RoundToNearest,
                approx: |s| s.round() }
            approx_dmin_to_dmax_no_nan! { ($($attrs)*), $src, $dst, ::RoundToNegInf,
                approx: |s| s.floor() }
            approx_dmin_to_dmax_no_nan! { ($($attrs)*), $src, $dst, ::RoundToPosInf,
                approx: |s| s.ceil() }
            approx_dmin_to_dmax_no_nan! { ($($attrs)*), $src, $dst, ::RoundToZero,
                approx: |s| s.trunc() }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    ($src:ty=> $($tail:tt)*) => {
        num_conv! { @ $src=> $($tail)*, }
    };
}

mod lang_ints {
    num_conv! { i8=>  w i16, w i32, w i64, w+u8, w+u16, w+u32, w+u64, w isize, w+usize }
    num_conv! { i16=> n i8, w i32, w i64, n+u8, w+u16, w+u32, w+u64, w isize, w+usize }
    num_conv! { i32=> n i8, n i16, w i64, n+u8, n+u16, w+u32, w+u64 }
    num_conv! { i64=> n i8, n i16, n i32, n+u8, n+u16, n+u32, w+u64 }
    num_conv! { i32=> #[32] e isize, #[64] w isize, w+usize }
    num_conv! { i64=> #[32] n isize, #[64] e isize, n+usize }

    num_conv! { u8=> n-i8, w i16, w i32, w i64, w u16, w u32, w u64, w isize, w usize }
    num_conv! { u16=> n-i8, n-i16, w i32, w i64, n-u8, w u32, w u64, w isize, w usize }
    num_conv! { u32=> n-i8, n-i16, n-i32, w i64, n-u8, n-u16, w u64 }
    num_conv! { u64=> n-i8, n-i16, n-i32, n-i64, n-u8, n-u16, n-u32 }
    num_conv! { u32=> #[32] n-isize, #[64] w isize, #[32] e usize, #[64] w usize }
    num_conv! { u64=> n-isize, #[32] n-usize, #[64] e usize }

    num_conv! { isize=> n i8, n i16, #[32] e i32, #[32] w i64, #[64] n i32, #[64] e i64 }
    num_conv! { isize=> n+u8, n+u16, #[32] w+u32, #[32] w+u64, #[64] n+u32, #[64] w+u64 }
    num_conv! { isize=> w+usize }

    num_conv! { usize=> n-i8, n-i16, #[32] n-i32, #[32] w i64, #[64] n-i32, #[64] n-i64 }
    num_conv! { usize=> n-u8, n-u16, #[32] e u32, #[32] w u64, #[64] n-u32, #[64] e u64 }
    num_conv! { usize=> n-isize }
}

mod lang_floats {
    use {ApproxFrom, ApproxScheme};
    use ValueFrom;
    use errors::{NoError, RangeError};

    // f32 -> f64: strictly widening
    impl<Scheme> ApproxFrom<f32, Scheme> for f64
    where Scheme: ApproxScheme {
        type Err = NoError;
        #[inline]
        fn approx_from(src: f32) -> Result<f64, Self::Err> {
            Ok(src as f64)
        }
    }

    impl ValueFrom<f32> for f64 {
        type Err = NoError;
        #[inline]
        fn value_from(src: f32) -> Result<f64, Self::Err> {
            Ok(src as f64)
        }
    }

    // f64 -> f32: narrowing, approximate
    impl ApproxFrom<f64> for f32 {
        type Err = RangeError<f64>;
        #[inline]
        fn approx_from(src: f64) -> Result<f32, Self::Err> {
            if !src.is_finite() {
                return Ok(src as f32);
            }
            if !(::std::f32::MIN as f64 <= src) {
                return Err(RangeError::NegOverflow(src));
            }
            if !(src <= ::std::f32::MAX as f64) {
                return Err(RangeError::PosOverflow(src));
            }
            Ok(src as f32)
        }
    }
}

mod lang_int_to_float {
    num_conv! { i8=>  w f32, w f64 }
    num_conv! { i16=> w f32, w f64 }
    num_conv! { i32=> nf [+- 16_777_216] f32, w f64 }
    num_conv! { i64=> nf [+- 16_777_216] f32, nf [+- 9_007_199_254_740_992] f64 }

    num_conv! { u8=>  w f32, w f64 }
    num_conv! { u16=> w f32, w f64 }
    num_conv! { u32=> nf [, 16_777_216] f32, w f64 }
    num_conv! { u64=> nf [, 16_777_216] f32, nf [, 9_007_199_254_740_992] f64 }
}

mod lang_float_to_int {
    num_conv! { f32=> fan i8, fan i16, fan i32, fan i64 }
    num_conv! { f32=> fan u8, fan u16, fan u32, fan u64 }
    num_conv! { f32=> fan isize, fan usize }

    num_conv! { f64=> fan i8, fan i16, fan i32, fan i64 }
    num_conv! { f64=> fan u8, fan u16, fan u32, fan u64 }
    num_conv! { f64=> fan isize, fan usize }
}

mod lang_char_to_int {
    use TryFrom;
    use ValueFrom;
    use errors::{NoError, PosOverflow};

    impl TryFrom<char> for u32 {
        type Err = NoError;
        #[inline]
        fn try_from(src: char) -> Result<u32, Self::Err> {
            Ok(src as u32)
        }
    }

    impl TryFrom<char> for usize {
        type Err = NoError;
        #[inline]
        fn try_from(src: char) -> Result<usize, Self::Err> {
            Ok(src as usize)
        }
    }

    impl TryFrom<char> for isize {
        type Err = NoError;
        #[inline]
        fn try_from(src: char) -> Result<isize, Self::Err> {
            Ok(src as isize)
        }
    }

    macro_rules! conv_char_to_int {
        ($($ts:ty),* $(,)*) => {
            $(
                impl TryFrom<char> for $ts {
                    type Err = PosOverflow<char>;
                    #[inline]
                    fn try_from(src: char) -> Result<$ts, Self::Err> {
                        <$ts as ValueFrom<_>>::value_from(src as u32)
                            .map_err(|_| PosOverflow(src))
                    }
                }
            )*
        };
    }

    macro_rules! conv_char_to_int_wide {
        ($($ts:ty),* $(,)*) => {
            $(
                impl TryFrom<char> for $ts {
                    type Err = NoError;
                    #[inline]
                    fn try_from(src: char) -> Result<$ts, Self::Err> {
                        <$ts as ValueFrom<_>>::value_from(src as u32)
                    }
                }
            )*
        };
    }

    conv_char_to_int! { i8, i16, i32, u8, u16 }
    conv_char_to_int_wide! { i64, u64 }
}

mod lang_int_to_char {
    use TryFrom;
    use ValueFrom;
    use errors::{NoError, Unrepresentable, UnwrapOk};

    impl TryFrom<u8> for char {
        type Err = NoError;
        #[inline]
        fn try_from(src: u8) -> Result<char, Self::Err> {
            Ok(src as char)
        }
    }
    impl TryFrom<u16> for char {
        type Err = Unrepresentable<u16>;
        #[inline]
        fn try_from(src: u16) -> Result<char, Self::Err> {
            TryFrom::try_from(
                <u32 as ValueFrom<_>>::value_from(src).unwrap_ok()
            ).map_err(|_| Unrepresentable(src))
        }
    }

    impl TryFrom<u32> for char {
        type Err = Unrepresentable<u32>;
        #[inline]
        fn try_from(src: u32) -> Result<char, Self::Err> {
            ::std::char::from_u32(src).ok_or_else(|| Unrepresentable(src))
        }
    }

    macro_rules! conv_int_to_char {
        ($($ts:ty),* $(,)*) => {
            $(
                impl TryFrom<$ts> for char {
                    type Err = Unrepresentable<$ts>;
                    #[inline]
                    fn try_from(src: $ts) -> Result<char, Self::Err> {
                        <u32 as ValueFrom<_>>::value_from(src)
                            .map_err(|_| Unrepresentable(src))
                            .and_then(|usv| TryFrom::try_from(usv)
                                .map_err(|_| Unrepresentable(src)))
                    }
                }
            )*
        };
    }

    conv_int_to_char! { i8, i16, i32, i64, isize, u64, usize }
}
