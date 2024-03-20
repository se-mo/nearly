use mockall::predicate::eq;
use nearly::{assert_nearly, debug_assert_nearly, nearly, Tolerance};

mod common;
use common::{MockLhs, Rhs};

/////////////
// nearly! //
/////////////

#[test]
fn macro_nearly_lt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);
    assert!(nearly!(a < b, eps = 0.1));

    a.checkpoint();

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a < b, eps = 0.1));
}

#[test]
fn macro_nearly_le_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);
    assert!(nearly!(a <= b, eps = 0.1));

    a.checkpoint();

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a <= b, eps = 0.1));
}

#[test]
fn macro_nearly_gt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);
    assert!(nearly!(a > b, eps = 0.1));

    a.checkpoint();

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a > b, eps = 0.1));
}

#[test]
fn macro_nearly_ge_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);
    assert!(nearly!(a >= b, eps = 0.1));

    a.checkpoint();

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a >= b, eps = 0.1));
}

#[test]
fn macro_nearly_lt_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);
    assert!(nearly!(a < b, ulps = 5));

    a.checkpoint();

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a < b, ulps = 5));
}

#[test]
fn macro_nearly_le_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);
    assert!(nearly!(a <= b, ulps = 5));

    a.checkpoint();

    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a <= b, ulps = 5));
}

#[test]
fn macro_nearly_gt_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);
    assert!(nearly!(a > b, ulps = 5));

    a.checkpoint();

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a > b, ulps = 5));
}

#[test]
fn macro_nearly_ge_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);
    assert!(nearly!(a >= b, ulps = 5));

    a.checkpoint();

    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a >= b, ulps = 5));
}

#[test]
fn macro_nearly_lt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a < b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a < b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a < b, eps = 0.15, ulps = 7));

    a.checkpoint();

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a < b, eps = 0.15, ulps = 7));
}

#[test]
fn macro_nearly_le_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a <= b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a <= b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a <= b, eps = 0.15, ulps = 7));

    a.checkpoint();

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a <= b, eps = 0.15, ulps = 7));
}

#[test]
fn macro_nearly_gt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a > b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a > b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a > b, eps = 0.15, ulps = 7));

    a.checkpoint();

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a > b, eps = 0.15, ulps = 7));
}

#[test]
fn macro_nearly_ge_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a >= b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a >= b, tol = Tolerance::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a >= b, eps = 0.15, ulps = 7));

    a.checkpoint();

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a >= b, eps = 0.15, ulps = 7));
}

#[test]
fn macro_nearly_lt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a < b));

    a.checkpoint();

    a.expect_nearly_lt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a < b));
}

#[test]
fn macro_nearly_le() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a <= b));

    a.checkpoint();

    a.expect_nearly_le()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a <= b));
}

#[test]
fn macro_nearly_gt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a > b));

    a.checkpoint();

    a.expect_nearly_gt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a > b));
}

#[test]
fn macro_nearly_ge() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);
    assert!(nearly!(a >= b));

    a.checkpoint();

    a.expect_nearly_ge()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);
    assert!(!nearly!(a >= b));
}

////////////////////
// assert_nearly! //
////////////////////

#[test]
fn macro_assert_nearly_lt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert_nearly!(a < b, eps = 0.1);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_assert_nearly_lt_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    assert_nearly!(a < b, eps = 0.1);
}

#[test]
fn macro_assert_nearly_le_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert_nearly!(a <= b, eps = 0.1);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_assert_nearly_le_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    assert_nearly!(a <= b, eps = 0.1);
}

#[test]
fn macro_assert_nearly_gt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert_nearly!(a > b, eps = 0.1);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_assert_nearly_gt_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    assert_nearly!(a > b, eps = 0.1);
}

#[test]
fn macro_assert_nearly_ge_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    assert_nearly!(a >= b, eps = 0.1);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_assert_nearly_ge_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    assert_nearly!(a >= b, eps = 0.1);
}

#[test]
fn macro_assert_nearly_lt_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert_nearly!(a < b, ulps = 5);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_assert_nearly_lt_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    assert_nearly!(a < b, ulps = 5);
}

#[test]
fn macro_assert_nearly_le_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert_nearly!(a <= b, ulps = 5);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_assert_nearly_le_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    assert_nearly!(a <= b, ulps = 5);
}

#[test]
fn macro_assert_nearly_gt_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert_nearly!(a > b, ulps = 5);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_assert_nearly_gt_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    assert_nearly!(a > b, ulps = 5);
}

#[test]
fn macro_assert_nearly_ge_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    assert_nearly!(a >= b, ulps = 5);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_assert_nearly_ge_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    assert_nearly!(a >= b, ulps = 5);
}

#[test]
fn macro_assert_nearly_lt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert_nearly!(a < b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert_nearly!(a < b, eps = 0.15, ulps = 7);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_assert_nearly_lt_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a < b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_assert_nearly_lt_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    assert_nearly!(a < b, eps = 0.15, ulps = 7);
}

#[test]
fn macro_assert_nearly_le_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert_nearly!(a <= b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert_nearly!(a <= b, eps = 0.15, ulps = 7);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_assert_nearly_le_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a <= b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_assert_nearly_le_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    assert_nearly!(a <= b, eps = 0.15, ulps = 7);
}

#[test]
fn macro_assert_nearly_gt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert_nearly!(a > b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert_nearly!(a > b, eps = 0.15, ulps = 7);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_assert_nearly_gt_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a > b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_assert_nearly_gt_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    assert_nearly!(a > b, eps = 0.15, ulps = 7);
}

#[test]
fn macro_assert_nearly_ge_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    assert_nearly!(a >= b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    assert_nearly!(a >= b, eps = 0.15, ulps = 7);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_assert_nearly_ge_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a >= b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_assert_nearly_ge_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    assert_nearly!(a >= b, eps = 0.15, ulps = 7);
}

#[test]
fn macro_assert_nearly_lt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert_nearly!(a < b);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_assert_nearly_lt_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a < b);
}

#[test]
fn macro_assert_nearly_le() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert_nearly!(a <= b);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_assert_nearly_le_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a <= b);
}

#[test]
fn macro_assert_nearly_gt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert_nearly!(a > b);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_assert_nearly_gt_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a > b);
}

#[test]
fn macro_assert_nearly_ge() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    assert_nearly!(a >= b);
}

#[test]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_assert_nearly_ge_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    assert_nearly!(a >= b);
}

//////////////////////////
// debug_assert_nearly! //
//////////////////////////

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_lt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a < b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_debug_assert_nearly_lt_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a < b, eps = 0.1);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_lt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps().times(0);
    debug_assert_nearly!(a < b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_le_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a <= b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_debug_assert_nearly_le_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a <= b, eps = 0.1);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_le_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps().times(0);
    debug_assert_nearly!(a <= b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_gt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a > b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_debug_assert_nearly_gt_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a > b, eps = 0.1);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_gt_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps().times(0);
    debug_assert_nearly!(a > b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ge_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a >= b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1"#)]
fn macro_debug_assert_nearly_ge_eps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a >= b, eps = 0.1);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ge_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps().times(0);
    debug_assert_nearly!(a >= b, eps = 0.1);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_lt_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a < b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_debug_assert_nearly_lt_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a < b, ulps = 5);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_lt_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_ulps().times(0);
    debug_assert_nearly!(a < b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_le_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a <= b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_debug_assert_nearly_le_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a <= b, ulps = 5);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_le_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_ulps().times(0);
    debug_assert_nearly!(a <= b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_et_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a > b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_debug_assert_nearly_gt_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a > b, ulps = 5);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_gt_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_ulps().times(0);
    debug_assert_nearly!(a > b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ge_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a >= b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
  ulps: 5"#)]
fn macro_debug_assert_nearly_ge_ulps_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a >= b, ulps = 5);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ge_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_ulps().times(0);
    debug_assert_nearly!(a >= b, ulps = 5);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_lt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a < b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a < b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_debug_assert_nearly_lt_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a < b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_debug_assert_nearly_lt_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a < b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_lt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_tol().times(0);
    debug_assert_nearly!(a < b, tol = Tolerance::new(0.1, 5));
    debug_assert_nearly!(a < b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_le_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a <= b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a <= b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_debug_assert_nearly_le_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a <= b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_debug_assert_nearly_le_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a <= b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_le_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_tol().times(0);
    debug_assert_nearly!(a <= b, tol = Tolerance::new(0.1, 5));
    debug_assert_nearly!(a <= b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_gt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a > b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a > b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_debug_assert_nearly_gt_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a > b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_debug_assert_nearly_gt_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a > b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_gt_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_tol().times(0);
    debug_assert_nearly!(a > b, tol = Tolerance::new(0.1, 5));
    debug_assert_nearly!(a > b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ge_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a >= b, tol = Tolerance::new(0.1, 5));

    a.checkpoint();

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(true);
    debug_assert_nearly!(a >= b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.1
  ulps: 5"#)]
fn macro_debug_assert_nearly_ge_tol_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a >= b, tol = Tolerance::new(0.1, 5));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.15
  ulps: 7"#)]
fn macro_debug_assert_nearly_ge_tol_tuple_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol()
        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.15, 7)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a >= b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ge_tol() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_tol().times(0);
    debug_assert_nearly!(a >= b, tol = Tolerance::new(0.1, 5));
    debug_assert_nearly!(a >= b, eps = 0.15, ulps = 7);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_lt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a < b);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left < right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_debug_assert_nearly_lt_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a < b);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_lt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt().times(0);
    debug_assert_nearly!(a < b);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_le() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a <= b);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left <= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_debug_assert_nearly_le_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a <= b);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_le() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le().times(0);
    debug_assert_nearly!(a <= b);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_gt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a > b);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left > right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_debug_assert_nearly_gt_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a > b);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_gt() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt().times(0);
    debug_assert_nearly!(a > b);
}

#[test]
#[cfg(debug_assertions)]
fn macro_debug_assert_nearly_ge() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(true);

    debug_assert_nearly!(a >= b);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = r#"assertion `nearly (left >= right)` failed
  left: MockLhs
 right: Rhs(5)
   eps: 0.01
  ulps: 3"#)]
fn macro_debug_assert_nearly_ge_panic() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge()
        .with(eq(Rhs(5)))
        .times(1)
        .return_const(false);

    debug_assert_nearly!(a >= b);
}

#[test]
#[cfg(not(debug_assertions))]
fn macro_debug_assert_nearly_ge() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge().times(0);
    debug_assert_nearly!(a >= b);
}
