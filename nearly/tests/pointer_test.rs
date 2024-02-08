#![cfg(feature = "std")]

use mockall::predicate::eq;
use nearly::{NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance};
use paste::paste;
use std::boxed::Box;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

mod common;
use common::{MockLhs, Rhs};

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

                    assert!(a.nearly_eq_eps(&b, &0.1));
                }
                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_eps()
                        .with(eq(Rhs(5)), eq(0.1))
                        .times(1)
                        .return_const(false);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(!a.nearly_eq_eps(&b, &0.1));
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

                    assert!(a.nearly_eq_ulps(&b, &5));
                }
                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_ulps()
                        .with(eq(Rhs(5)), eq(5))
                        .times(1)
                        .return_const(false);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(!a.nearly_eq_ulps(&b, &5));
                }
            }

            #[test]
            fn [<nearly_eq_tol_ $ptr>]() {
                let b: rhs_type!($ptr) = rhs_value!($ptr);

                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_tol()
                        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                        .times(1)
                        .return_const(true);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(a.nearly_eq_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
                }
                {
                    let mut a_val = MockLhs::new();
                    a_val.expect_nearly_eq_tol()
                        .with(eq(Rhs(5)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                        .times(1)
                        .return_const(false);
                    let a: lhs_type!($ptr) = lhs_value!(a_val, $ptr);

                    assert!(!a.nearly_eq_tol(&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
                }
            }
        }
    };
}

impl_test!(arc);
impl_test!(box);
impl_test!(rc);
impl_test!(pin);
