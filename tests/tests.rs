use finite_float::{Float32, Float64};

#[test]
fn test_constants() {
    assert_eq!(Float32::MANTISSA_DIGITS, 24);
    assert_eq!(Float64::MANTISSA_DIGITS, 53);

    assert_eq!(f32::from(Float32::ZERO), 0.0);
    assert!(f32::from(Float32::ZERO).is_sign_positive());
    assert_eq!(f64::from(Float64::ZERO), 0.0);
    assert!(f64::from(Float64::ZERO).is_sign_positive());

    assert_eq!(f32::from(Float32::EPSILON), f32::EPSILON);
    assert_eq!(f64::from(Float64::EPSILON), f64::EPSILON);

    assert_eq!(f32::from(Float32::MIN), f32::MIN);
    assert_eq!(f64::from(Float64::MIN), f64::MIN);

    assert_eq!(f32::from(Float32::MAX), f32::MAX);
    assert_eq!(f64::from(Float64::MAX), f64::MAX);

    assert_eq!(f32::from(Float32::MIN_POSITIVE), f32::MIN_POSITIVE);
    assert_eq!(f64::from(Float64::MIN_POSITIVE), f64::MIN_POSITIVE);

    assert_eq!(f32::from(Float32::MAX_NEGATIVE), -f32::MIN_POSITIVE);
    assert_eq!(f64::from(Float64::MAX_NEGATIVE), -f64::MIN_POSITIVE);
}

#[test]
#[allow(clippy::approx_constant)]
fn test_new() {
    // NaN
    assert!(Float32::new(f32::NAN).is_none());
    assert!(Float64::new(f64::NAN).is_none());

    // normal
    assert_eq!(f32::from(Float32::new(3.14).unwrap()), 3.14);
    assert_eq!(f64::from(Float64::new(3.14).unwrap()), 3.14);

    // infinity
    assert_eq!(Float32::new(f32::INFINITY).unwrap(), Float32::MAX);
    assert_eq!(Float32::new(-f32::INFINITY).unwrap(), Float32::MIN);
    assert_eq!(Float64::new(f64::INFINITY).unwrap(), Float64::MAX);
    assert_eq!(Float64::new(-f64::INFINITY).unwrap(), Float64::MIN);

    // zero
    assert_eq!(Float32::new(0.0).unwrap(), Float32::ZERO);
    assert_eq!(Float32::new(-0.0).unwrap(), Float32::ZERO);
    assert!(f32::from(Float32::new(-0.0).unwrap()).is_sign_positive());
    assert_eq!(Float64::new(0.0).unwrap(), Float64::ZERO);
    assert_eq!(Float64::new(-0.0).unwrap(), Float64::ZERO);
    assert!(f64::from(Float64::new(-0.0).unwrap()).is_sign_positive());

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
    assert_eq!(f32::from("3.14".parse::<Float32>().unwrap()), 3.14);
    assert_eq!(f64::from("3.14".parse::<Float64>().unwrap()), 3.14);

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
    assert!(f32::from("-0.0".parse::<Float32>().unwrap()).is_sign_positive());
    assert!(f64::from("-0.0".parse::<Float64>().unwrap()).is_sign_positive());
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
