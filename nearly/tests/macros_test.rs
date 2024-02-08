use mockall::predicate::eq;
use mockall::{mock, Sequence};
use nearly::{
    assert_nearly, debug_assert_nearly, nearly, EpsTolerance, EpsToleranceType, NearlyEq,
    NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance, UlpsTolerance, UlpsToleranceType,
};

#[derive(Debug, PartialEq)]
struct Rhs(i32);

mock!(
    #[derive(Debug)]
    Lhs{}

    impl EpsTolerance<Rhs> for Lhs {
        type T = f32;
        const DEFAULT: f32 = 0.01;
    }

    impl UlpsTolerance<Rhs> for Lhs {
        type T = i32;
        const DEFAULT: i32 = 3;
    }

    impl NearlyEqEps<Rhs> for Lhs {
        fn nearly_eq_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_ne_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyEqUlps<Rhs> for Lhs {
        fn nearly_eq_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_ne_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyEqTol<Rhs> for Lhs {
        fn nearly_eq_tol(&self, other: &Rhs, tol: &Tolerance<Self, Rhs>) -> bool;
        fn nearly_ne_tol(&self, other: &Rhs, tol: &Tolerance<Self, Rhs>) -> bool;
    }

    impl NearlyEq<Rhs> for Lhs {
        fn nearly_eq(&self, other: &Rhs) -> bool;
        fn nearly_ne(&self, other: &Rhs) -> bool;
    }
);

/////////////
// nearly! //
/////////////

#[test]
fn macro_nearly_eq_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert!(nearly!(a == b, eps = 0.1));
}

#[test]
fn macro_nearly_eq_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert!(nearly!(a == b, ulps = 5));
}

#[test]
fn macro_nearly_eq_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    assert!(nearly!(
        a == b,
        tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5)
    ));
    assert!(nearly!(a == b, eps = 0.15, ulps = 7));
}

#[test]
fn macro_nearly_eq() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert!(nearly!(a == b));
}

#[test]
fn macro_nearly_ne_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert!(nearly!(a != b, eps = 0.1));
}

#[test]
fn macro_nearly_ne_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert!(nearly!(a != b, ulps = 5));
}

#[test]
fn macro_nearly_ne_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    assert!(nearly!(
        a != b,
        tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5)
    ));
    assert!(nearly!(a != b, eps = 0.15, ulps = 7));
}

#[test]
fn macro_nearly_ne() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert!(nearly!(a != b));
}

////////////////////
// assert_nearly! //
////////////////////

#[test]
fn macro_assert_nearly_eq_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert_nearly!(a == b, eps = 0.1);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`"#)]
fn macro_assert_nearly_eq_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    assert_nearly!(a == b, eps = 0.1);
}

#[test]
fn macro_assert_nearly_eq_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert_nearly!(a == b, ulps = 5);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
  ulps: `5`"#)]
fn macro_assert_nearly_eq_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    assert_nearly!(a == b, ulps = 5);
}

#[test]
fn macro_assert_nearly_eq_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    assert_nearly!(a == b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
    assert_nearly!(a == b, eps = 0.15, ulps = 7);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`,
  ulps: `5`"#)]
fn macro_assert_nearly_eq_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a == b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.15`,
  ulps: `7`"#)]
fn macro_assert_nearly_eq_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    assert_nearly!(a == b, eps = 0.15, ulps = 7);
}

#[test]
fn macro_assert_nearly_eq() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert_nearly!(a == b);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.01`,
  ulps: `3`"#)]
fn macro_assert_nearly_eq_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a == b);
}

#[test]
fn macro_assert_nearly_ne_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert_nearly!(a != b, eps = 0.1);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`"#)]
fn macro_assert_nearly_ne_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    assert_nearly!(a != b, eps = 0.1);
}

#[test]
fn macro_assert_nearly_ne_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert_nearly!(a != b, ulps = 5);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
  ulps: `5`"#)]
fn macro_assert_nearly_ne_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    assert_nearly!(a != b, ulps = 5);
}

#[test]
fn macro_assert_nearly_ne_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    assert_nearly!(a != b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
    assert_nearly!(a != b, eps = 0.15, ulps = 7);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`,
  ulps: `5`"#)]
fn macro_assert_nearly_ne_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a != b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.15`,
  ulps: `7`"#)]
fn macro_assert_nearly_ne_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    assert_nearly!(a != b, eps = 0.15, ulps = 7);
}

#[test]
fn macro_assert_nearly_ne() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert_nearly!(a != b);
}

#[test]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.01`,
  ulps: `3`"#)]
fn macro_assert_nearly_ne_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a != b);
}

//////////////////////////
// debug_assert_nearly! //
//////////////////////////

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_eq_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a == b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`"#)]
fn macro_debug_assert_nearly_eq_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a == b, eps = 0.1);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps().times(0);
    debug_assert_nearly!(a == b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_eq_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a == b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
  ulps: `5`"#)]
fn macro_debug_assert_nearly_eq_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a == b, ulps = 5);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_ulps().times(0);
    debug_assert_nearly!(a == b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_eq_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    debug_assert_nearly!(a == b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
    debug_assert_nearly!(a == b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`,
  ulps: `5`"#)]
fn macro_debug_assert_nearly_eq_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a == b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.15`,
  ulps: `7`"#)]
fn macro_debug_assert_nearly_eq_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a == b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_tol().times(0);

    debug_assert_nearly!(a == b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
    debug_assert_nearly!(a == b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_eq() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a == b);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left == right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.01`,
  ulps: `3`"#)]
fn macro_debug_assert_nearly_eq_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a == b);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_eq() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq().times(0);
    debug_assert_nearly!(a == b);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ne_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a != b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`"#)]
fn macro_debug_assert_nearly_ne_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a != b, eps = 0.1);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_eps().times(0);
    debug_assert_nearly!(a != b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ne_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a != b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
  ulps: `5`"#)]
fn macro_debug_assert_nearly_ne_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a != b, ulps = 5);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_ulps().times(0);
    debug_assert_nearly!(a != b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ne_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    debug_assert_nearly!(a != b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
    debug_assert_nearly!(a != b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.1`,
  ulps: `5`"#)]
fn macro_debug_assert_nearly_ne_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a != b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.15`,
  ulps: `7`"#)]
fn macro_debug_assert_nearly_ne_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a != b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne_tol().times(0);

    debug_assert_nearly!(a != b, tol = Tolerance::<MockLhs, Rhs>::new(0.1, 5));
    debug_assert_nearly!(a != b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ne() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a != b);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion failed: `nearly (left != right)`
  left: `MockLhs`,
 right: `Rhs(5)`,
   eps: `0.01`,
  ulps: `3`"#)]
fn macro_debug_assert_nearly_ne_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a != b);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ne() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ne().times(0);
    debug_assert_nearly!(a != b);
}
