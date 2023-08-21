#[cfg(feature = "std")]
mod std_types {
    use nearly::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
    use nearly::{ToleranceF32, ToleranceF64};
    use paste::paste;

    use std::boxed::Box;
    use std::rc::Rc;
    use std::sync::Arc;

    use std::pin::Pin;

    macro_rules! impl_test_f32 {
        ($ptr: ty, $name: expr, [$($ref:tt)*]) => {
            paste! {
                #[test]
                fn [<nearly_eq_eps_ $name _f32>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                    let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                    assert_ne!(a, b);

                    assert!(!a.nearly_eq_eps(&b, 0.0000007));
                    assert!(!b.nearly_eq_eps(&a, 0.0000007));

                    assert!(a.nearly_eq_eps(&b, 0.0000009));
                    assert!(b.nearly_eq_eps(&a, 0.0000009));

                    assert!(a.nearly_eq_eps(&b, 0.0000011));
                    assert!(b.nearly_eq_eps(&a, 0.0000011));
                }

                #[test]
                fn [<nearly_ne_eps_ $name _f32>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                    let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                    assert_ne!(a, b);

                    assert!(a.nearly_ne_eps(&b, 0.0000007));
                    assert!(b.nearly_ne_eps(&a, 0.0000007));

                    assert!(!a.nearly_ne_eps(&b, 0.0000009));
                    assert!(!b.nearly_ne_eps(&a, 0.0000009));

                    assert!(!a.nearly_ne_eps(&b, 0.0000011));
                    assert!(!b.nearly_ne_eps(&a, 0.0000011));
                }

                #[test]
                fn [<nearly_eq_ulps_ $name _f32>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                    let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                    assert_ne!(a, b);

                    assert!(!a.nearly_eq_ulps(&b, 6));
                    assert!(!b.nearly_eq_ulps(&a, 6));

                    assert!(a.nearly_eq_ulps(&b, 7));
                    assert!(b.nearly_eq_ulps(&a, 7));

                    assert!(a.nearly_eq_ulps(&b, 8));
                    assert!(b.nearly_eq_ulps(&a, 8));
                }

                #[test]
                fn [<nearly_ne_ulps_ $name _f32>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                    let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                    assert_ne!(a, b);

                    assert!(a.nearly_ne_ulps(&b, 6));
                    assert!(b.nearly_ne_ulps(&a, 6));

                    assert!(!a.nearly_ne_ulps(&b, 7));
                    assert!(!b.nearly_ne_ulps(&a, 7));

                    assert!(!a.nearly_ne_ulps(&b, 8));
                    assert!(!b.nearly_ne_ulps(&a, 8));
                }

                #[test]
                fn [<nearly_eq_tol_ $name _f32>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                    let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                    assert_ne!(a, b);

                    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 6)));
                    assert!(!a.nearly_eq_tol(&b, ToleranceF32::new(0.0000007, 0)));
                    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 6)));
                    assert!(!b.nearly_eq_tol(&a, ToleranceF32::new(0.0000007, 0)));

                    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 7)));
                    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0000009, 0)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 7)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0000009, 0)));

                    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0, 8)));
                    assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.0000011, 0)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0, 8)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF32::new(0.0000011, 0)));
                }

                #[test]
                fn [<nearly_ne_tol_ $name _f32>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                    let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                    assert_ne!(a, b);

                    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 6)));
                    assert!(a.nearly_ne_tol(&b, ToleranceF32::new(0.0000007, 0)));
                    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 6)));
                    assert!(b.nearly_ne_tol(&a, ToleranceF32::new(0.0000007, 0)));

                    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 7)));
                    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0000009, 0)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 7)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0000009, 0)));

                    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0, 8)));
                    assert!(!a.nearly_ne_tol(&b, ToleranceF32::new(0.0000011, 0)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0, 8)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF32::new(0.0000011, 0)));
                }

                #[test]
                fn [<nearly_eq_ $name _f32>]() {
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                        let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                        assert_ne!(a, b);

                        assert!(a.nearly_eq(&b));
                        assert!(b.nearly_eq(&a));
                    }
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                        let b: $ptr = $ptr::new($($ref)* 1.1_f32);
                        assert_ne!(a, b);

                        assert!(!a.nearly_eq(&b));
                        assert!(!b.nearly_eq(&a));
                    }
                }

                #[test]
                fn [<nearly_ne_ $name _f32>]() {
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                        let b: $ptr = $ptr::new($($ref)* 1.0000008_f32);
                        assert_ne!(a, b);

                        assert!(!a.nearly_ne(&b));
                        assert!(!b.nearly_ne(&a));
                    }
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f32);
                        let b: $ptr = $ptr::new($($ref)* 1.1_f32);
                        assert_ne!(a, b);

                        assert!(a.nearly_ne(&b));
                        assert!(b.nearly_ne(&a));
                    }
                }
            }
        };
    }

    macro_rules! impl_test_f64 {
        ($ptr: ty, $name: expr, [$($ref:tt)*]) => {
            paste! {
                #[test]
                fn [<nearly_eq_eps_ $name _f64>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                    let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                    assert_ne!(a, b);

                    assert!(!a.nearly_eq_eps(&b, 0.000000000000001));
                    assert!(!b.nearly_eq_eps(&a, 0.000000000000001));

                    assert!(a.nearly_eq_eps(&b, 0.0000000000000016));
                    assert!(b.nearly_eq_eps(&a, 0.0000000000000016));

                    assert!(a.nearly_eq_eps(&b, 0.0000000000004));
                    assert!(b.nearly_eq_eps(&a, 0.0000000000004));
                }

                #[test]
                fn [<nearly_ne_eps_ $name _f64>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                    let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                    assert_ne!(a, b);

                    assert!(a.nearly_ne_eps(&b, 0.000000000000001));
                    assert!(b.nearly_ne_eps(&a, 0.000000000000001));

                    assert!(!a.nearly_ne_eps(&b, 0.0000000000000016));
                    assert!(!b.nearly_ne_eps(&a, 0.0000000000000016));

                    assert!(!a.nearly_ne_eps(&b, 0.000000000000002));
                    assert!(!b.nearly_ne_eps(&a, 0.000000000000002));
                }

                #[test]
                fn [<nearly_eq_ulps_ $name _f64>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                    let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                    assert_ne!(a, b);

                    assert!(!a.nearly_eq_ulps(&b, 6));
                    assert!(!b.nearly_eq_ulps(&a, 6));

                    assert!(a.nearly_eq_ulps(&b, 7));
                    assert!(b.nearly_eq_ulps(&a, 7));

                    assert!(a.nearly_eq_ulps(&b, 8));
                    assert!(b.nearly_eq_ulps(&a, 8));
                }

                #[test]
                fn [<nearly_ne_ulps_ $name _f64>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                    let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                    assert_ne!(a, b);

                    assert!(a.nearly_ne_ulps(&b, 6));
                    assert!(b.nearly_ne_ulps(&a, 6));

                    assert!(!a.nearly_ne_ulps(&b, 7));
                    assert!(!b.nearly_ne_ulps(&a, 7));

                    assert!(!a.nearly_ne_ulps(&b, 8));
                    assert!(!b.nearly_ne_ulps(&a, 8));
                }

                #[test]
                fn [<nearly_eq_tol_ $name _f64>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                    let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                    assert_ne!(a, b);

                    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 6)));
                    assert!(!a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000001, 0)));
                    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 6)));
                    assert!(!b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000001, 0)));

                    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 7)));
                    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0000000000000016, 0)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 7)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0000000000000016, 0)));

                    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.0, 8)));
                    assert!(a.nearly_eq_tol(&b, ToleranceF64::new(0.000000000000002, 0)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.0, 8)));
                    assert!(b.nearly_eq_tol(&a, ToleranceF64::new(0.000000000000002, 0)));
                }

                #[test]
                fn [<nearly_ne_tol_ $name _f64>]() {
                    let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                    let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                    assert_ne!(a, b);

                    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 6)));
                    assert!(a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000001, 0)));
                    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 6)));
                    assert!(b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000001, 0)));

                    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 7)));
                    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0000000000000016, 0)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 7)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0000000000000016, 0)));

                    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.0, 8)));
                    assert!(!a.nearly_ne_tol(&b, ToleranceF64::new(0.000000000000002, 0)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.0, 8)));
                    assert!(!b.nearly_ne_tol(&a, ToleranceF64::new(0.000000000000002, 0)));
                }

                #[test]
                fn [<nearly_eq_ $name _f64>]() {
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                        let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                        assert_ne!(a, b);

                        assert!(a.nearly_eq(&b));
                        assert!(b.nearly_eq(&a));
                    }
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                        let b: $ptr = $ptr::new($($ref)* 1.1_f64);
                        assert_ne!(a, b);

                        assert!(!a.nearly_eq(&b));
                        assert!(!b.nearly_eq(&a));
                    }
                }

                #[test]
                fn [<nearly_ne_ $name _f64>]() {
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                        let b: $ptr = $ptr::new($($ref)* 1.0000000000000016_f64);
                        assert_ne!(a, b);

                        assert!(!a.nearly_ne(&b));
                        assert!(!b.nearly_ne(&a));
                    }
                    {
                        let a: $ptr = $ptr::new($($ref)* 1.0_f64);
                        let b: $ptr = $ptr::new($($ref)* 1.1_f64);
                        assert_ne!(a, b);

                        assert!(a.nearly_ne(&b));
                        assert!(b.nearly_ne(&a));
                    }
                }
            }
        };
    }

    impl_test_f32!(Arc::<f32>, "arc", []);
    impl_test_f64!(Arc::<f64>, "arc", []);

    impl_test_f32!(Box::<f32>, "box", []);
    impl_test_f64!(Box::<f64>, "box", []);

    impl_test_f32!(Rc::<f32>, "rc", []);
    impl_test_f64!(Rc::<f64>, "rc", []);

    impl_test_f32!(Pin::<&f32>, "pin", [&]);
    impl_test_f64!(Pin::<&f64>, "pin", [&]);
}
