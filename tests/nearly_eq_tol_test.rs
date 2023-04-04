use nearly::{NearlyEqTol, ToleranceF32, ToleranceF64};

#[test]
fn nearly_eq_tol_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 0)));
}

#[test]
fn nearly_eq_tol_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 0)));
}

#[test]
fn nearly_eq_tol_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 0)));
}

#[test]
fn nearly_eq_tol_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 0)));
}

#[test]
fn nearly_eq_tol_eps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.000003;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.000002, 0)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.000002, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.000003, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.000003, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.000004, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.000004, 0)));
}

#[test]
fn nearly_eq_tol_eps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000003;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.0000000000002, 0)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.0000000000002, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0000000000003, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0000000000003, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0000000000004, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0000000000004, 0)));
}

#[test]
fn nearly_eq_tol_ulps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 6)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 6)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 7)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 7)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 8)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 8)));
}

#[test]
fn nearly_eq_tol_ulps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000016;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 6)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 6)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 7)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 7)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 8)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 8)));
}

#[test]
fn nearly_eq_tol_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.019, i32::MAX)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.019, i32::MAX)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.02, i32::MAX)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.02, i32::MAX)));
}

#[test]
fn nearly_eq_tol_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.019, i64::MAX)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.019, i64::MAX)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.02, i64::MAX)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.02, i64::MAX)));
}

#[test]
fn nearly_eq_tol_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.000001, 1)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.000001, 1)));
}

#[test]
fn nearly_eq_tol_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000001, 1)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000001, 1)));
}

#[test]
fn nearly_eq_tol_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(f32::MAX, i32::MAX << 1)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(f32::MAX, i32::MAX << 1)));

    assert!(f32::INFINITY.nearly_eq_tol(&f32::INFINITY, ToleranceF32::new(0.0, 0)));
    assert!((-f32::INFINITY).nearly_eq_tol(&-f32::INFINITY, ToleranceF32::new(0.0, 0)));
    assert!(!f32::INFINITY.nearly_eq_tol(&-f32::INFINITY, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!(!(-f32::INFINITY).nearly_eq_tol(&f32::INFINITY, ToleranceF32::new(f32::MAX, i32::MAX)));
}

#[test]
fn nearly_eq_tol_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(f64::MAX, i64::MAX << 1)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(f64::MAX, i64::MAX << 1)));

    assert!(f64::INFINITY.nearly_eq_tol(&f64::INFINITY, ToleranceF64::new(0.0, 0)));
    assert!((-f64::INFINITY).nearly_eq_tol(&-f64::INFINITY, ToleranceF64::new(0.0, 0)));
    assert!(!f64::INFINITY.nearly_eq_tol(&-f64::INFINITY, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!(!(-f64::INFINITY).nearly_eq_tol(&f64::INFINITY, ToleranceF64::new(f64::MAX, i64::MAX)));
}

#[test]
fn nearly_eq_tol_nan_f32() {
    assert!(!f32::NAN.nearly_eq_tol(&f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!(!f32::NAN.nearly_eq_tol(&-f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!(!(-f32::NAN).nearly_eq_tol(&f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!(!(-f32::NAN).nearly_eq_tol(&-f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
}

#[test]
fn nearly_eq_tol_nan_f64() {
    assert!(!f64::NAN.nearly_eq_tol(&f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!(!f64::NAN.nearly_eq_tol(&-f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!(!(-f64::NAN).nearly_eq_tol(&f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!(!(-f64::NAN).nearly_eq_tol(&-f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
}

#[test]
fn nearly_eq_tol_array_f32() {
    let a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 6)));
    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.0000007, 0)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 6)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.0000007, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 7)));
    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0000009, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 7)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0000009, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 8)));
    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.000001, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 8)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.000001, 0)));
}

#[test]
fn nearly_eq_tol_array_f64() {
    let a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 6)));
    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000001, 0)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 6)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000001, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 7)));
    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000002, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 7)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000002, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 8)));
    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000003, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 8)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000003, 0)));
}

#[test]
fn nearly_eq_tol_slice_f32() {
    let array_a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let array_b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    let a: &[f32] = &array_a[1..4];
    let b: &[f32] = &array_b[1..4];

    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 6)));
    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.0000007, 0)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 6)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.0000007, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 7)));
    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0000009, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 7)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0000009, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 8)));
    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.000001, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 8)));
    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.000001, 0)));
}

#[test]
fn nearly_eq_tol_slice_f64() {
    let array_a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let array_b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    let a: &[f64] = &array_a[1..4];
    let b: &[f64] = &array_b[1..4];

    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 6)));
    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000001, 0)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 6)));
    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000001, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 7)));
    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000002, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 7)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000002, 0)));

    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 8)));
    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000003, 0)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 8)));
    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000003, 0)));
}

////////////////

#[test]
fn nearly_ne_tol_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 0)));
}

#[test]
fn nearly_ne_tol_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 0)));
}

#[test]
fn nearly_ne_tol_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 0)));
}

#[test]
fn nearly_ne_tol_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 0)));
}

#[test]
fn nearly_ne_tol_eps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.000003;
    assert_ne!(a, b);

    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.000002, 0)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.000002, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.000003, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.000003, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.000004, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.000004, 0)));
}

#[test]
fn nearly_ne_tol_eps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000003;
    assert_ne!(a, b);

    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.0000000000002, 0)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.0000000000002, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0000000000003, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0000000000003, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0000000000004, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0000000000004, 0)));
}

#[test]
fn nearly_ne_tol_ulps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;
    assert_ne!(a, b);

    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 6)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 6)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 7)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 7)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 8)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 8)));
}

#[test]
fn nearly_ne_tol_ulps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000016;
    assert_ne!(a, b);

    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 6)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 6)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 7)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 7)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 8)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 8)));
}

#[test]
fn nearly_ne_tol_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.019, i32::MAX)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.019, i32::MAX)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.02, i32::MAX)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.02, i32::MAX)));
}

#[test]
fn nearly_ne_tol_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.019, i64::MAX)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.019, i64::MAX)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.02, i64::MAX)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.02, i64::MAX)));
}

#[test]
fn nearly_ne_tol_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.000001, 1)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.000001, 1)));
}

#[test]
fn nearly_ne_tol_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000001, 1)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000001, 1)));
}

#[test]
fn nearly_ne_tol_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(f32::MAX, i32::MAX << 1)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(f32::MAX, i32::MAX << 1)));

    assert!(!f32::INFINITY.nearly_ne_tol(&f32::INFINITY, ToleranceF32::new(0.0, 0)));
    assert!(!(-f32::INFINITY).nearly_ne_tol(&-f32::INFINITY, ToleranceF32::new(0.0, 0)));
    assert!(f32::INFINITY.nearly_ne_tol(&-f32::INFINITY, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!((-f32::INFINITY).nearly_ne_tol(&f32::INFINITY, ToleranceF32::new(f32::MAX, i32::MAX)));
}

#[test]
fn nearly_ne_tol_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(f64::MAX, i64::MAX << 1)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(f64::MAX, i64::MAX << 1)));

    assert!(!f64::INFINITY.nearly_ne_tol(&f64::INFINITY, ToleranceF64::new(0.0, 0)));
    assert!(!(-f64::INFINITY).nearly_ne_tol(&-f64::INFINITY, ToleranceF64::new(0.0, 0)));
    assert!(f64::INFINITY.nearly_ne_tol(&-f64::INFINITY, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!((-f64::INFINITY).nearly_ne_tol(&f64::INFINITY, ToleranceF64::new(f64::MAX, i64::MAX)));
}

#[test]
fn nearly_ne_tol_nan_f32() {
    assert!(f32::NAN.nearly_ne_tol(&f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!(f32::NAN.nearly_ne_tol(&-f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!((-f32::NAN).nearly_ne_tol(&f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
    assert!((-f32::NAN).nearly_ne_tol(&-f32::NAN, ToleranceF32::new(f32::MAX, i32::MAX)));
}

#[test]
fn nearly_ne_tol_nan_f64() {
    assert!(f64::NAN.nearly_ne_tol(&f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!(f64::NAN.nearly_ne_tol(&-f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!((-f64::NAN).nearly_ne_tol(&f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
    assert!((-f64::NAN).nearly_ne_tol(&-f64::NAN, ToleranceF64::new(f64::MAX, i64::MAX)));
}

#[test]
fn nearly_ne_tol_array_f32() {
    let a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 6)));
    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.0000007, 0)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 6)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.0000007, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 7)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0000009, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 7)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0000009, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 8)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.000001, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 8)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.000001, 0)));
}

#[test]
fn nearly_ne_tol_array_f64() {
    let a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 6)));
    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000001, 0)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 6)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000001, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 7)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000002, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 7)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000002, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 8)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000003, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 8)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000003, 0)));
}

#[test]
fn nearly_ne_tol_slice_f32() {
    let array_a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let array_b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    let a: &[f32] = &array_a[1..4];
    let b: &[f32] = &array_b[1..4];

    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 6)));
    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.0000007, 0)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 6)));
    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.0000007, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 7)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0000009, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 7)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0000009, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 8)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.000001, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 8)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.000001, 0)));
}

#[test]
fn nearly_ne_tol_slice_f64() {
    let array_a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let array_b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    let a: &[f64] = &array_a[1..4];
    let b: &[f64] = &array_b[1..4];

    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 6)));
    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000001, 0)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 6)));
    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000001, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 7)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000002, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 7)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000002, 0)));

    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 8)));
    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000003, 0)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 8)));
    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000003, 0)));
}
