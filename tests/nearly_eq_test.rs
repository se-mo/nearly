use nearly::NearlyEq;

#[test]
fn nearly_eq_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000005;
    assert_ne!(a, b);

    println!("e: {}", f32::EPSILON);
    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000009;
    assert_ne!(a, b);

    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_different_sign_f32() {
    let a: f32 = 0.0000001192;
    let b: f32 = -0.0000001192;

    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_different_sign_f64() {
    let a: f64 = 0.000000000000000222;
    let b: f64 = -0.000000000000000222;

    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq(&b));
    assert!(b.nearly_eq(&a));
}

#[test]
fn nearly_eq_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(!a.nearly_eq(&b));
    assert!(!b.nearly_eq(&a));

    assert!(f32::INFINITY.nearly_eq(&f32::INFINITY));
    assert!((-f32::INFINITY).nearly_eq(&-f32::INFINITY));
    assert!(!f32::INFINITY.nearly_eq(&-f32::INFINITY));
    assert!(!(-f32::INFINITY).nearly_eq(&f32::INFINITY));
}

#[test]
fn nearly_eq_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(!a.nearly_eq(&b));
    assert!(!b.nearly_eq(&a));

    assert!(f64::INFINITY.nearly_eq(&f64::INFINITY));
    assert!((-f64::INFINITY).nearly_eq(&-f64::INFINITY));
    assert!(!f64::INFINITY.nearly_eq(&-f64::INFINITY));
    assert!(!(-f64::INFINITY).nearly_eq(&f64::INFINITY));
}

#[test]
fn nearly_eq_nan_f32() {
    assert!(!f32::NAN.nearly_eq(&f32::NAN));
    assert!(!f32::NAN.nearly_eq(&-f32::NAN));
    assert!(!(-f32::NAN).nearly_eq(&f32::NAN));
    assert!(!(-f32::NAN).nearly_eq(&-f32::NAN));
}

#[test]
fn nearly_eq_nan_f64() {
    assert!(!f64::NAN.nearly_eq(&f64::NAN));
    assert!(!f64::NAN.nearly_eq(&-f64::NAN));
    assert!(!(-f64::NAN).nearly_eq(&f64::NAN));
    assert!(!(-f64::NAN).nearly_eq(&-f64::NAN));
}

////////////////

#[test]
fn nearly_ne_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000005;
    assert_ne!(a, b);

    println!("e: {}", f32::EPSILON);
    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000009;
    assert_ne!(a, b);

    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_different_sign_f32() {
    let a: f32 = 0.0000001192;
    let b: f32 = -0.0000001192;

    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_different_sign_f64() {
    let a: f64 = 0.000000000000000222;
    let b: f64 = -0.000000000000000222;

    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne(&b));
    assert!(!b.nearly_ne(&a));
}

#[test]
fn nearly_ne_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(a.nearly_ne(&b));
    assert!(b.nearly_ne(&a));

    assert!(!f32::INFINITY.nearly_ne(&f32::INFINITY));
    assert!(!(-f32::INFINITY).nearly_ne(&-f32::INFINITY));
    assert!(f32::INFINITY.nearly_ne(&-f32::INFINITY));
    assert!((-f32::INFINITY).nearly_ne(&f32::INFINITY));
}

#[test]
fn nearly_ne_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(a.nearly_ne(&b));
    assert!(b.nearly_ne(&a));

    assert!(!f64::INFINITY.nearly_ne(&f64::INFINITY));
    assert!(!(-f64::INFINITY).nearly_ne(&-f64::INFINITY));
    assert!(f64::INFINITY.nearly_ne(&-f64::INFINITY));
    assert!((-f64::INFINITY).nearly_ne(&f64::INFINITY));
}

#[test]
fn nearly_ne_nan_f32() {
    assert!(f32::NAN.nearly_ne(&f32::NAN));
    assert!(f32::NAN.nearly_ne(&-f32::NAN));
    assert!((-f32::NAN).nearly_ne(&f32::NAN));
    assert!((-f32::NAN).nearly_ne(&-f32::NAN));
}

#[test]
fn nearly_ne_nan_f64() {
    assert!(f64::NAN.nearly_ne(&f64::NAN));
    assert!(f64::NAN.nearly_ne(&-f64::NAN));
    assert!((-f64::NAN).nearly_ne(&f64::NAN));
    assert!((-f64::NAN).nearly_ne(&-f64::NAN));
}
