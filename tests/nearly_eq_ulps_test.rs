use nearly::NearlyEqUlps;

#[test]
fn nearly_eq_ulps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_ulps(&b, 0));
    assert!(b.nearly_eq_ulps(&a, 0));
}

#[test]
fn nearly_eq_ulps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_ulps(&b, 0));
    assert!(b.nearly_eq_ulps(&a, 0));
}

#[test]
fn nearly_eq_ulps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_ulps(&b, 0));
    assert!(b.nearly_eq_ulps(&a, 0));
}

#[test]
fn nearly_eq_ulps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(a.nearly_eq_ulps(&b, 0));
    assert!(b.nearly_eq_ulps(&a, 0));
}

#[test]
fn nearly_eq_ulps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_ulps(&b, 6));
    assert!(!b.nearly_eq_ulps(&a, 6));

    assert!(a.nearly_eq_ulps(&b, 7));
    assert!(b.nearly_eq_ulps(&a, 7));

    assert!(a.nearly_eq_ulps(&b, 8));
    assert!(b.nearly_eq_ulps(&a, 8));
}

#[test]
fn nearly_eq_ulps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000016;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_ulps(&b, 6));
    assert!(!b.nearly_eq_ulps(&a, 6));

    assert!(a.nearly_eq_ulps(&b, 7));
    assert!(b.nearly_eq_ulps(&a, 7));

    assert!(a.nearly_eq_ulps(&b, 8));
    assert!(b.nearly_eq_ulps(&a, 8));
}

#[test]
fn nearly_eq_ulps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;
    assert!(!a.nearly_eq_ulps(&b, i32::MAX));
    assert!(!b.nearly_eq_ulps(&a, i32::MAX));
}

#[test]
fn nearly_eq_ulps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;
    assert!(!a.nearly_eq_ulps(&b, i64::MAX));
    assert!(!b.nearly_eq_ulps(&a, i64::MAX));
}

#[test]
fn nearly_eq_ulps_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq_ulps(&b, 1));
    assert!(b.nearly_eq_ulps(&a, 1));
}

#[test]
fn nearly_eq_ulps_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(a.nearly_eq_ulps(&b, 1));
    assert!(b.nearly_eq_ulps(&a, 1));
}

#[test]
fn nearly_eq_ulps_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(!a.nearly_eq_ulps(&b, i32::MAX << 1));
    assert!(!b.nearly_eq_ulps(&a, i32::MAX << 1));

    assert!(f32::INFINITY.nearly_eq_ulps(&f32::INFINITY, 0));
    assert!((-f32::INFINITY).nearly_eq_ulps(&-f32::INFINITY, 0));
    assert!(!f32::INFINITY.nearly_eq_ulps(&-f32::INFINITY, i32::MAX));
    assert!(!(-f32::INFINITY).nearly_eq_ulps(&f32::INFINITY, i32::MAX));
}

#[test]
fn nearly_eq_ulps_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(!a.nearly_eq_ulps(&b, i64::MAX << 1));
    assert!(!b.nearly_eq_ulps(&a, i64::MAX << 1));

    assert!(f64::INFINITY.nearly_eq_ulps(&f64::INFINITY, 0));
    assert!((-f64::INFINITY).nearly_eq_ulps(&-f64::INFINITY, 0));
    assert!(!f64::INFINITY.nearly_eq_ulps(&-f64::INFINITY, i64::MAX));
    assert!(!(-f64::INFINITY).nearly_eq_ulps(&f64::INFINITY, i64::MAX));
}

#[test]
fn nearly_eq_ulps_nan_f32() {
    assert!(!f32::NAN.nearly_eq_ulps(&f32::NAN, i32::MAX));
    assert!(!f32::NAN.nearly_eq_ulps(&-f32::NAN, i32::MAX));
    assert!(!(-f32::NAN).nearly_eq_ulps(&f32::NAN, i32::MAX));
    assert!(!(-f32::NAN).nearly_eq_ulps(&-f32::NAN, i32::MAX));
}

#[test]
fn nearly_eq_ulps_nan_f64() {
    assert!(!f64::NAN.nearly_eq_ulps(&f64::NAN, i64::MAX));
    assert!(!f64::NAN.nearly_eq_ulps(&-f64::NAN, i64::MAX));
    assert!(!(-f64::NAN).nearly_eq_ulps(&f64::NAN, i64::MAX));
    assert!(!(-f64::NAN).nearly_eq_ulps(&-f64::NAN, i64::MAX));
}

#[test]
fn nearly_eq_ulps_array_f32() {
    let a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    assert!(!a.nearly_eq_ulps(&b, 6));
    assert!(!b.nearly_eq_ulps(&a, 6));

    assert!(a.nearly_eq_ulps(&b, 7));
    assert!(b.nearly_eq_ulps(&a, 7));

    assert!(a.nearly_eq_ulps(&b, 8));
    assert!(b.nearly_eq_ulps(&a, 8));
}

#[test]
fn nearly_eq_ulps_array_f64() {
    let a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    assert!(!a.nearly_eq_ulps(&b, 6));
    assert!(!b.nearly_eq_ulps(&a, 6));

    assert!(a.nearly_eq_ulps(&b, 7));
    assert!(b.nearly_eq_ulps(&a, 7));

    assert!(a.nearly_eq_ulps(&b, 8));
    assert!(b.nearly_eq_ulps(&a, 8));
}

#[test]
fn nearly_eq_ulps_slice_f32() {
    let array_a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let array_b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    let a: &[f32] = &array_a[1..4];
    let b: &[f32] = &array_b[1..4];

    assert!(!a.nearly_eq_ulps(&b, 6));
    assert!(!b.nearly_eq_ulps(&a, 6));

    assert!(a.nearly_eq_ulps(&b, 7));
    assert!(b.nearly_eq_ulps(&a, 7));

    assert!(a.nearly_eq_ulps(&b, 8));
    assert!(b.nearly_eq_ulps(&a, 8));
}

#[test]
fn nearly_eq_ulps_slice_f64() {
    let array_a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let array_b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    let a: &[f64] = &array_a[1..4];
    let b: &[f64] = &array_b[1..4];

    assert!(!a.nearly_eq_ulps(&b, 6));
    assert!(!b.nearly_eq_ulps(&a, 6));

    assert!(a.nearly_eq_ulps(&b, 7));
    assert!(b.nearly_eq_ulps(&a, 7));

    assert!(a.nearly_eq_ulps(&b, 8));
    assert!(b.nearly_eq_ulps(&a, 8));
}

////////////////

#[test]
fn nearly_ne_ulps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_ulps(&b, 0));
    assert!(!b.nearly_ne_ulps(&a, 0));
}

#[test]
fn nearly_ne_ulps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_ulps(&b, 0));
    assert!(!b.nearly_ne_ulps(&a, 0));
}

#[test]
fn nearly_ne_ulps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_ulps(&b, 0));
    assert!(!b.nearly_ne_ulps(&a, 0));
}

#[test]
fn nearly_ne_ulps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert_eq!(a, b);
    assert!(!a.nearly_ne_ulps(&b, 0));
    assert!(!b.nearly_ne_ulps(&a, 0));
}

#[test]
fn nearly_ne_ulps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;
    assert_ne!(a, b);

    assert!(a.nearly_ne_ulps(&b, 6));
    assert!(b.nearly_ne_ulps(&a, 6));

    assert!(!a.nearly_ne_ulps(&b, 7));
    assert!(!b.nearly_ne_ulps(&a, 7));

    assert!(!a.nearly_ne_ulps(&b, 8));
    assert!(!b.nearly_ne_ulps(&a, 8));
}

#[test]
fn nearly_ne_ulps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000016;
    assert_ne!(a, b);

    assert!(a.nearly_ne_ulps(&b, 6));
    assert!(b.nearly_ne_ulps(&a, 6));

    assert!(!a.nearly_ne_ulps(&b, 7));
    assert!(!b.nearly_ne_ulps(&a, 7));

    assert!(!a.nearly_ne_ulps(&b, 8));
    assert!(!b.nearly_ne_ulps(&a, 8));
}

#[test]
fn nearly_ne_ulps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;
    assert!(a.nearly_ne_ulps(&b, i32::MAX));
    assert!(b.nearly_ne_ulps(&a, i32::MAX));
}

#[test]
fn nearly_ne_ulps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;
    assert!(a.nearly_ne_ulps(&b, i64::MAX));
    assert!(b.nearly_ne_ulps(&a, i64::MAX));
}

#[test]
fn nearly_ne_ulps_sum_f32() {
    let mut a: f32 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f32 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne_ulps(&b, 1));
    assert!(!b.nearly_ne_ulps(&a, 1));
}

#[test]
fn nearly_ne_ulps_sum_f64() {
    let mut a: f64 = 0.0;
    for _i in 0..10 {
        a += 0.1;
    }
    let b: f64 = 1.0;

    assert_ne!(a, b);
    assert!(!a.nearly_ne_ulps(&b, 1));
    assert!(!b.nearly_ne_ulps(&a, 1));
}

#[test]
fn nearly_ne_ulps_inf_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::INFINITY;
    assert!(a.nearly_ne_ulps(&b, i32::MAX << 1));
    assert!(b.nearly_ne_ulps(&a, i32::MAX << 1));

    assert!(!f32::INFINITY.nearly_ne_ulps(&f32::INFINITY, 0));
    assert!(!(-f32::INFINITY).nearly_ne_ulps(&-f32::INFINITY, 0));
    assert!(f32::INFINITY.nearly_ne_ulps(&-f32::INFINITY, i32::MAX));
    assert!((-f32::INFINITY).nearly_ne_ulps(&f32::INFINITY, i32::MAX));
}

#[test]
fn nearly_ne_ulps_inf_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::INFINITY;
    assert!(a.nearly_ne_ulps(&b, i64::MAX << 1));
    assert!(b.nearly_ne_ulps(&a, i64::MAX << 1));

    assert!(!f64::INFINITY.nearly_ne_ulps(&f64::INFINITY, 0));
    assert!(!(-f64::INFINITY).nearly_ne_ulps(&-f64::INFINITY, 0));
    assert!(f64::INFINITY.nearly_ne_ulps(&-f64::INFINITY, i64::MAX));
    assert!((-f64::INFINITY).nearly_ne_ulps(&f64::INFINITY, i64::MAX));
}

#[test]
fn nearly_ne_ulps_nan_f32() {
    assert!(f32::NAN.nearly_ne_ulps(&f32::NAN, i32::MAX));
    assert!(f32::NAN.nearly_ne_ulps(&-f32::NAN, i32::MAX));
    assert!((-f32::NAN).nearly_ne_ulps(&f32::NAN, i32::MAX));
    assert!((-f32::NAN).nearly_ne_ulps(&-f32::NAN, i32::MAX));
}

#[test]
fn nearly_ne_ulps_nan_f64() {
    assert!(f64::NAN.nearly_ne_ulps(&f64::NAN, i64::MAX));
    assert!(f64::NAN.nearly_ne_ulps(&-f64::NAN, i64::MAX));
    assert!((-f64::NAN).nearly_ne_ulps(&f64::NAN, i64::MAX));
    assert!((-f64::NAN).nearly_ne_ulps(&-f64::NAN, i64::MAX));
}

#[test]
fn nearly_ne_ulps_array_f32() {
    let a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    assert!(a.nearly_ne_ulps(&b, 6));
    assert!(b.nearly_ne_ulps(&a, 6));

    assert!(!a.nearly_ne_ulps(&b, 7));
    assert!(!b.nearly_ne_ulps(&a, 7));

    assert!(!a.nearly_ne_ulps(&b, 8));
    assert!(!b.nearly_ne_ulps(&a, 8));
}

#[test]
fn nearly_ne_ulps_array_f64() {
    let a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    assert!(a.nearly_ne_ulps(&b, 6));
    assert!(b.nearly_ne_ulps(&a, 6));

    assert!(!a.nearly_ne_ulps(&b, 7));
    assert!(!b.nearly_ne_ulps(&a, 7));

    assert!(!a.nearly_ne_ulps(&b, 8));
    assert!(!b.nearly_ne_ulps(&a, 8));
}

#[test]
fn nearly_ne_ulps_slice_f32() {
    let array_a: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
    let array_b: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];

    let a: &[f32] = &array_a[1..4];
    let b: &[f32] = &array_b[1..4];

    assert!(a.nearly_ne_ulps(&b, 6));
    assert!(b.nearly_ne_ulps(&a, 6));

    assert!(!a.nearly_ne_ulps(&b, 7));
    assert!(!b.nearly_ne_ulps(&a, 7));

    assert!(!a.nearly_ne_ulps(&b, 8));
    assert!(!b.nearly_ne_ulps(&a, 8));
}

#[test]
fn nearly_ne_ulps_slice_f64() {
    let array_a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
    let array_b: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];

    let a: &[f64] = &array_a[1..4];
    let b: &[f64] = &array_b[1..4];

    assert!(a.nearly_ne_ulps(&b, 6));
    assert!(b.nearly_ne_ulps(&a, 6));

    assert!(!a.nearly_ne_ulps(&b, 7));
    assert!(!b.nearly_ne_ulps(&a, 7));

    assert!(!a.nearly_ne_ulps(&b, 8));
    assert!(!b.nearly_ne_ulps(&a, 8));
}
