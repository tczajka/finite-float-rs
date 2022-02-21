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

#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    cmp::Ordering,
    convert::TryFrom,
    fmt,
    hash::{Hash, Hasher},
    num::{FpCategory, ParseFloatError},
    ops::{Add, Neg, Sub},
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

macro_rules! impl_binary_op_references {
    ($op:ident for $t:ident, $f:ident) => {
        impl $op<&$t> for $t {
            type Output = $t;

            fn $f(self, rhs: &$t) -> $t {
                self.$f(*rhs)
            }
        }

        impl $op<$t> for &$t {
            type Output = $t;

            fn $f(self, rhs: $t) -> $t {
                (*self).$f(rhs)
            }
        }

        impl $op<&$t> for &$t {
            type Output = $t;

            fn $f(self, rhs: &$t) -> $t {
                (*self).$f(*rhs)
            }
        }
    };
}

macro_rules! impl_finite_float {
    ($t:ident, $base:ident) => {
        /// Finite floating point number.
        #[derive(Clone, Copy, PartialEq, PartialOrd)]
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
                    Some(Self::from_primitive(val))
                }
            }

            /// Return the value as a primitive type.
            pub fn get(self) -> $base {
                self.0
            }

            /// `val` can't be NaN
            ///
            /// `underflow_sign` is called when `val` is 0.0, in which case it indicates
            /// the comparison of the true value to zero.
            #[inline]
            fn from_primitive_with_underflow_sign<US>(val: $base, underflow_sign: US) -> Self
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

            #[inline]
            fn from_primitive(val: $base) -> Self {
                Self::from_primitive_with_underflow_sign(val, || Ordering::Equal)
            }
        }

        impl Eq for $t {}

        #[allow(clippy::derive_ord_xor_partial_ord)]
        impl Ord for $t {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        impl Default for $t {
            fn default() -> Self {
                Self::ZERO
            }
        }

        #[allow(clippy::derive_hash_xor_eq)]
        impl Hash for $t {
            fn hash<H>(&self, state: &mut H)
            where H: Hasher
            {
                self.get().to_bits().hash(state)
            }
        }

        impl From<$t> for $base {
            fn from(val: $t) -> Self {
                val.get()
            }
        }

        impl TryFrom<$base> for $t {
            type Error = NanError;

            fn try_from(val: $base) -> Result<$t, NanError> {
                $t::new(val).ok_or(NanError)
            }
        }

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
                    Ok(Self::from_primitive_with_underflow_sign(val, || parse_sign_of_tiny_float(s)))
                }
            }
        }

        impl Neg for $t {
            type Output = Self;

            fn neg(self) -> Self {
                Self::from_primitive(-self.get())
            }
        }

        impl Neg for &$t {
            type Output = $t;

            fn neg(self) -> $t {
                (*self).neg()
            }
        }

        impl Add for $t {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                // Result is 0 iff self == -rhs.
                Self::from_primitive(self.get() + rhs.get())
            }
        }

        impl_binary_op_references!(Add for $t, add);

        impl Sub for $t {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                // Result is 0 iff self == rhs.
                Self::from_primitive(self.get() - rhs.get())
            }
        }

        impl_binary_op_references!(Sub for $t, sub);
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

/// Error indicating an attempt to convert a NaN to a finite float.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NanError;

impl fmt::Display for NanError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conversion from NaN to finite float")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NanError {}
