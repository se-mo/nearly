#![allow(clippy::let_unit_value)]

use nearly::Tolerance;
use nearly::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};

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
