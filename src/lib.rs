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

macro_rules! impl_finite_float {
    ($t:ident, $base:ident) => {
        /// Finite floating point number.
        pub struct $t($base);

        impl $t {
            /// Radix of the internal representation.
            pub const RADIX: u32 = $base::RADIX;

            /// Number of significant digits in base 2.
            pub const MANTISSA_DIGITS: u32 = $base::MANTISSA_DIGITS;

            /// Approximate number of significant digits in base 10.
            pub const DIGITS: u32 = $base::DIGITS;

            /// Difference between 1.0 and the next larger representable number.
            pub const EPSILON: $t = $t($base::EPSILON);

            /// Smallest finite (negative) value.
            pub const MIN: $t = $t($base::MIN);

            /// Smallest positive value.
            pub const MIN_POSITIVE: $t = $t($base::MIN_POSITIVE);

            /// Minimum possible power of 2 exponent.
            /// TODO: Why is $base::MIN_EXP off by 1?
            pub const MIN_EXP: i32 = $base::MIN_EXP - 1;

            /// Maximum possible power of 2 exponent.
            pub const MAX_EXP: i32 = $base::MAX_EXP;

            /// Minimum possible power of 10 exponent.
            pub const MIN_10_EXP: i32 = $base::MIN_10_EXP;

            /// Maximum possible power of 10 exponent.
            pub const MAX_10_EXP: i32 = $base::MAX_10_EXP;
        }
    };
}

impl_finite_float!(Float32, f32);
impl_finite_float!(Float64, f64);
