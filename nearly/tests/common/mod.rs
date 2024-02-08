use mockall::mock;
use nearly::{
    EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance,
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

    impl NearlyEqTol<Rhs> for Lhs {
        fn nearly_eq_tol(&self, other: &Rhs, tol: &Tolerance<Self, Rhs>) -> bool;
    }

    impl NearlyEq<Rhs> for Lhs {}
);
