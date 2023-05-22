use nearly::NearlyEqEps;

///////////////////
// nearly_eq_eps //
///////////////////

#[test]
fn nearly_eq_eps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_eps(&b, 0.0));
    assert!(b.nearly_eq_eps(&a, 0.0));
}

#[test]
fn nearly_eq_eps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_eps(&b, 0.0));
    assert!(b.nearly_eq_eps(&a, 0.0));
}

#[test]
fn nearly_eq_eps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_eps(&b, 0.0));
    assert!(b.nearly_eq_eps(&a, 0.0));
}

#[test]
fn nearly_eq_eps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_eps(&b, 0.0));
    assert!(b.nearly_eq_eps(&a, 0.0));
}

#[test]
fn nearly_eq_eps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_eps(&b, 0.0000007));
    assert!(!b.nearly_eq_eps(&a, 0.0000007));

    assert!(a.nearly_eq_eps(&b, 0.0000009));
    assert!(b.nearly_eq_eps(&a, 0.0000009));

    assert!(a.nearly_eq_eps(&b, 0.0000011));
    assert!(b.nearly_eq_eps(&a, 0.0000011));
}

#[test]
fn nearly_eq_eps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000003;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_eps(&b, 0.0000000000002));
    assert!(!b.nearly_eq_eps(&a, 0.0000000000002));

    assert!(a.nearly_eq_eps(&b, 0.0000000000003));
    assert!(b.nearly_eq_eps(&a, 0.0000000000003));

    assert!(a.nearly_eq_eps(&b, 0.0000000000004));
    assert!(b.nearly_eq_eps(&a, 0.0000000000004));
}

#[test]
fn nearly_eq_eps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(!a.nearly_eq_eps(&b, 0.019));
    assert!(!b.nearly_eq_eps(&a, 0.019));

    assert!(a.nearly_eq_eps(&b, 0.02));
    assert!(b.nearly_eq_eps(&a, 0.02));
}

#[test]
fn nearly_eq_eps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(!a.nearly_eq_eps(&b, 0.019));
    assert!(!b.nearly_eq_eps(&a, 0.019));

    assert!(a.nearly_eq_eps(&b, 0.02));
    assert!(b.nearly_eq_eps(&a, 0.02));
}

#[test]
fn nearly_eq_eps_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq_eps(&b, 0.000001));
    assert!(b.nearly_eq_eps(&a, 0.000001));
}

#[test]
fn nearly_eq_eps_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq_eps(&b, 0.000000000000001));
    assert!(b.nearly_eq_eps(&a, 0.000000000000001));
}

#[test]
fn nearly_eq_eps_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(!a.nearly_eq_eps(&b, f32::MAX));
    assert!(!b.nearly_eq_eps(&a, f32::MAX));

    assert!(f32::INFINITY.nearly_eq_eps(&f32::INFINITY, 0.0));
    assert!((-f32::INFINITY).nearly_eq_eps(&-f32::INFINITY, 0.0));
    assert!(!f32::INFINITY.nearly_eq_eps(&-f32::INFINITY, f32::MAX));
    assert!(!(-f32::INFINITY).nearly_eq_eps(&f32::INFINITY, f32::MAX));
}

#[test]
fn nearly_eq_eps_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(!a.nearly_eq_eps(&b, f64::MAX));
    assert!(!b.nearly_eq_eps(&a, f64::MAX));

    assert!(f64::INFINITY.nearly_eq_eps(&f64::INFINITY, 0.0));
    assert!((-f64::INFINITY).nearly_eq_eps(&-f64::INFINITY, 0.0));
    assert!(!f64::INFINITY.nearly_eq_eps(&-f64::INFINITY, f64::MAX));
    assert!(!(-f64::INFINITY).nearly_eq_eps(&f64::INFINITY, f64::MAX));
}

#[test]
fn nearly_eq_eps_nan_f32() {
    assert!(!f32::NAN.nearly_eq_eps(&f32::NAN, f32::MAX));
    assert!(!f32::NAN.nearly_eq_eps(&-f32::NAN, f32::MAX));
    assert!(!(-f32::NAN).nearly_eq_eps(&f32::NAN, f32::MAX));
    assert!(!(-f32::NAN).nearly_eq_eps(&-f32::NAN, f32::MAX));
}

#[test]
fn nearly_eq_eps_nan_f64() {
    assert!(!f64::NAN.nearly_eq_eps(&f64::NAN, f64::MAX));
    assert!(!f64::NAN.nearly_eq_eps(&-f64::NAN, f64::MAX));
    assert!(!(-f64::NAN).nearly_eq_eps(&f64::NAN, f64::MAX));
    assert!(!(-f64::NAN).nearly_eq_eps(&-f64::NAN, f64::MAX));
}

///////////////////
// nearly_ne_eps //
///////////////////

#[test]
fn nearly_ne_eps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_eps(&b, 0.0));
    assert!(!b.nearly_ne_eps(&a, 0.0));
}

#[test]
fn nearly_ne_eps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_eps(&b, 0.0));
    assert!(!b.nearly_ne_eps(&a, 0.0));
}

#[test]
fn nearly_ne_eps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_eps(&b, 0.0));
    assert!(!b.nearly_ne_eps(&a, 0.0));
}

#[test]
fn nearly_ne_eps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_eps(&b, 0.0));
    assert!(!b.nearly_ne_eps(&a, 0.0));
}

#[test]
fn nearly_ne_eps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;
    assert_ne!(a, b);

    assert!(a.nearly_ne_eps(&b, 0.0000007));
    assert!(b.nearly_ne_eps(&a, 0.0000007));

    assert!(!a.nearly_ne_eps(&b, 0.0000009));
    assert!(!b.nearly_ne_eps(&a, 0.0000009));

    assert!(!a.nearly_ne_eps(&b, 0.0000011));
    assert!(!b.nearly_ne_eps(&a, 0.0000011));
}

#[test]
fn nearly_ne_eps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000003;
    assert_ne!(a, b);

    assert!(a.nearly_ne_eps(&b, 0.0000000000002));
    assert!(b.nearly_ne_eps(&a, 0.0000000000002));

    assert!(!a.nearly_ne_eps(&b, 0.0000000000003));
    assert!(!b.nearly_ne_eps(&a, 0.0000000000003));

    assert!(!a.nearly_ne_eps(&b, 0.0000000000004));
    assert!(!b.nearly_ne_eps(&a, 0.0000000000004));
}

#[test]
fn nearly_ne_eps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(a.nearly_ne_eps(&b, 0.019));
    assert!(b.nearly_ne_eps(&a, 0.019));

    assert!(!a.nearly_ne_eps(&b, 0.02));
    assert!(!b.nearly_ne_eps(&a, 0.02));
}

#[test]
fn nearly_ne_eps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(a.nearly_ne_eps(&b, 0.019));
    assert!(b.nearly_ne_eps(&a, 0.019));

    assert!(!a.nearly_ne_eps(&b, 0.02));
    assert!(!b.nearly_ne_eps(&a, 0.02));
}

#[test]
fn nearly_ne_eps_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne_eps(&b, 0.000001));
    assert!(!b.nearly_ne_eps(&a, 0.000001));
}

#[test]
fn nearly_ne_eps_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne_eps(&b, 0.000000000000001));
    assert!(!b.nearly_ne_eps(&a, 0.000000000000001));
}

#[test]
fn nearly_ne_eps_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(a.nearly_ne_eps(&b, f32::MAX));
    assert!(b.nearly_ne_eps(&a, f32::MAX));

    assert!(!f32::INFINITY.nearly_ne_eps(&f32::INFINITY, 0.0));
    assert!(!(-f32::INFINITY).nearly_ne_eps(&-f32::INFINITY, 0.0));
    assert!(f32::INFINITY.nearly_ne_eps(&-f32::INFINITY, f32::MAX));
    assert!((-f32::INFINITY).nearly_ne_eps(&f32::INFINITY, f32::MAX));
}

#[test]
fn nearly_ne_eps_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(a.nearly_ne_eps(&b, f64::MAX));
    assert!(b.nearly_ne_eps(&a, f64::MAX));

    assert!(!f64::INFINITY.nearly_ne_eps(&f64::INFINITY, 0.0));
    assert!(!(-f64::INFINITY).nearly_ne_eps(&-f64::INFINITY, 0.0));
    assert!(f64::INFINITY.nearly_ne_eps(&-f64::INFINITY, f64::MAX));
    assert!((-f64::INFINITY).nearly_ne_eps(&f64::INFINITY, f64::MAX));
}

#[test]
fn nearly_ne_eps_nan_f32() {
    assert!(f32::NAN.nearly_ne_eps(&f32::NAN, f32::MAX));
    assert!(f32::NAN.nearly_ne_eps(&-f32::NAN, f32::MAX));
    assert!((-f32::NAN).nearly_ne_eps(&f32::NAN, f32::MAX));
    assert!((-f32::NAN).nearly_ne_eps(&-f32::NAN, f32::MAX));
}

#[test]
fn nearly_ne_eps_nan_f64() {
    assert!(f64::NAN.nearly_ne_eps(&f64::NAN, f64::MAX));
    assert!(f64::NAN.nearly_ne_eps(&-f64::NAN, f64::MAX));
    assert!((-f64::NAN).nearly_ne_eps(&f64::NAN, f64::MAX));
    assert!((-f64::NAN).nearly_ne_eps(&-f64::NAN, f64::MAX));
}
