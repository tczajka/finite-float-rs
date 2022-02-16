use finite_float::{Float32, Float64};

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
