use finite_float::{Float32, Float64, NanError};

use std::{cmp::Ordering, collections::HashMap, convert::TryFrom};

#[test]
fn test_constants() {
    assert_eq!(Float32::MANTISSA_DIGITS, 24);
    assert_eq!(Float64::MANTISSA_DIGITS, 53);

    assert_eq!(Float32::ZERO.get(), 0.0);
    assert!(Float32::ZERO.get().is_sign_positive());
    assert_eq!(Float64::ZERO.get(), 0.0);
    assert!(Float64::ZERO.get().is_sign_positive());

    assert_eq!(Float32::EPSILON.get(), f32::EPSILON);
    assert_eq!(Float64::EPSILON.get(), f64::EPSILON);

    assert_eq!(Float32::MIN.get(), f32::MIN);
    assert_eq!(Float64::MIN.get(), f64::MIN);

    assert_eq!(Float32::MAX.get(), f32::MAX);
    assert_eq!(Float64::MAX.get(), f64::MAX);

    assert_eq!(Float32::MIN_POSITIVE.get(), f32::MIN_POSITIVE);
    assert_eq!(Float64::MIN_POSITIVE.get(), f64::MIN_POSITIVE);

    assert_eq!(Float32::MAX_NEGATIVE.get(), -f32::MIN_POSITIVE);
    assert_eq!(Float64::MAX_NEGATIVE.get(), -f64::MIN_POSITIVE);
}

#[test]
#[allow(clippy::approx_constant)]
fn test_new() {
    // NaN
    assert!(Float32::new(f32::NAN).is_none());
    assert!(Float64::new(f64::NAN).is_none());

    // normal
    assert_eq!(Float32::new(3.14).unwrap().get(), 3.14);
    assert_eq!(Float64::new(3.14).unwrap().get(), 3.14);

    // infinity
    assert_eq!(Float32::new(f32::INFINITY).unwrap(), Float32::MAX);
    assert_eq!(Float32::new(-f32::INFINITY).unwrap(), Float32::MIN);
    assert_eq!(Float64::new(f64::INFINITY).unwrap(), Float64::MAX);
    assert_eq!(Float64::new(-f64::INFINITY).unwrap(), Float64::MIN);

    // zero
    assert_eq!(Float32::new(0.0).unwrap(), Float32::ZERO);
    assert_eq!(Float32::new(-0.0).unwrap(), Float32::ZERO);
    assert!(Float32::new(-0.0).unwrap().get().is_sign_positive());
    assert_eq!(Float64::new(0.0).unwrap(), Float64::ZERO);
    assert_eq!(Float64::new(-0.0).unwrap(), Float64::ZERO);
    assert!(Float64::new(-0.0).unwrap().get().is_sign_positive());

    // subnormal
    assert_eq!(
        Float32::new(f32::MIN_POSITIVE / 2.0).unwrap(),
        Float32::MIN_POSITIVE
    );
    assert_eq!(
        Float32::new(-f32::MIN_POSITIVE / 2.0).unwrap(),
        Float32::MAX_NEGATIVE
    );
    assert_eq!(
        Float64::new(f64::MIN_POSITIVE / 2.0).unwrap(),
        Float64::MIN_POSITIVE
    );
    assert_eq!(
        Float64::new(-f64::MIN_POSITIVE / 2.0).unwrap(),
        Float64::MAX_NEGATIVE
    );
}

#[test]
fn test_default() {
    assert_eq!(Float32::default(), Float32::ZERO);
    assert!(Float32::default().get().is_sign_positive());
    assert_eq!(Float64::default(), Float64::ZERO);
    assert!(Float64::default().get().is_sign_positive());
}

#[test]
fn test_ord() {
    assert_eq!(
        Float32::new(2.0).unwrap().cmp(&Float32::new(3.0).unwrap()),
        Ordering::Less
    );
    assert_eq!(
        Float32::new(0.0).unwrap().cmp(&Float32::new(-0.0).unwrap()),
        Ordering::Equal
    );
    assert_eq!(
        Float64::new(2.0).unwrap().cmp(&Float64::new(3.0).unwrap()),
        Ordering::Less
    );
    assert_eq!(
        Float64::new(0.0).unwrap().cmp(&Float64::new(-0.0).unwrap()),
        Ordering::Equal
    );
}

#[test]
fn test_hash() {
    let mut m: HashMap<Float32, u32> = HashMap::new();
    m.insert(Float32::ZERO, 0);
    m.insert(Float32::new(3.0).unwrap(), 1);
    assert_eq!(m.get(&Float32::new(-0.0).unwrap()), Some(&0));
    assert_eq!(m.get(&Float32::new(3.0).unwrap()), Some(&1));
    assert_eq!(m.get(&Float32::new(4.0).unwrap()), None);

    let mut m: HashMap<Float64, u32> = HashMap::new();
    m.insert(Float64::ZERO, 0);
    m.insert(Float64::new(3.0).unwrap(), 1);
    assert_eq!(m.get(&Float64::new(-0.0).unwrap()), Some(&0));
    assert_eq!(m.get(&Float64::new(3.0).unwrap()), Some(&1));
    assert_eq!(m.get(&Float64::new(4.0).unwrap()), None);
}

#[test]
#[allow(clippy::approx_constant)]
fn test_conversions() {
    assert_eq!(f32::from(Float32::new(3.14).unwrap()), 3.14);
    assert_eq!(f64::from(Float64::new(3.14).unwrap()), 3.14);

    assert_eq!(
        Float32::try_from(3.14).unwrap(),
        Float32::new(3.14).unwrap()
    );

    assert_eq!(
        Float64::try_from(3.14).unwrap(),
        Float64::new(3.14).unwrap()
    );

    assert_eq!(Float32::try_from(f32::NAN).unwrap_err(), NanError);
    assert_eq!(Float64::try_from(f64::NAN).unwrap_err(), NanError);
}

#[test]
#[allow(clippy::approx_constant)]
fn test_formatting() {
    assert_eq!(format!("{:?}", Float32::ZERO), "0.0");
    assert_eq!(format!("{:?}", Float64::ZERO), "0.0");
    assert_eq!(format!("{:?}", Float32::new(3.14).unwrap()), "3.14");
    assert_eq!(format!("{:?}", Float64::new(3.14).unwrap()), "3.14");
    assert_eq!(format!("{}", Float32::new(3.14).unwrap()), "3.14");
    assert_eq!(format!("{}", Float64::new(3.14).unwrap()), "3.14");
    assert_eq!(format!("{:e}", Float32::new(3.14).unwrap()), "3.14e0");
    assert_eq!(format!("{:e}", Float64::new(3.14).unwrap()), "3.14e0");
    assert_eq!(format!("{:E}", Float32::new(3.14).unwrap()), "3.14E0");
    assert_eq!(format!("{:E}", Float64::new(3.14).unwrap()), "3.14E0");
}

#[test]
#[allow(clippy::approx_constant)]
fn test_parse() {
    // Invalid format.
    assert!("foo".parse::<Float32>().is_err());
    assert!("foo".parse::<Float64>().is_err());

    // NaN doesn't parse.
    assert!("NaN".parse::<Float32>().is_err());
    assert!("NaN".parse::<Float64>().is_err());

    // Normal.
    assert_eq!("3.14".parse::<Float32>().unwrap().get(), 3.14);
    assert_eq!("3.14".parse::<Float64>().unwrap().get(), 3.14);

    // Overflow.
    assert_eq!("1e1000".parse::<Float32>().unwrap(), Float32::MAX);
    assert_eq!("1e1000".parse::<Float64>().unwrap(), Float64::MAX);
    assert_eq!("-1e1000".parse::<Float32>().unwrap(), Float32::MIN);
    assert_eq!("-1e1000".parse::<Float64>().unwrap(), Float64::MIN);

    // Infinity.
    assert_eq!("inf".parse::<Float32>().unwrap(), Float32::MAX);
    assert_eq!("inf".parse::<Float64>().unwrap(), Float64::MAX);
    assert_eq!("-inf".parse::<Float32>().unwrap(), Float32::MIN);
    assert_eq!("-inf".parse::<Float64>().unwrap(), Float64::MIN);

    // Zero.
    assert_eq!("0.0".parse::<Float32>().unwrap(), Float32::ZERO);
    assert_eq!("0.0".parse::<Float64>().unwrap(), Float64::ZERO);
    assert_eq!("-0.0".parse::<Float32>().unwrap(), Float32::ZERO);
    assert_eq!("-0.0".parse::<Float64>().unwrap(), Float64::ZERO);
    assert!("-0.0".parse::<Float32>().unwrap().get().is_sign_positive());
    assert!("-0.0".parse::<Float64>().unwrap().get().is_sign_positive());
    assert_eq!("0.0e10000".parse::<Float32>().unwrap(), Float32::ZERO);
    assert_eq!("0.0e10000".parse::<Float64>().unwrap(), Float64::ZERO);
    assert_eq!("-0.0e-10000".parse::<Float32>().unwrap(), Float32::ZERO);
    assert_eq!("-0.0e-10000".parse::<Float64>().unwrap(), Float64::ZERO);

    // Subnormal.
    assert_eq!("1e-40".parse::<Float32>().unwrap(), Float32::MIN_POSITIVE);
    assert_eq!("1e-310".parse::<Float64>().unwrap(), Float64::MIN_POSITIVE);
    assert_eq!("-1e-40".parse::<Float32>().unwrap(), Float32::MAX_NEGATIVE);
    assert_eq!("-1e-310".parse::<Float64>().unwrap(), Float64::MAX_NEGATIVE);

    // Below subnormal.
    assert_eq!(
        "1.00e-10000".parse::<Float32>().unwrap(),
        Float32::MIN_POSITIVE
    );
    assert_eq!(
        "1.00e-10000".parse::<Float64>().unwrap(),
        Float64::MIN_POSITIVE
    );
    assert_eq!(
        "-0.0000090e-10000".parse::<Float32>().unwrap(),
        Float32::MAX_NEGATIVE
    );
    assert_eq!(
        "-0.0000090e-10000".parse::<Float64>().unwrap(),
        Float64::MAX_NEGATIVE
    );
}

#[test]
fn test_neg() {
    assert_eq!(-Float32::new(3.0).unwrap(), Float32::new(-3.0).unwrap());
    assert_eq!(-Float32::ZERO, Float32::ZERO);
    assert!((-Float32::ZERO).get().is_sign_positive());
    assert_eq!(-&Float32::new(3.0).unwrap(), Float32::new(-3.0).unwrap());

    assert_eq!(-Float64::new(3.0).unwrap(), Float64::new(-3.0).unwrap());
    assert_eq!(-Float64::ZERO, Float64::ZERO);
    assert!((-Float64::ZERO).get().is_sign_positive());
    assert_eq!(-&Float64::new(3.0).unwrap(), Float64::new(-3.0).unwrap());
}

#[test]
fn test_add() {
    // Normal.
    assert_eq!(
        Float32::new(3.0).unwrap() + Float32::new(4.0).unwrap(),
        Float32::new(7.0).unwrap()
    );
    assert_eq!(
        Float64::new(3.0).unwrap() + Float64::new(4.0).unwrap(),
        Float64::new(7.0).unwrap()
    );

    // Zero.
    assert_eq!(
        Float32::new(-3.0).unwrap() + Float32::new(3.0).unwrap(),
        Float32::ZERO
    );
    assert_eq!(
        Float64::new(-3.0).unwrap() + Float64::new(3.0).unwrap(),
        Float64::ZERO
    );

    assert!((Float32::new(-3.0).unwrap() + Float32::new(3.0).unwrap())
        .get()
        .is_sign_positive());
    assert!((Float64::new(-3.0).unwrap() + Float64::new(3.0).unwrap())
        .get()
        .is_sign_positive());

    assert_eq!(Float32::MAX + Float32::MIN, Float32::ZERO);
    assert_eq!(Float64::MAX + Float64::MIN, Float64::ZERO);

    // Overflow.
    assert_eq!(Float32::MAX + Float32::MAX, Float32::MAX);
    assert_eq!(Float64::MAX + Float64::MAX, Float64::MAX);
    assert_eq!(Float32::MIN + Float32::MIN, Float32::MIN);
    assert_eq!(Float64::MIN + Float64::MIN, Float64::MIN);

    // Underflow.
    assert_eq!(
        Float32::new(f32::MIN_POSITIVE * (1.0 + f32::EPSILON)).unwrap() + Float32::MAX_NEGATIVE,
        Float32::MIN_POSITIVE
    );
    assert_eq!(
        Float64::new(f64::MIN_POSITIVE * (1.0 + f64::EPSILON)).unwrap() + Float64::MAX_NEGATIVE,
        Float64::MIN_POSITIVE
    );

    assert_eq!(
        Float32::new(-f32::MIN_POSITIVE * (1.0 + f32::EPSILON)).unwrap() + Float32::MIN_POSITIVE,
        Float32::MAX_NEGATIVE
    );
    assert_eq!(
        Float64::new(-f64::MIN_POSITIVE * (1.0 + f64::EPSILON)).unwrap() + Float64::MIN_POSITIVE,
        Float64::MAX_NEGATIVE
    );
}

#[test]
fn test_sub() {
    // Normal.
    assert_eq!(
        Float32::new(3.0).unwrap() - Float32::new(4.0).unwrap(),
        Float32::new(-1.0).unwrap()
    );
    assert_eq!(
        Float64::new(3.0).unwrap() - Float64::new(4.0).unwrap(),
        Float64::new(-1.0).unwrap()
    );

    // Zero.
    assert_eq!(
        Float32::new(-3.0).unwrap() - Float32::new(-3.0).unwrap(),
        Float32::ZERO
    );
    assert_eq!(
        Float64::new(-3.0).unwrap() - Float64::new(-3.0).unwrap(),
        Float64::ZERO
    );

    assert!((Float32::new(-3.0).unwrap() - Float32::new(-3.0).unwrap())
        .get()
        .is_sign_positive());
    assert!((Float64::new(-3.0).unwrap() - Float64::new(-3.0).unwrap())
        .get()
        .is_sign_positive());

    assert_eq!(Float32::MAX - Float32::MAX, Float32::ZERO);
    assert_eq!(Float64::MAX - Float64::MAX, Float64::ZERO);

    // Overflow.
    assert_eq!(Float32::MAX - Float32::MIN, Float32::MAX);
    assert_eq!(Float64::MAX - Float64::MIN, Float64::MAX);
    assert_eq!(Float32::MIN - Float32::MAX, Float32::MIN);
    assert_eq!(Float64::MIN - Float64::MAX, Float64::MIN);

    // Underflow.
    assert_eq!(
        Float32::new(f32::MIN_POSITIVE * (1.0 + f32::EPSILON)).unwrap() - Float32::MIN_POSITIVE,
        Float32::MIN_POSITIVE
    );
    assert_eq!(
        Float64::new(f64::MIN_POSITIVE * (1.0 + f64::EPSILON)).unwrap() - Float64::MIN_POSITIVE,
        Float64::MIN_POSITIVE
    );

    assert_eq!(
        Float32::new(-f32::MIN_POSITIVE * (1.0 + f32::EPSILON)).unwrap() - Float32::MAX_NEGATIVE,
        Float32::MAX_NEGATIVE
    );
    assert_eq!(
        Float64::new(-f64::MIN_POSITIVE * (1.0 + f64::EPSILON)).unwrap() - Float64::MAX_NEGATIVE,
        Float64::MAX_NEGATIVE
    );
}
