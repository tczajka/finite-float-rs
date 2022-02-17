// Copyright (c) 2021 Tomek Czajka
//
// Licensed under either of
//
// * Apache License, Version 2.0
//   (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
// * MIT license
//   (LICENSE-MIT or https://opensource.org/licenses/MIT)
//
// at your option.
//
// Unless you explicitly state otherwise, any contribution intentionally submitted
// for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
// dual licensed as above, without any additional terms or conditions.

//! Finite floating-point arithmetic.

#![no_std]

use core::{
    cmp::Ordering,
    fmt,
    num::{FpCategory, ParseFloatError},
    str::FromStr,
};

macro_rules! impl_fmt {
    ($trait:ident for $t:ident) => {
        impl fmt::$trait for $t {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::$trait::fmt(&self.get(), f)
            }
        }
    };
}

macro_rules! impl_finite_float {
    ($t:ident, $base:ident) => {
        /// Finite floating point number.
        #[derive(Clone, Copy, PartialEq)]
        pub struct $t($base);

        impl $t {
            /// Number of significant digits in base 2.
            pub const MANTISSA_DIGITS: u32 = $base::MANTISSA_DIGITS;

            /// Zero.
            pub const ZERO: Self = Self(0.0);

            /// Difference between 1.0 and the next larger representable number.
            pub const EPSILON: Self = Self($base::EPSILON);

            /// Smallest (negative) value.
            pub const MIN: $t = Self($base::MIN);

            /// Largest value.
            pub const MAX: $t = Self($base::MAX);

            /// Smallest positive value.
            pub const MIN_POSITIVE: Self = Self($base::MIN_POSITIVE);

            /// Largest negative value.
            pub const MAX_NEGATIVE: Self = Self(-$base::MIN_POSITIVE);

            /// Create a new value.
            ///
            /// NaN results in None.
            #[inline]
            pub fn new(val: $base) -> Option<Self> {
                if val.is_nan() {
                    None
                } else {
                    Some(Self::from_primitive(val, || Ordering::Equal))
                }
            }

            /// Returns the value as a primitive IEEE-754 type.
            #[inline]
            pub fn get(self) -> $base {
                self.0
            }

            /// `val` can't be NaN
            ///
            /// `underflow_sign` is called when `val` is 0.0, in which case it indicates
            /// the comparison of the true value to zero.
            #[inline]
            fn from_primitive<US>(val: $base, underflow_sign: US) -> $t
            where
                US: FnOnce() -> Ordering,
            {
                match val.classify() {
                    FpCategory::Nan => unreachable!(),
                    FpCategory::Infinite => {
                        if val > 0.0 {
                            Self::MAX
                        } else {
                            Self::MIN
                        }
                    }
                    FpCategory::Zero => match underflow_sign() {
                        Ordering::Less => Self::MAX_NEGATIVE,
                        Ordering::Equal => Self(0.0),
                        Ordering::Greater => Self::MIN_POSITIVE,
                    },
                    FpCategory::Subnormal => {
                        if val > 0.0 {
                            Self::MIN_POSITIVE
                        } else {
                            Self::MAX_NEGATIVE
                        }
                    }
                    FpCategory::Normal => $t(val),
                }
            }
        }

        impl Eq for $t {}

        impl_fmt!(Debug for $t);
        impl_fmt!(Display for $t);
        impl_fmt!(LowerExp for $t);
        impl_fmt!(UpperExp for $t);

        impl FromStr for $t {
            type Err = ParseFloatError;

            fn from_str(s: &str) -> Result<Self, ParseFloatError> {
                let val = $base::from_str(s)?;
                if val.is_nan() {
                    Err($base::from_str("NaN value is invalid").unwrap_err())
                } else {
                    Ok(Self::from_primitive(val, || parse_sign_of_tiny_float(s)))
                }
            }
        }
    };
}

impl_finite_float!(Float32, f32);
impl_finite_float!(Float64, f64);

/// Returns the sign of a floating point number that has rounded to zero.
fn parse_sign_of_tiny_float(s: &str) -> Ordering {
    // Only look at the prefix consisting of: [+-] Digit* ( '.' Digit* )?
    let mut sign = Ordering::Greater;
    for byte in s.as_bytes() {
        match byte {
            b'-' => sign = Ordering::Less,
            b'+' | b'0' | b'.' => {}
            b'1'..=b'9' => return sign,
            _ => break,
        }
    }
    Ordering::Equal
}
