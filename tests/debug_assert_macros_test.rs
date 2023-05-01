use nearly::{debug_assert_nearly_eq, debug_assert_nearly_ne};
use nearly::{ToleranceF32, ToleranceF64};

/////////////////////////////
// debug_assert_nearly_eq! //
/////////////////////////////

#[test]
fn macro_debug_assert_nearly_eq_eps_f32() {
    debug_assert_nearly_eq!(0.0_f32, 0.0_f32, eps = 0.0_f32);
    debug_assert_nearly_eq!(1.0_f32, 1.0_f32, eps = 0.01_f32);
    debug_assert_nearly_eq!(1.0_f32, 1.000003, eps = 0.000004_f32);
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32, eps = 1.11_f32);
}

#[test]
fn macro_debug_assert_nearly_eq_eps_f64() {
    debug_assert_nearly_eq!(0.0_f64, 0.0_f64, eps = 0.0_f64);
    debug_assert_nearly_eq!(1.0_f64, 1.0_f64, eps = 0.01_f64);
    debug_assert_nearly_eq!(1.0_f64, 1.0000000000003_f64, eps = 0.0000000000004_f64);
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64, eps = 1.11_f64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_eps right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.09`"#)]
fn macro_debug_assert_nearly_eq_eps_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32, eps = 0.09_f32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_eps_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32, eps = 0.09_f32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_eps right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.09`"#)]
fn macro_debug_assert_nearly_eq_eps_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64, eps = 0.09_f64);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_eps_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64, eps = 0.09_f64);
}

#[test]
fn macro_debug_assert_nearly_eq_ulps_f32() {
    debug_assert_nearly_eq!(0.0_f32, 0.0_f32, ulps = 4_i32);
    debug_assert_nearly_eq!(1.0_f32, 1.0_f32, ulps = 4_i32);
    debug_assert_nearly_eq!(1.0_f32, 1.0000005_f32, ulps = 4_i32);
    debug_assert_nearly_eq!(-1.0_f32, -1.0000005_f32, ulps = 4_i32);
}

#[test]
fn macro_debug_assert_nearly_eq_ulps_f64() {
    debug_assert_nearly_eq!(0.0_f64, 0.0_f64, ulps = 4_i64);
    debug_assert_nearly_eq!(1.0_f64, 1.0_f64, ulps = 4_i64);
    debug_assert_nearly_eq!(1.0_f64, 1.0000000000000009_f64, ulps = 4_i64);
    debug_assert_nearly_eq!(-1.0_f64, -1.0000000000000009_f64, ulps = 4_i64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_ulps right)`
  left: `1.0`,
 right: `1.001`,
  ulps: `20`"#)]
fn macro_debug_assert_nearly_eq_ulps_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.001_f32, ulps = 20_i32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_ulps_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.001_f32, ulps = 20_i32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_ulps right)`
  left: `1.0`,
 right: `1.001`,
  ulps: `20`"#)]
fn macro_debug_assert_nearly_eq_ulps_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.001_f64, ulps = 20_i64);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_ulps_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.001_f64, ulps = 20_i64);
}

#[test]
fn macro_debug_assert_nearly_eq_tol_f32() {
    debug_assert_nearly_eq!(0.0_f32, 0.0_f32, tol = ToleranceF32::default());
    debug_assert_nearly_eq!(1.0_f32, 1.0_f32, tol = ToleranceF32::default());
    debug_assert_nearly_eq!(1.0_f32, 1.0000005_f32, tol = ToleranceF32::default());
    debug_assert_nearly_eq!(-1.0_f32, -1.0000005_f32, tol = ToleranceF32::default());
}

#[test]
fn macro_debug_assert_nearly_eq_tol_f64() {
    debug_assert_nearly_eq!(0.0_f64, 0.0_f64, tol = ToleranceF64::default());
    debug_assert_nearly_eq!(1.0_f64, 1.0_f64, tol = ToleranceF64::default());
    debug_assert_nearly_eq!(
        1.0_f64,
        1.0000000000000009_f64,
        tol = ToleranceF64::default()
    );
    debug_assert_nearly_eq!(
        -1.0_f64,
        -1.0000000000000009_f64,
        tol = ToleranceF64::default()
    );
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.05`,
  ulps: `11`"#)]
fn macro_debug_assert_nearly_eq_tol_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32, tol = ToleranceF32::new(0.05_f32, 11_i32));
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_tol_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32, tol = ToleranceF32::new(0.05_f32, 11_i32));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.05`,
  ulps: `11`"#)]
fn macro_debug_assert_nearly_eq_tol_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64, tol = ToleranceF64::new(0.05_f64, 11_i64));
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_tol_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64, tol = ToleranceF64::new(0.05_f64, 11_i64));
}

#[test]
fn macro_debug_assert_nearly_eq_eps_ulps_f32() {
    let default_eps = ToleranceF32::default().eps;
    let default_ulps = ToleranceF32::default().ulps;

    debug_assert_nearly_eq!(0.0_f32, 0.0_f32, eps = default_eps, ulps = default_ulps);
    debug_assert_nearly_eq!(1.0_f32, 1.0_f32, eps = default_eps, ulps = default_ulps);
    debug_assert_nearly_eq!(
        1.0_f32,
        1.0000005_f32,
        eps = default_eps,
        ulps = default_ulps
    );
    debug_assert_nearly_eq!(
        -1.0_f32,
        -1.0000005_f32,
        eps = default_eps,
        ulps = default_ulps
    );
}

#[test]
fn macro_debug_assert_nearly_eq_eps_ulps_f64() {
    let default_eps = ToleranceF64::default().eps;
    let default_ulps = ToleranceF64::default().ulps;

    debug_assert_nearly_eq!(0.0_f64, 0.0_f64, eps = default_eps, ulps = default_ulps);
    debug_assert_nearly_eq!(1.0_f64, 1.0_f64, eps = default_eps, ulps = default_ulps);
    debug_assert_nearly_eq!(
        1.0_f64,
        1.0000000000000009_f64,
        eps = default_eps,
        ulps = default_ulps
    );
    debug_assert_nearly_eq!(
        -1.0_f64,
        -1.0000000000000009_f64,
        eps = default_eps,
        ulps = default_ulps
    );
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.05`,
  ulps: `11`"#)]
fn macro_debug_assert_nearly_eq_eps_ulps_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32, eps = 0.05_f32, ulps = 11_i32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_eps_ulps_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32, eps = 0.05_f32, ulps = 11_i32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`
  left: `1.0`,
 right: `1.1`,
   eps: `0.05`,
  ulps: `11`"#)]
fn macro_debug_assert_nearly_eq_eps_ulps_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64, eps = 0.05_f64, ulps = 11_i64);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_eps_ulps_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64, eps = 0.05_f64, ulps = 11_i64);
}

#[test]
fn macro_debug_assert_nearly_eq_f32() {
    debug_assert_nearly_eq!(0.0_f32, 0.0_f32);
    debug_assert_nearly_eq!(1.0_f32, 1.0_f32);
    debug_assert_nearly_eq!(1.0_f32, 1.0000005_f32);
    debug_assert_nearly_eq!(-1.0_f32, -1.0000005_f32);
}

#[test]
fn macro_debug_assert_nearly_eq_f64() {
    debug_assert_nearly_eq!(0.0_f64, 0.0_f64);
    debug_assert_nearly_eq!(1.0_f64, 1.0_f64);
    debug_assert_nearly_eq!(1.0_f64, 1.0000000000000009_f64);
    debug_assert_nearly_eq!(-1.0_f64, -1.0000000000000009_f64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq right)`
  left: `1.0`,
 right: `1.1`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_debug_assert_nearly_eq_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_panic_f32() {
    debug_assert_nearly_eq!(1.0_f32, 1.1_f32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_eq right)`
  left: `1.0`,
 right: `1.1`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_debug_assert_nearly_eq_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_panic_f64() {
    debug_assert_nearly_eq!(1.0_f64, 1.1_f64);
}

/////////////////////////////
// debug_assert_nearly_ne! //
/////////////////////////////

#[test]
fn macro_debug_assert_nearly_ne_eps_f32() {
    debug_assert_nearly_ne!(0.0_f32, 0.00001_f32, eps = 0.000005_f32);
    debug_assert_nearly_ne!(1.0_f32, -1.0_f32, eps = 0.1_f32);
    debug_assert_nearly_ne!(2.3_f32, 5.9_f32, eps = 2.6_f32);
}

#[test]
fn macro_debug_assert_nearly_ne_eps_f64() {
    debug_assert_nearly_ne!(0.0_f64, 0.00001_f64, eps = 0.000005_f64);
    debug_assert_nearly_ne!(1.0_f64, -1.0_f64, eps = 0.1_f64);
    debug_assert_nearly_ne!(2.3_f64, 5.9_f64, eps = 2.6_f64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_eps right)`
  left: `1.0`,
 right: `1.001`,
   eps: `0.002`"#)]
fn macro_debug_assert_nearly_ne_eps_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.001_f32, eps = 0.002_f32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_eps_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.001_f32, eps = 0.002_f32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_eps right)`
  left: `1.0`,
 right: `1.001`,
   eps: `0.002`"#)]
fn macro_debug_assert_nearly_ne_eps_panic_f64() {
    debug_assert_nearly_ne!(1.0_f64, 1.001_f64, eps = 0.002_f64);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_eps_panic_f64() {
    debug_assert_nearly_ne!(1.0_f64, 1.001_f64, eps = 0.002_f64);
}

#[test]
fn macro_debug_assert_nearly_ne_ulps_f32() {
    debug_assert_nearly_ne!(0.0_f32, 0.00001_f32, ulps = 4_i32);
    debug_assert_nearly_ne!(1.0_f32, -1.0_f32, ulps = 100_i32);
    debug_assert_nearly_ne!(2.3_f32, 5.9_f32, ulps = 100_i32);
}

#[test]
fn macro_debug_assert_nearly_ne_ulps_f64() {
    debug_assert_nearly_ne!(0.0_f64, 0.00001_f64, ulps = 4_i64);
    debug_assert_nearly_ne!(1.0_f64, -1.0_f64, ulps = 100_i64);
    debug_assert_nearly_ne!(2.3_f64, 5.9_f64, ulps = 100_i64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_ulps right)`
  left: `1.0`,
 right: `1.001`,
  ulps: `10000`"#)]
fn macro_debug_assert_nearly_ne_ulps_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.001_f32, ulps = 10000_i32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_ulps_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.001_f32, ulps = 10000_i32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_ulps right)`
  left: `1.0`,
 right: `1.0000000000001`,
  ulps: `10000`"#)]
fn macro_debug_assert_nearly_ne_ulps_panic_f64() {
    debug_assert_nearly_ne!(1.0_f64, 1.0000000000001_f64, ulps = 10000_i64);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_ulps_panic_f64() {
    debug_assert_nearly_ne!(1.0_f64, 1.0000000000001_f64, ulps = 10000_i64);
}

#[test]
fn macro_debug_assert_nearly_ne_tol_f32() {
    debug_assert_nearly_ne!(0.0_f32, 0.00001_f32, tol = ToleranceF32::default());
    debug_assert_nearly_ne!(1.0_f32, -1.0_f32, tol = ToleranceF32::new(0.1_f32, 100_i32));
    debug_assert_nearly_ne!(2.3_f32, 5.9_f32, tol = ToleranceF32::new(2.6_f32, 100_i32));
}

#[test]
fn macro_debug_assert_nearly_ne_tol_f64() {
    debug_assert_nearly_ne!(0.0_f64, 0.00001_f64, tol = ToleranceF64::default());
    debug_assert_nearly_ne!(1.0_f64, -1.0_f64, tol = ToleranceF64::new(0.1_f64, 100_i64));
    debug_assert_nearly_ne!(2.3_f64, 5.9_f64, tol = ToleranceF64::new(2.6_f64, 100_i64));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`
  left: `1.0`,
 right: `1.001`,
   eps: `0.002`,
  ulps: `10000`"#)]
fn macro_debug_assert_nearly_ne_tol_panic_f32() {
    debug_assert_nearly_ne!(
        1.0_f32,
        1.001_f32,
        tol = ToleranceF32::new(0.002_f32, 10000_i32)
    );
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_tol_panic_f32() {
    debug_assert_nearly_ne!(
        1.0_f32,
        1.001_f32,
        tol = ToleranceF32::new(0.002_f32, 10000_i32)
    );
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`
  left: `1.0`,
 right: `1.0000000000001`,
   eps: `0.002`,
  ulps: `10000`"#)]
fn macro_debug_assert_nearly_ne_tol_panic_f64() {
    debug_assert_nearly_ne!(
        1.0_f64,
        1.0000000000001_f64,
        tol = ToleranceF64::new(0.002_f64, 10000_i64)
    );
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_tol_panic_f64() {
    debug_assert_nearly_ne!(
        1.0_f64,
        1.0000000000001_f64,
        tol = ToleranceF64::new(0.002_f64, 10000_i64)
    );
}

#[test]
fn macro_debug_assert_nearly_ne_eps_ulps_f32() {
    debug_assert_nearly_ne!(
        0.0_f32,
        0.00001_f32,
        eps = ToleranceF32::default().eps,
        ulps = ToleranceF32::default().ulps
    );
    debug_assert_nearly_ne!(1.0_f32, -1.0_f32, eps = 0.1_f32, ulps = 100_i32);
    debug_assert_nearly_ne!(2.3_f32, 5.9_f32, eps = 2.6_f32, ulps = 100_i32);
}

#[test]
fn macro_debug_assert_nearly_ne_eps_ulps_f64() {
    debug_assert_nearly_ne!(
        0.0_f64,
        0.00001_f64,
        eps = ToleranceF64::default().eps,
        ulps = ToleranceF64::default().ulps
    );
    debug_assert_nearly_ne!(1.0_f64, -1.0_f64, eps = 0.1_f64, ulps = 100_i64);
    debug_assert_nearly_ne!(2.3_f64, 5.9_f64, eps = 2.6_f64, ulps = 100_i64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`
  left: `1.0`,
 right: `1.001`,
   eps: `0.002`,
  ulps: `10000`"#)]
fn macro_debug_assert_nearly_ne_eps_ulps_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.001_f32, eps = 0.002_f32, ulps = 10000_i32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_eps_ulps_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.001_f32, eps = 0.002_f32, ulps = 10000_i32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`
  left: `1.0`,
 right: `1.0000000000001`,
   eps: `0.002`,
  ulps: `10000`"#)]
fn macro_debug_assert_nearly_ne_eps_ulps_panic_f64() {
    debug_assert_nearly_ne!(
        1.0_f64,
        1.0000000000001_f64,
        eps = 0.002_f64,
        ulps = 10000_i64
    );
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_eps_ulps_panic_f64() {
    debug_assert_nearly_ne!(
        1.0_f64,
        1.0000000000001_f64,
        eps = 0.002_f64,
        ulps = 10000_i64
    );
}

#[test]
fn macro_debug_assert_nearly_ne_f32() {
    debug_assert_nearly_ne!(0.0_f32, 0.00001_f32);
    debug_assert_nearly_ne!(1.0_f32, -1.0_f32);
    debug_assert_nearly_ne!(2.3_f32, 5.9_f32);
}

#[test]
fn macro_debug_assert_nearly_ne_f64() {
    debug_assert_nearly_ne!(0.0_f64, 0.00001_f64);
    debug_assert_nearly_ne!(1.0_f64, -1.0_f64);
    debug_assert_nearly_ne!(2.3_f64, 5.9_f64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne right)`
  left: `1.0`,
 right: `1.0000005`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_debug_assert_nearly_ne_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.0000005_f32);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_panic_f32() {
    debug_assert_nearly_ne!(1.0_f32, 1.0000005_f32);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `(left nearly_ne right)`
  left: `1.0`,
 right: `1.0000000000000009`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#)]
fn macro_debug_assert_nearly_ne_panic_f64() {
    debug_assert_nearly_ne!(1.0_f64, 1.0000000000000009_f64);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_panic_f64() {
    debug_assert_nearly_ne!(1.0_f64, 1.0000000000000009_f64);
}
