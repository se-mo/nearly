use nearly::{assert_nearly_eq, assert_nearly_ne, ToleranceF32, ToleranceF64};

#[test]
fn macro_assert_nearly_eq_eps_f32() {
    assert_nearly_eq!(0.0_f32, 0.0_f32, eps = 0.0_f32);
    assert_nearly_eq!(1.0_f32, 1.0_f32, eps = 0.01_f32);
    assert_nearly_eq!(1.0_f32, 1.000003, eps = 0.000004_f32);
    assert_nearly_eq!(1.0_f32, 1.1_f32, eps = 1.11_f32);
}

#[test]
fn macro_assert_nearly_eq_eps_f64() {
    assert_nearly_eq!(0.0_f64, 0.0_f64, eps = 0.0_f64);
    assert_nearly_eq!(1.0_f64, 1.0_f64, eps = 0.01_f64);
    assert_nearly_eq!(1.0_f64, 1.0000000000003_f64, eps = 0.0000000000004_f64);
    assert_nearly_eq!(1.0_f64, 1.1_f64, eps = 1.11_f64);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_eps right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.09`"#)]
fn macro_assert_nearly_eq_eps_panic_f32() {
    assert_nearly_eq!(1.0_f32, 1.1_f32, eps = 0.09_f32);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_eps right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.09`"#)]
fn macro_assert_nearly_eq_eps_panic_f64() {
    assert_nearly_eq!(1.0_f64, 1.1_f64, eps = 0.09_f64);
}

#[test]
fn macro_assert_nearly_eq_ulps_f32() {
    assert_nearly_eq!(0.0_f32, 0.0_f32, ulps = 4_i32);
    assert_nearly_eq!(1.0_f32, 1.0_f32, ulps = 4_i32);
    assert_nearly_eq!(1.0_f32, 1.0000005_f32, ulps = 4_i32);
    assert_nearly_eq!(-1.0_f32, -1.0000005_f32, ulps = 4_i32);
}

#[test]
fn macro_assert_nearly_eq_ulps_f64() {
    assert_nearly_eq!(0.0_f64, 0.0_f64, ulps = 4_i64);
    assert_nearly_eq!(1.0_f64, 1.0_f64, ulps = 4_i64);
    assert_nearly_eq!(1.0_f64, 1.0000000000000009_f64, ulps = 4_i64);
    assert_nearly_eq!(-1.0_f64, -1.0000000000000009_f64, ulps = 4_i64);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_ulps right)`
  left: `1.0`,
 right: `1.001`,
  ulps: `20`"#)]
fn macro_assert_nearly_eq_ulps_panic_f32() {
    assert_nearly_eq!(1.0_f32, 1.001_f32, ulps = 20_i32);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_ulps right)`
  left: `1.0`,
 right: `1.001`,
  ulps: `20`"#)]
fn macro_assert_nearly_eq_ulps_panic_f64() {
    assert_nearly_eq!(1.0_f64, 1.001_f64, ulps = 20_i64);
}

#[test]
fn macro_assert_nearly_eq_tol_f32() {
    assert_nearly_eq!(0.0_f32, 0.0_f32, tol = ToleranceF32::default());
    assert_nearly_eq!(1.0_f32, 1.0_f32, tol = ToleranceF32::default());
    assert_nearly_eq!(1.0_f32, 1.0000005_f32, tol = ToleranceF32::default());
    assert_nearly_eq!(-1.0_f32, -1.0000005_f32, tol = ToleranceF32::default());
}

#[test]
fn macro_assert_nearly_eq_tol_f64() {
    assert_nearly_eq!(0.0_f64, 0.0_f64, tol = ToleranceF64::default());
    assert_nearly_eq!(1.0_f64, 1.0_f64, tol = ToleranceF64::default());
    assert_nearly_eq!(
        1.0_f64,
        1.0000000000000009_f64,
        tol = ToleranceF64::default()
    );
    assert_nearly_eq!(
        -1.0_f64,
        -1.0000000000000009_f64,
        tol = ToleranceF64::default()
    );
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.05`,
  ulps: `11`"#)]
fn macro_assert_nearly_eq_tol_panic_f32() {
    assert_nearly_eq!(1.0_f32, 1.1_f32, tol = ToleranceF32::new(0.05_f32, 11_i32));
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.05`,
  ulps: `11`"#)]
fn macro_assert_nearly_eq_tol_panic_f64() {
    assert_nearly_eq!(1.0_f64, 1.1_f64, tol = ToleranceF64::new(0.05_f64, 11_i64));
}

#[test]
fn macro_assert_nearly_eq_f32() {
    assert_nearly_eq!(0.0_f32, 0.0_f32);
    assert_nearly_eq!(1.0_f32, 1.0_f32);
    assert_nearly_eq!(1.0_f32, 1.0000005_f32);
    assert_nearly_eq!(-1.0_f32, -1.0000005_f32);
}

#[test]
fn macro_assert_nearly_eq_f64() {
    assert_nearly_eq!(0.0_f64, 0.0_f64);
    assert_nearly_eq!(1.0_f64, 1.0_f64);
    assert_nearly_eq!(1.0_f64, 1.0000000000000009_f64);
    assert_nearly_eq!(-1.0_f64, -1.0000000000000009_f64);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq right)`
  left: `1.0`,
 right: `1.1`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_assert_nearly_eq_panic_f32() {
    assert_nearly_eq!(1.0_f32, 1.1_f32);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq right)`
  left: `1.0`,
 right: `1.1`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_assert_nearly_eq_panic_f64() {
    assert_nearly_eq!(1.0_f64, 1.1_f64);
}

////////////////

#[test]
fn macro_assert_nearly_ne_eps_f32() {
    assert_nearly_ne!(0.0_f32, 0.00001_f32, eps = 0.000005_f32);
    assert_nearly_ne!(1.0_f32, -1.0_f32, eps = 0.1_f32);
    assert_nearly_ne!(2.3_f32, 5.9_f32, eps = 2.6_f32);
}

#[test]
fn macro_assert_nearly_ne_eps_f64() {
    assert_nearly_ne!(0.0_f64, 0.00001_f64, eps = 0.000005_f64);
    assert_nearly_ne!(1.0_f64, -1.0_f64, eps = 0.1_f64);
    assert_nearly_ne!(2.3_f64, 5.9_f64, eps = 2.6_f64);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_eps right)`
  left: `1.0`,
 right: `1.001`,
   eps: `0.002`"#)]
fn macro_assert_nearly_ne_eps_panic_f32() {
    assert_nearly_ne!(1.0_f32, 1.001_f32, eps = 0.002_f32);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_eps right)`
  left: `1.0`,
 right: `1.001`,
   eps: `0.002`"#)]
fn macro_assert_nearly_ne_eps_panic_f64() {
    assert_nearly_ne!(1.0_f64, 1.001_f64, eps = 0.002_f64);
}

#[test]
fn macro_assert_nearly_ne_ulps_f32() {
    assert_nearly_ne!(0.0_f32, 0.00001_f32, ulps = 4_i32);
    assert_nearly_ne!(1.0_f32, -1.0_f32, ulps = 100_i32);
    assert_nearly_ne!(2.3_f32, 5.9_f32, ulps = 100_i32);
}

#[test]
fn macro_assert_nearly_ne_ulps_f64() {
    assert_nearly_ne!(0.0_f64, 0.00001_f64, ulps = 4_i64);
    assert_nearly_ne!(1.0_f64, -1.0_f64, ulps = 100_i64);
    assert_nearly_ne!(2.3_f64, 5.9_f64, ulps = 100_i64);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_ulps right)`
  left: `1.0`,
 right: `1.001`,
  ulps: `10000`"#)]
fn macro_assert_nearly_ne_ulps_panic_f32() {
    assert_nearly_ne!(1.0_f32, 1.001_f32, ulps = 10000_i32);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_ulps right)`
  left: `1.0`,
 right: `1.0000000000001`,
  ulps: `10000`"#)]
fn macro_assert_nearly_ne_ulps_panic_f64() {
    assert_nearly_ne!(1.0_f64, 1.0000000000001_f64, ulps = 10000_i64);
}

#[test]
fn macro_assert_nearly_ne_tol_f32() {
    assert_nearly_ne!(0.0_f32, 0.00001_f32, tol = ToleranceF32::default());
    assert_nearly_ne!(1.0_f32, -1.0_f32, tol = ToleranceF32::new(0.1_f32, 100_i32));
    assert_nearly_ne!(2.3_f32, 5.9_f32, tol = ToleranceF32::new(2.6_f32, 100_i32));
}

#[test]
fn macro_assert_nearly_ne_tol_f64() {
    assert_nearly_ne!(0.0_f64, 0.00001_f64, tol = ToleranceF64::default());
    assert_nearly_ne!(1.0_f64, -1.0_f64, tol = ToleranceF64::new(0.1_f64, 100_i64));
    assert_nearly_ne!(2.3_f64, 5.9_f64, tol = ToleranceF64::new(2.6_f64, 100_i64));
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`
  left: `1.0`,
 right: `1.001`,
   eps: `0.002`,
  ulps: `10000`"#)]
fn macro_assert_nearly_ne_tol_panic_f32() {
    assert_nearly_ne!(
        1.0_f32,
        1.001_f32,
        tol = ToleranceF32::new(0.002_f32, 10000_i32)
    );
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`
  left: `1.0`,
 right: `1.0000000000001`,
   eps: `0.002`,
  ulps: `10000`"#)]
fn macro_assert_nearly_ne_tol_panic_f64() {
    assert_nearly_ne!(
        1.0_f64,
        1.0000000000001_f64,
        tol = ToleranceF64::new(0.002_f64, 10000_i64)
    );
}

#[test]
fn macro_assert_nearly_ne_f32() {
    assert_nearly_ne!(0.0_f32, 0.00001_f32);
    assert_nearly_ne!(1.0_f32, -1.0_f32);
    assert_nearly_ne!(2.3_f32, 5.9_f32);
}

#[test]
fn macro_assert_nearly_ne_f64() {
    assert_nearly_ne!(0.0_f64, 0.00001_f64);
    assert_nearly_ne!(1.0_f64, -1.0_f64);
    assert_nearly_ne!(2.3_f64, 5.9_f64);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne right)`
  left: `1.0`,
 right: `1.0000005`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_assert_nearly_ne_panic_f32() {
    assert_nearly_ne!(1.0_f32, 1.0000005_f32);
}

#[test]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne right)`
  left: `1.0`,
 right: `1.0000000000000009`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_assert_nearly_ne_panic_f64() {
    assert_nearly_ne!(1.0_f64, 1.0000000000000009_f64);
}
