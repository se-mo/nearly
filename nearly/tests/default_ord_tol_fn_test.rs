use mockall::{mock, predicate::eq, Sequence};
use nearly::{
    EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, NearlyOrd,
    NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps, Tolerance, UlpsTolerance, UlpsToleranceType,
};

#[derive(Debug, PartialEq)]
pub struct Rhs(pub i32);

mock!(
    pub Lhs{}

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
    }

    impl NearlyEqUlps<Rhs> for Lhs {
        fn nearly_eq_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyEqTol<Rhs> for Lhs {}

    impl NearlyEq<Rhs> for Lhs {}

    impl NearlyOrdEps<Rhs> for Lhs {
        fn nearly_lt_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_le_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_gt_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_ge_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyOrdUlps<Rhs> for Lhs {
        fn nearly_lt_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_le_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_gt_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_ge_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyOrdTol<Rhs> for Lhs {}

    impl NearlyOrd<Rhs> for Lhs {}
);

#[test]
fn nearly_lt_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_lt_ulps().times(0);
    assert!(a.nearly_lt_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_lt_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_lt_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
}

#[test]
fn nearly_le_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_le_ulps().times(0);
    assert!(a.nearly_le_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_le_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_le_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
}

#[test]
fn nearly_gt_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_gt_ulps().times(0);
    assert!(a.nearly_gt_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_gt_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_gt_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
}

#[test]
fn nearly_ge_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_ge_ulps().times(0);
    assert!(a.nearly_ge_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_ge_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

    a.checkpoint();

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_ge_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
}

#[test]
fn nearly_lt() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_lt_ulps().times(0);
    assert!(a.nearly_lt(&b));

    a.checkpoint();

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_lt(&b));

    a.checkpoint();

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_lt(&b));
}

#[test]
fn nearly_le() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_le_ulps().times(0);
    assert!(a.nearly_le(&b));

    a.checkpoint();

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_le(&b));

    a.checkpoint();

    a.expect_nearly_le_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_le_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_le(&b));
}

#[test]
fn nearly_gt() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_gt_ulps().times(0);
    assert!(a.nearly_gt(&b));

    a.checkpoint();

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_gt(&b));

    a.checkpoint();

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_gt(&b));
}

#[test]
fn nearly_ge() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_ge_ulps().times(0);
    assert!(a.nearly_ge(&b));

    a.checkpoint();

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_ge(&b));

    a.checkpoint();

    a.expect_nearly_ge_eps()
        .with(eq(Rhs(5)), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_ge_ulps()
        .with(eq(Rhs(5)), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_ge(&b));
}
