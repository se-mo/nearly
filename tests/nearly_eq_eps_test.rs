use nearly::NearlyEqEps;

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
    let b: f32 = 1.000003;
    assert_ne!(a, b);

    assert!(!a.nearly_eq_eps(&b, 0.000002));
    assert!(!b.nearly_eq_eps(&a, 0.000002));

    assert!(a.nearly_eq_eps(&b, 0.000003));
    assert!(b.nearly_eq_eps(&a, 0.000003));

    assert!(a.nearly_eq_eps(&b, 0.000004));
    assert!(b.nearly_eq_eps(&a, 0.000004));
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

#[test]
fn nearly_eq_eps_array_f32() {
    let a: [f32; 5] = [1.0, 1.0, 1.0, 1.000003, 1.0];
    let b: [f32; 5] = [1.0, 1.000003, 1.0, 1.0, 1.0];

    assert!(!a.nearly_eq_eps(&b, 0.000002));
    assert!(!b.nearly_eq_eps(&a, 0.000002));

    assert!(a.nearly_eq_eps(&b, 0.000003));
    assert!(b.nearly_eq_eps(&a, 0.000003));

    assert!(a.nearly_eq_eps(&b, 0.000004));
    assert!(b.nearly_eq_eps(&a, 0.000004));
}

#[test]
fn nearly_eq_eps_array_f64() {
    let a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000003, 1.0];
    let b: [f64; 5] = [1.0, 1.0000000000003, 1.0, 1.0, 1.0];

    assert!(!a.nearly_eq_eps(&b, 0.0000000000002));
    assert!(!b.nearly_eq_eps(&a, 0.0000000000002));

    assert!(a.nearly_eq_eps(&b, 0.0000000000003));
    assert!(b.nearly_eq_eps(&a, 0.0000000000003));

    assert!(a.nearly_eq_eps(&b, 0.0000000000004));
    assert!(b.nearly_eq_eps(&a, 0.0000000000004));
}

#[test]
fn nearly_eq_eps_slice_f32() {
    let array_a: [f32; 5] = [1.0, 1.0, 1.0, 1.000003, 1.0];
    let array_b: [f32; 5] = [1.0, 1.000003, 1.0, 1.0, 1.0];

    let a: &[f32] = &array_a[1..4];
    let b: &[f32] = &array_b[1..4];

    assert!(!a.nearly_eq_eps(&b, 0.000002));
    assert!(!b.nearly_eq_eps(&a, 0.000002));

    assert!(a.nearly_eq_eps(&b, 0.000003));
    assert!(b.nearly_eq_eps(&a, 0.000003));

    assert!(a.nearly_eq_eps(&b, 0.000004));
    assert!(b.nearly_eq_eps(&a, 0.000004));
}

#[test]
fn nearly_eq_eps_slice_f64() {
    let array_a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000003, 1.0];
    let array_b: [f64; 5] = [1.0, 1.0000000000003, 1.0, 1.0, 1.0];

    let a: &[f64] = &array_a[1..4];
    let b: &[f64] = &array_b[1..4];

    assert!(!a.nearly_eq_eps(&b, 0.0000000000002));
    assert!(!b.nearly_eq_eps(&a, 0.0000000000002));

    assert!(a.nearly_eq_eps(&b, 0.0000000000003));
    assert!(b.nearly_eq_eps(&a, 0.0000000000003));

    assert!(a.nearly_eq_eps(&b, 0.0000000000004));
    assert!(b.nearly_eq_eps(&a, 0.0000000000004));
}

////////////////

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
    let b: f32 = 1.000003;
    assert_ne!(a, b);

    assert!(a.nearly_ne_eps(&b, 0.000002));
    assert!(b.nearly_ne_eps(&a, 0.000002));

    assert!(!a.nearly_ne_eps(&b, 0.000003));
    assert!(!b.nearly_ne_eps(&a, 0.000003));

    assert!(!a.nearly_ne_eps(&b, 0.000004));
    assert!(!b.nearly_ne_eps(&a, 0.000004));
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

#[test]
fn nearly_ne_eps_array_f32() {
    let a: [f32; 4] = [1.0, 1.0, 1.0, 1.000003];
    let b: [f32; 4] = [1.0, 1.000003, 1.0, 1.0];

    assert!(a.nearly_ne_eps(&b, 0.000002));
    assert!(b.nearly_ne_eps(&a, 0.000002));

    assert!(!a.nearly_ne_eps(&b, 0.000003));
    assert!(!b.nearly_ne_eps(&a, 0.000003));

    assert!(!a.nearly_ne_eps(&b, 0.000004));
    assert!(!b.nearly_ne_eps(&a, 0.000004));
}

#[test]
fn nearly_ne_eps_array_f64() {
    let a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000003, 1.0];
    let b: [f64; 5] = [1.0, 1.0000000000003, 1.0, 1.0, 1.0];

    assert!(a.nearly_ne_eps(&b, 0.0000000000002));
    assert!(b.nearly_ne_eps(&a, 0.0000000000002));

    assert!(!a.nearly_ne_eps(&b, 0.0000000000003));
    assert!(!b.nearly_ne_eps(&a, 0.0000000000003));

    assert!(!a.nearly_ne_eps(&b, 0.0000000000004));
    assert!(!b.nearly_ne_eps(&a, 0.0000000000004));
}

#[test]
fn nearly_ne_eps_slice_f32() {
    let array_a: [f32; 4] = [1.0, 1.0, 1.0, 1.000003];
    let array_b: [f32; 4] = [1.0, 1.000003, 1.0, 1.0];

    let a: &[f32] = &array_a[1..4];
    let b: &[f32] = &array_b[1..4];

    assert!(a.nearly_ne_eps(&b, 0.000002));
    assert!(b.nearly_ne_eps(&a, 0.000002));

    assert!(!a.nearly_ne_eps(&b, 0.000003));
    assert!(!b.nearly_ne_eps(&a, 0.000003));

    assert!(!a.nearly_ne_eps(&b, 0.000004));
    assert!(!b.nearly_ne_eps(&a, 0.000004));
}

#[test]
fn nearly_ne_eps_slice_f64() {
    let array_a: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000003, 1.0];
    let array_b: [f64; 5] = [1.0, 1.0000000000003, 1.0, 1.0, 1.0];

    let a: &[f64] = &array_a[1..4];
    let b: &[f64] = &array_b[1..4];

    assert!(a.nearly_ne_eps(&b, 0.0000000000002));
    assert!(b.nearly_ne_eps(&a, 0.0000000000002));

    assert!(!a.nearly_ne_eps(&b, 0.0000000000003));
    assert!(!b.nearly_ne_eps(&a, 0.0000000000003));

    assert!(!a.nearly_ne_eps(&b, 0.0000000000004));
    assert!(!b.nearly_ne_eps(&a, 0.0000000000004));
}
