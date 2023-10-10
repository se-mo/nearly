use mockall::predicate::eq;
use mockall::Sequence;
use nearly::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance};

mod common;
use common::{MockLhs, Rhs};

#[test]
fn nearly_ne_eps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(false);
    assert!(a.nearly_ne_eps(&b, 0.1));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);
    assert!(!a.nearly_ne_eps(&b, 0.1));
}

#[test]
fn nearly_ne_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(false);
    assert!(a.nearly_ne_ulps(&b, 5));

    a.checkpoint();

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .return_const(true);
    assert!(!a.nearly_ne_ulps(&b, 5));
}

#[test]
fn nearly_eq_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);
    a.expect_nearly_eq_ulps().times(0);
    assert!(a.nearly_eq_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_eq_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_eq_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
}

#[test]
fn nearly_ne_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .return_const(true);
    a.expect_nearly_eq_ulps().times(0);
    assert!(!a.nearly_ne_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(!a.nearly_ne_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(a.nearly_ne_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
}

#[test]
fn nearly_eq() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .return_const(true);
    a.expect_nearly_eq_ulps().times(0);
    assert!(a.nearly_eq(&b));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_eq(&b));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_eq(&b));
}

#[test]
fn nearly_ne() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .return_const(true);
    a.expect_nearly_eq_ulps().times(0);
    assert!(!a.nearly_ne(&b));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(!a.nearly_ne(&b));

    a.checkpoint();

    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(a.nearly_ne(&b));
}
