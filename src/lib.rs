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

use core::num::FpCategory;

macro_rules! impl_finite_float {
    ($t:ident, $base:ident) => {
        /// Finite floating point number.
        pub struct $t($base);

        impl $t {
            /// Number of significant digits in base 2.
            pub const MANTISSA_DIGITS: u32 = $base::MANTISSA_DIGITS;

            /// Difference between 1.0 and the next larger representable number.
            pub const EPSILON: $t = $t($base::EPSILON);

            /// Smallest (negative) value.
            pub const MIN: $t = $t($base::MIN);

            /// Largest value.
            pub const MAX: $t = $t($base::MAX);

            /// Smallest positive value.
            pub const MIN_POSITIVE: $t = $t($base::MIN_POSITIVE);

            /// Largest negative value.
            pub const MAX_NEGATIVE: $t = $t(-$base::MIN_POSITIVE);

            /// Create a new value.
            ///
            /// NaN results in None.
            pub fn new(val: $base) -> Option<$t> {
                match val.classify() {
                    FpCategory::Nan => None,
                    FpCategory::Infinite => {
                        if val > 0.0 {
                            Some($t::MAX)
                        } else {
                            Some($t::MIN)
                        }
                    }
                    FpCategory::Zero => Some($t(0.0)),
                    FpCategory::Subnormal => {
                        if val > 0.0 {
                            Some($t::MIN_POSITIVE)
                        } else {
                            Some($t::MAX_NEGATIVE)
                        }
                    }
                    FpCategory::Normal => Some($t(val)),
                }
            }
        }
    };
}

impl_finite_float!(Float32, f32);
impl_finite_float!(Float64, f64);
