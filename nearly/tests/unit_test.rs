#![allow(clippy::let_unit_value)]

use nearly::{
    NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, NearlyOrd, NearlyOrdEps, NearlyOrdTol,
    NearlyOrdUlps, Tolerance,
};

#[test]
fn nearly_eq_unit() {
    let a = ();
    let b = ();

    assert!(a.nearly_eq_eps(&b, &()));
    assert!(a.nearly_eq_ulps(&b, &()));
    assert!(a.nearly_eq_tol(&b, &Tolerance::<()>::new((), ())));
    assert!(a.nearly_eq(&b));
}

#[test]
fn nearly_ne_unit() {
    let a = ();
    let b = ();

    assert!(!a.nearly_ne_eps(&b, &()));
    assert!(!a.nearly_ne_ulps(&b, &()));
    assert!(!a.nearly_ne_tol(&b, &Tolerance::<()>::new((), ())));
    assert!(!a.nearly_ne(&b));
}

#[test]
fn nearly_lt_unit() {
    let a = ();
    let b = ();

    assert!(!a.nearly_lt_eps(&b, &()));
    assert!(!a.nearly_lt_ulps(&b, &()));
    assert!(!a.nearly_lt_tol(&b, &Tolerance::<()>::new((), ())));
    assert!(!a.nearly_lt(&b));
}

#[test]
fn nearly_le_unit() {
    let a = ();
    let b = ();

    assert!(a.nearly_le_eps(&b, &()));
    assert!(a.nearly_le_ulps(&b, &()));
    assert!(a.nearly_le_tol(&b, &Tolerance::<()>::new((), ())));
    assert!(a.nearly_le(&b));
}

#[test]
fn nearly_gt_unit() {
    let a = ();
    let b = ();

    assert!(!a.nearly_gt_eps(&b, &()));
    assert!(!a.nearly_gt_ulps(&b, &()));
    assert!(!a.nearly_gt_tol(&b, &Tolerance::<()>::new((), ())));
    assert!(!a.nearly_gt(&b));
}

#[test]
fn nearly_ge_unit() {
    let a = ();
    let b = ();

    assert!(a.nearly_ge_eps(&b, &()));
    assert!(a.nearly_ge_ulps(&b, &()));
    assert!(a.nearly_ge_tol(&b, &Tolerance::<()>::new((), ())));
    assert!(a.nearly_ge(&b));
}
