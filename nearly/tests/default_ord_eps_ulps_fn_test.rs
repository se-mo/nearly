use mockall::{mock, predicate::eq, Sequence};
use nearly::{
    EpsTolerance, EpsToleranceType, NearlyEqEps, NearlyEqUlps, NearlyOrdEps, NearlyOrdUlps,
    UlpsTolerance, UlpsToleranceType,
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

    impl NearlyOrdEps<Rhs> for Lhs {
        fn nearly_lt_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_gt_eps(&self, other: &Rhs, eps: &EpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyOrdUlps<Rhs> for Lhs {
        fn nearly_lt_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_gt_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<Self, Rhs>) -> bool;
    }
);

#[test]
fn nearly_le_eps() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_eq_eps().times(0);
    assert!(a.nearly_le_eps(&b, &0.1));

    a.checkpoint();

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_le_eps(&b, &0.1));

    a.checkpoint();

    a.expect_nearly_lt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_le_eps(&b, &0.1));
}

#[test]
fn nearly_ge_eps() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_eq_eps().times(0);
    assert!(a.nearly_ge_eps(&b, &0.1));

    a.checkpoint();

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_ge_eps(&b, &0.1));

    a.checkpoint();

    a.expect_nearly_gt_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_eps()
        .with(eq(Rhs(5)), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_ge_eps(&b, &0.1));
}

#[test]
fn nearly_le_ulps() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_eq_ulps().times(0);
    assert!(a.nearly_le_ulps(&b, &5));

    a.checkpoint();

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_le_ulps(&b, &5));

    a.checkpoint();

    a.expect_nearly_lt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_le_ulps(&b, &5));
}

#[test]
fn nearly_ge_ulps() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs(5);

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    a.expect_nearly_eq_ulps().times(0);
    assert!(a.nearly_ge_ulps(&b, &5));

    a.checkpoint();

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);
    assert!(a.nearly_ge_ulps(&b, &5));

    a.checkpoint();

    a.expect_nearly_gt_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    a.expect_nearly_eq_ulps()
        .with(eq(Rhs(5)), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);
    assert!(!a.nearly_ge_ulps(&b, &5));
}
