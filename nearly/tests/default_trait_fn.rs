use mockall::predicate::eq;
use mockall::{mock, Sequence};
use nearly::{
    EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance,
    UlpsTolerance, UlpsToleranceType,
};

#[derive(Debug, PartialEq)]
struct Rhs;

mock!(
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
        fn nearly_eq_eps(&self, other: &Rhs, eps: EpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyEqUlps<Rhs> for Lhs {
        fn nearly_eq_ulps(&self, other: &Rhs, ulps: UlpsToleranceType<Self, Rhs>) -> bool;
    }
);

#[test]
fn nearly_eq_eps() {
    let mut a = MockLhs::new();
    let b = Rhs;

    a.expect_nearly_eq_eps()
        .with(eq(Rhs), eq(0.1))
        .times(1)
        .return_const(false);

    assert!(a.nearly_ne_eps(&b, 0.1));
}

#[test]
fn nearly_eq_ulps() {
    let mut a = MockLhs::new();
    let b = Rhs;

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs), eq(5))
        .times(1)
        .return_const(false);

    assert!(a.nearly_ne_ulps(&b, 5));
}

#[test]
fn nearly_eq_tol() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs;

    a.expect_nearly_eq_eps()
        .with(eq(Rhs), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs), eq(0.1))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs), eq(5))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);

    assert!(a.nearly_eq_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
    assert!(a.nearly_ne_tol(&b, Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
}

#[test]
fn nearly_eq() {
    let mut seq = Sequence::new();

    let mut a = MockLhs::new();
    let b = Rhs;

    a.expect_nearly_eq_eps()
        .with(eq(Rhs), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(true);

    a.expect_nearly_eq_eps()
        .with(eq(Rhs), eq(0.01))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);

    a.expect_nearly_eq_ulps()
        .with(eq(Rhs), eq(3))
        .times(1)
        .in_sequence(&mut seq)
        .return_const(false);

    assert!(a.nearly_eq(&b));
    assert!(a.nearly_ne(&b));
}