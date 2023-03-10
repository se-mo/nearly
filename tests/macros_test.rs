use nearly::{nearly_eq, nearly_ne, ToleranceF32, ToleranceF64};

#[test]
fn macro_nearly_eq_eps_f32() {
    assert!(nearly_eq!(0.0_f32, 0.0_f32, eps = 0.0_f32));
    assert!(nearly_eq!(1.0_f32, 1.0_f32, eps = 0.01_f32));
    assert!(nearly_eq!(1.0_f32, 1.000003, eps = 0.000004_f32));
    assert!(nearly_eq!(1.0_f32, 1.1_f32, eps = 1.11_f32));
    assert!(!nearly_eq!(1.0_f32, f32::INFINITY, eps = f32::MAX));
    assert!(!nearly_eq!(1.0_f32, f32::NAN, eps = f32::MAX));
}

#[test]
fn macro_nearly_eq_eps_f64() {
    assert!(nearly_eq!(0.0_f64, 0.0_f64, eps = 0.0_f64));
    assert!(nearly_eq!(1.0_f64, 1.0_f64, eps = 0.01_f64));
    assert!(nearly_eq!(
        1.0_f64,
        1.0000000000003_f64,
        eps = 0.0000000000004_f64
    ));
    assert!(nearly_eq!(1.0_f64, 1.1_f64, eps = 1.11_f64));
    assert!(!nearly_eq!(1.0_f64, f64::INFINITY, eps = f64::MAX));
    assert!(!nearly_eq!(1.0_f64, f64::NAN, eps = f64::MAX));
}

#[test]
fn macro_nearly_eq_ulps_f32() {
    assert!(nearly_eq!(0.0_f32, 0.0_f32, ulps = 4_i32));
    assert!(nearly_eq!(1.0_f32, 1.0_f32, ulps = 4_i32));
    assert!(nearly_eq!(1.0_f32, 1.0000005_f32, ulps = 4_i32));
    assert!(nearly_eq!(-1.0_f32, -1.0000005_f32, ulps = 4_i32));
    assert!(!nearly_eq!(1.0_f32, f32::INFINITY, ulps = i32::MAX << 1));
    assert!(!nearly_eq!(1.0_f32, f32::NAN, ulps = i32::MAX));
}

#[test]
fn macro_nearly_eq_ulps_f64() {
    assert!(nearly_eq!(0.0_f64, 0.0_f64, ulps = 4_i64));
    assert!(nearly_eq!(1.0_f64, 1.0_f64, ulps = 4_i64));
    assert!(nearly_eq!(1.0_f64, 1.0000000000000009_f64, ulps = 4_i64));
    assert!(nearly_eq!(-1.0_f64, -1.0000000000000009_f64, ulps = 4_i64));
    assert!(!nearly_eq!(1.0_f64, f64::INFINITY, ulps = i64::MAX << 1));
    assert!(!nearly_eq!(1.0_f64, f64::NAN, ulps = i64::MAX));
}

#[test]
fn macro_nearly_eq_tol_f32() {
    assert!(nearly_eq!(0.0_f32, 0.0_f32, tol = ToleranceF32::default()));
    assert!(nearly_eq!(1.0_f32, 1.0_f32, tol = ToleranceF32::default()));
    assert!(nearly_eq!(
        1.0_f32,
        1.0000005_f32,
        tol = ToleranceF32::default()
    ));
    assert!(nearly_eq!(
        -1.0_f32,
        -1.0000005_f32,
        tol = ToleranceF32::default()
    ));
    assert!(!nearly_eq!(
        1.0_f32,
        f32::INFINITY,
        tol = ToleranceF32::new(f32::MAX, i32::MAX << 1)
    ));
    assert!(!nearly_eq!(
        1.0_f32,
        f32::NAN,
        tol = ToleranceF32::new(f32::MAX, i32::MAX)
    ));
}

#[test]
fn macro_nearly_eq_tol_f64() {
    assert!(nearly_eq!(0.0_f64, 0.0_f64, tol = ToleranceF64::default()));
    assert!(nearly_eq!(1.0_f64, 1.0_f64, tol = ToleranceF64::default()));
    assert!(nearly_eq!(
        1.0_f64,
        1.0000000000000009_f64,
        tol = ToleranceF64::default()
    ));
    assert!(nearly_eq!(
        -1.0_f64,
        -1.0000000000000009_f64,
        tol = ToleranceF64::default()
    ));
    assert!(!nearly_eq!(
        1.0_f64,
        f64::INFINITY,
        tol = ToleranceF64::new(f64::MAX, i64::MAX << 1)
    ));
    assert!(!nearly_eq!(
        1.0_f64,
        f64::NAN,
        tol = ToleranceF64::new(f64::MAX, i64::MAX)
    ));
}

#[test]
fn macro_nearly_eq_f32() {
    assert!(nearly_eq!(0.0_f32, 0.0_f32));
    assert!(nearly_eq!(1.0_f32, 1.0_f32));
    assert!(nearly_eq!(1.0_f32, 1.0000005_f32));
    assert!(nearly_eq!(-1.0_f32, -1.0000005_f32));
    assert!(!nearly_eq!(1.0_f32, f32::INFINITY));
    assert!(!nearly_eq!(1.0_f32, f32::NAN));
}

#[test]
fn macro_nearly_eq_f64() {
    assert!(nearly_eq!(0.0_f64, 0.0_f64));
    assert!(nearly_eq!(1.0_f64, 1.0_f64));
    assert!(nearly_eq!(1.0_f64, 1.0000000000000009_f64));
    assert!(nearly_eq!(-1.0_f64, -1.0000000000000009_f64));
    assert!(!nearly_eq!(1.0_f64, f64::INFINITY));
    assert!(!nearly_eq!(1.0_f64, f64::NAN));
}

////////////////

#[test]
fn macro_nearly_ne_eps_f32() {
    assert!(nearly_ne!(0.0_f32, 0.00001_f32, eps = 0.000005_f32));
    assert!(nearly_ne!(1.0_f32, -1.0_f32, eps = 0.1_f32));
    assert!(nearly_ne!(2.3_f32, 5.9_f32, eps = 2.6_f32));
    assert!(nearly_ne!(1.0_f32, f32::INFINITY, eps = f32::MAX));
    assert!(nearly_ne!(1.0_f32, f32::NAN, eps = f32::MAX));
}

#[test]
fn macro_nearly_ne_eps_f64() {
    assert!(nearly_ne!(0.0_f64, 0.00001_f64, eps = 0.000005_f64));
    assert!(nearly_ne!(1.0_f64, -1.0_f64, eps = 0.1_f64));
    assert!(nearly_ne!(2.3_f64, 5.9_f64, eps = 2.6_f64));
    assert!(nearly_ne!(1.0_f64, f64::INFINITY, eps = f64::MAX));
    assert!(nearly_ne!(1.0_f64, f64::NAN, eps = f64::MAX));
}

#[test]
fn macro_nearly_ne_ulps_f32() {
    assert!(nearly_ne!(0.0_f32, 0.00001_f32, ulps = 4_i32));
    assert!(nearly_ne!(1.0_f32, -1.0_f32, ulps = 100_i32));
    assert!(nearly_ne!(2.3_f32, 5.9_f32, ulps = 100_i32));
    assert!(nearly_ne!(1.0_f32, f32::INFINITY, ulps = i32::MAX << 1));
    assert!(nearly_ne!(1.0_f32, f32::NAN, ulps = i32::MAX));
}

#[test]
fn macro_nearly_ne_ulps_f64() {
    assert!(nearly_ne!(0.0_f64, 0.00001_f64, ulps = 4_i64));
    assert!(nearly_ne!(1.0_f64, -1.0_f64, ulps = 100_i64));
    assert!(nearly_ne!(2.3_f64, 5.9_f64, ulps = 100_i64));
    assert!(nearly_ne!(1.0_f64, f64::INFINITY, ulps = i64::MAX << 1));
    assert!(nearly_ne!(1.0_f64, f64::NAN, ulps = i64::MAX));
}

#[test]
fn macro_nearly_ne_tol_f32() {
    assert!(nearly_ne!(
        0.0_f32,
        0.00001_f32,
        tol = ToleranceF32::default()
    ));
    assert!(nearly_ne!(
        1.0_f32,
        -1.0_f32,
        tol = ToleranceF32::new(0.1_f32, 100_i32)
    ));
    assert!(nearly_ne!(
        2.3_f32,
        5.9_f32,
        tol = ToleranceF32::new(2.6_f32, 100_i32)
    ));
    assert!(nearly_ne!(
        1.0_f32,
        f32::INFINITY,
        tol = ToleranceF32::new(f32::MAX, i32::MAX << 1)
    ));
    assert!(nearly_ne!(
        1.0_f32,
        f32::NAN,
        tol = ToleranceF32::new(f32::MAX, i32::MAX)
    ));
}

#[test]
fn macro_nearly_ne_tol_f64() {
    assert!(nearly_ne!(
        0.0_f64,
        0.00001_f64,
        tol = ToleranceF64::default()
    ));
    assert!(nearly_ne!(
        1.0_f64,
        -1.0_f64,
        tol = ToleranceF64::new(0.1_f64, 100_i64)
    ));
    assert!(nearly_ne!(
        2.3_f64,
        5.9_f64,
        tol = ToleranceF64::new(2.6_f64, 100_i64)
    ));
    assert!(nearly_ne!(
        1.0_f64,
        f64::INFINITY,
        tol = ToleranceF64::new(f64::MAX, i64::MAX << 1)
    ));
    assert!(nearly_ne!(
        1.0_f64,
        f64::NAN,
        tol = ToleranceF64::new(f64::MAX, i64::MAX)
    ));
}

#[test]
fn macro_nearly_ne_f32() {
    assert!(nearly_ne!(0.0_f32, 0.00001_f32));
    assert!(nearly_ne!(1.0_f32, -1.0_f32));
    assert!(nearly_ne!(2.3_f32, 5.9_f32));
    assert!(nearly_ne!(1.0_f32, f32::INFINITY));
    assert!(nearly_ne!(1.0_f32, f32::NAN));
}

#[test]
fn macro_nearly_ne_f64() {
    assert!(nearly_ne!(0.0_f64, 0.00001_f64));
    assert!(nearly_ne!(1.0_f64, -1.0_f64));
    assert!(nearly_ne!(2.3_f64, 5.9_f64));
    assert!(nearly_ne!(1.0_f64, f64::INFINITY));
    assert!(nearly_ne!(1.0_f64, f64::NAN));
}
