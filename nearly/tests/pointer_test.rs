#![cfg(feature = "std")]

use mockall::mock;
use mockall::predicate::eq;
use nearly::{
    EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance,
    UlpsTolerance, UlpsToleranceType,
};
use paste::paste;
use std::boxed::Box;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

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
        fn nearly_eq_eps(&self, other: &Rhs, eps: EpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_ne_eps(&self, other: &Rhs, eps: EpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyEqUlps<Rhs> for Lhs {
        fn nearly_eq_ulps(&self, other: &Rhs, ulps: UlpsToleranceType<Self, Rhs>) -> bool;
        fn nearly_ne_ulps(&self, other: &Rhs, ulps: UlpsToleranceType<Self, Rhs>) -> bool;
    }

    impl NearlyEqTol<Rhs> for Lhs {
        fn nearly_eq_tol(&self, other: &Rhs, tolerance: Tolerance<Self, Rhs>) -> bool;
        fn nearly_ne_tol(&self, other: &Rhs, tolerance: Tolerance<Self, Rhs>) -> bool;
    }

    impl NearlyEq<Rhs> for Lhs {
        fn nearly_eq(&self, other: &Rhs) -> bool;
        fn nearly_ne(&self, other: &Rhs) -> bool;
    }
);

macro_rules! get_type {
    ($inner: ty, arc) => {
        Arc<$inner>
    };
    ($inner: ty, box) => {
        Box<$inner>
    };
    ($inner: ty, rc) => {
        Rc<$inner>
    };
    ($inner: ty, pin) => {
        Pin<&$inner>
    };
}

macro_rules! lhs_type {
    ($ptr: tt) => {
        get_type!(MockLhs, $ptr)
    };
}

macro_rules! rhs_type {
    ($ptr: tt) => {
        get_type!(Rhs, $ptr)
    };
}

macro_rules! lhs_value {
    ($value: ident, pin) => {
        <lhs_type!(pin)>::new(&$value)
    };
    ($value: ident, $ptr: tt) => {
        <lhs_type!($ptr)>::new($value)
    };
}

macro_rules! rhs_value {
    (pin) => {
        <rhs_type!(pin)>::new(&Rhs(5))
    };
    ($ptr: tt) => {
        <rhs_type!($ptr)>::new(Rhs(5))
    };
}

macro_rules! impl_test {
    ($ptr: tt) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_ $ptr>]() {
                let b: rhs_type!($ptr) = rhs_value!($ptr);

                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_eps()
                        .with(eq(Rhs(5)), eq(0.1))
                        .times(1)
                        .return_const(true);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(a.nearly_eq_eps(&b, 0.1));
                }
                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_eps()
                        .with(eq(Rhs(5)), eq(0.1))
                        .times(1)
                        .return_const(false);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(!a.nearly_eq_eps(&b, 0.1));
                }
            }

            #[test]
            fn [<nearly_eq_ulps_ $ptr>]() {
                let b: rhs_type!($ptr) = rhs_value!($ptr);

                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_ulps()
                        .with(eq(Rhs(5)), eq(5))
                        .times(1)
                        .return_const(true);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(a.nearly_eq_ulps(&b, 5));
                }
                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_ulps()
                        .with(eq(Rhs(5)), eq(5))
                        .times(1)
                        .return_const(false);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(!a.nearly_eq_ulps(&b, 5));
                }
            }
        }
    };
}

impl_test!(arc);
impl_test!(box);
impl_test!(rc);
impl_test!(pin);
