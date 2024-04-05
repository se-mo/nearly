use mockall::mock;
use nearly::{
    EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, NearlyOrd,
    NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps, Tolerance, UlpsTolerance, UlpsToleranceType,
};

#[derive(Debug, PartialEq)]
pub struct Rhs(pub i32);

mock!(
    #[derive(Debug)]
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

    impl NearlyOrdTol<Rhs> for Lhs {
        fn nearly_lt_tol(&self, other: &Rhs, tol: &Tolerance<Self, Rhs>) -> bool;
        fn nearly_le_tol(&self, other: &Rhs, tol: &Tolerance<Self, Rhs>) -> bool;
        fn nearly_gt_tol(&self, other: &Rhs, tol: &Tolerance<Self, Rhs>) -> bool;
        fn nearly_ge_tol(&self, other: &Rhs, tol: &Tolerance<Self, Rhs>) -> bool;
    }

    impl NearlyOrd<Rhs> for Lhs {
        fn nearly_lt(&self, other: &Rhs) -> bool;
        fn nearly_le(&self, other: &Rhs) -> bool;
        fn nearly_gt(&self, other: &Rhs) -> bool;
        fn nearly_ge(&self, other: &Rhs) -> bool;
    }
);

/// Inner value of enum variant
#[macro_export]
macro_rules! inner_value {
    ($target: expr, $var: path) => {{
        if let $var(ref mut inner) = $target {
            inner
        } else {
            panic!("mismatch variant when extracting inner value")
        }
    }};
}
