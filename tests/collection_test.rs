use nearly::{assert_nearly_eq, assert_nearly_ne};
use nearly::{debug_assert_nearly_eq, debug_assert_nearly_ne};
use nearly::{nearly_eq, nearly_ne};
use nearly::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use nearly::{ToleranceF32, ToleranceF64};
use paste::paste;

static A_ARRAY_F32: [f32; 5] = [1.0, 1.0, 1.0, 1.0000008, 1.0];
static B_ARRAY_F32: [f32; 5] = [1.0, 1.0000008, 1.0, 1.0, 1.0];
static C_ARRAY_F32: [f32; 5] = [1.0, 1.1, 1.0, 1.0000008, 1.0];
static D_ARRAY_F32: [f32; 6] = [1.0, 1.0, 1.0, 1.0000008, 1.0, 1.0];

static A_ARRAY_F64: [f64; 5] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0];
static B_ARRAY_F64: [f64; 5] = [1.0, 1.0000000000000016, 1.0, 1.0, 1.0];
static C_ARRAY_F64: [f64; 5] = [1.0, 1.1, 1.0, 1.0000000000000016, 1.0];
static D_ARRAY_F64: [f64; 6] = [1.0, 1.0, 1.0, 1.0000000000000016, 1.0, 1.0];

macro_rules! get_value {
    ($name: expr, "a", $type: ty) => {
        paste! {
            [<get_value_ $name>]!("a", $type)
        }
    };
    ($name: expr, "b", $type: ty) => {
        paste! {
            [<get_value_ $name>]!("b", $type)
        }
    };
    ($name: expr, "c", $type: ty) => {
        paste! {
            [<get_value_ $name>]!("c", $type)
        }
    };
    ($name: expr, "d", $type: ty) => {
        paste! {
            [<get_value_ $name>]!("d", $type)
        }
    };
}

macro_rules! get_value_array {
    ("a", f32) => {
        A_ARRAY_F32
    };
    ("b", f32) => {
        B_ARRAY_F32
    };
    ("c", f32) => {
        C_ARRAY_F32
    };
    ("d", f32) => {
        D_ARRAY_F32
    };
    ("a", f64) => {
        A_ARRAY_F64
    };
    ("b", f64) => {
        B_ARRAY_F64
    };
    ("c", f64) => {
        C_ARRAY_F64
    };
    ("d", f64) => {
        D_ARRAY_F64
    };
}

macro_rules! get_value_slice {
    ("a", f32) => {
        &A_ARRAY_F32[0..5]
    };
    ("b", f32) => {
        &B_ARRAY_F32[0..5]
    };
    ("c", f32) => {
        &C_ARRAY_F32[0..5]
    };
    ("d", f32) => {
        &D_ARRAY_F32[0..6]
    };
    ("a", f64) => {
        &A_ARRAY_F64[0..5]
    };
    ("b", f64) => {
        &B_ARRAY_F64[0..5]
    };
    ("c", f64) => {
        &C_ARRAY_F64[0..5]
    };
    ("d", f64) => {
        &D_ARRAY_F64[0..6]
    };
}

macro_rules! get_value_vec {
    ("a", f32) => {
        A_ARRAY_F32.into()
    };
    ("b", f32) => {
        B_ARRAY_F32.into()
    };
    ("c", f32) => {
        C_ARRAY_F32.into()
    };
    ("d", f32) => {
        D_ARRAY_F32.into()
    };
    ("a", f64) => {
        A_ARRAY_F64.into()
    };
    ("b", f64) => {
        B_ARRAY_F64.into()
    };
    ("c", f64) => {
        C_ARRAY_F64.into()
    };
    ("d", f64) => {
        D_ARRAY_F64.into()
    };
}

macro_rules! get_value_vec_deque {
    ("a", f32) => {
        A_ARRAY_F32.into()
    };
    ("b", f32) => {
        B_ARRAY_F32.into()
    };
    ("c", f32) => {
        C_ARRAY_F32.into()
    };
    ("d", f32) => {
        D_ARRAY_F32.into()
    };
    ("a", f64) => {
        A_ARRAY_F64.into()
    };
    ("b", f64) => {
        B_ARRAY_F64.into()
    };
    ("c", f64) => {
        C_ARRAY_F64.into()
    };
    ("d", f64) => {
        D_ARRAY_F64.into()
    };
}

macro_rules! get_value_linked_list {
    ("a", f32) => {
        LinkedList::from(A_ARRAY_F32)
    };
    ("b", f32) => {
        LinkedList::from(B_ARRAY_F32)
    };
    ("c", f32) => {
        LinkedList::from(C_ARRAY_F32)
    };
    ("d", f32) => {
        LinkedList::from(D_ARRAY_F32)
    };
    ("a", f64) => {
        LinkedList::from(A_ARRAY_F64)
    };
    ("b", f64) => {
        LinkedList::from(B_ARRAY_F64)
    };
    ("c", f64) => {
        LinkedList::from(C_ARRAY_F64)
    };
    ("d", f64) => {
        LinkedList::from(D_ARRAY_F64)
    };
}

macro_rules! create_map {
    ($map: ident, $array: expr, 5) => {
        $map::from([
            (0, $array[0]),
            (1, $array[1]),
            (2, $array[2]),
            (3, $array[3]),
            (4, $array[4]),
        ])
    };
    ($map: ident, $array: expr, 6) => {
        $map::from([
            (0, $array[0]),
            (1, $array[1]),
            (2, $array[2]),
            (3, $array[3]),
            (4, $array[4]),
            (5, $array[5]),
        ])
    };
}

macro_rules! get_value_hashmap {
    ("a", f32) => {
        create_map!(HashMap, A_ARRAY_F32, 5)
    };
    ("b", f32) => {
        create_map!(HashMap, B_ARRAY_F32, 5)
    };
    ("c", f32) => {
        create_map!(HashMap, C_ARRAY_F32, 5)
    };
    ("d", f32) => {
        create_map!(HashMap, D_ARRAY_F32, 6)
    };
    ("a", f64) => {
        create_map!(HashMap, A_ARRAY_F64, 5)
    };
    ("b", f64) => {
        create_map!(HashMap, B_ARRAY_F64, 5)
    };
    ("c", f64) => {
        create_map!(HashMap, C_ARRAY_F64, 5)
    };
    ("d", f64) => {
        create_map!(HashMap, D_ARRAY_F64, 6)
    };
}

macro_rules! get_value_btree_map {
    ("a", f32) => {
        create_map!(BTreeMap, A_ARRAY_F32, 5)
    };
    ("b", f32) => {
        create_map!(BTreeMap, B_ARRAY_F32, 5)
    };
    ("c", f32) => {
        create_map!(BTreeMap, C_ARRAY_F32, 5)
    };
    ("d", f32) => {
        create_map!(BTreeMap, D_ARRAY_F32, 6)
    };
    ("a", f64) => {
        create_map!(BTreeMap, A_ARRAY_F64, 5)
    };
    ("b", f64) => {
        create_map!(BTreeMap, B_ARRAY_F64, 5)
    };
    ("c", f64) => {
        create_map!(BTreeMap, C_ARRAY_F64, 5)
    };
    ("d", f64) => {
        create_map!(BTreeMap, D_ARRAY_F64, 6)
    };
}

macro_rules! impl_test_f32 {
    ($lhs: ty, $rhs: ty, $name_lhs: expr, $name_rhs: expr) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert!(!a.nearly_eq_eps(&b, 0.0000007));
                assert!(!b.nearly_eq_eps(&a, 0.0000007));

                assert!(a.nearly_eq_eps(&b, 0.0000009));
                assert!(b.nearly_eq_eps(&a, 0.0000009));

                assert!(a.nearly_eq_eps(&b, 0.0000011));
                assert!(b.nearly_eq_eps(&a, 0.0000011));
            }

            #[test]
            fn [<nearly_ne_eps_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert!(a.nearly_ne_eps(&b, 0.0000007));
                assert!(b.nearly_ne_eps(&a, 0.0000007));

                assert!(!a.nearly_ne_eps(&b, 0.0000009));
                assert!(!b.nearly_ne_eps(&a, 0.0000009));

                assert!(!a.nearly_ne_eps(&b, 0.0000011));
                assert!(!b.nearly_ne_eps(&a, 0.0000011));
            }

            #[test]
            fn [<nearly_eq_ulps_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert!(!a.nearly_eq_ulps(&b, 6));
                assert!(!b.nearly_eq_ulps(&a, 6));

                assert!(a.nearly_eq_ulps(&b, 7));
                assert!(b.nearly_eq_ulps(&a, 7));

                assert!(a.nearly_eq_ulps(&b, 8));
                assert!(b.nearly_eq_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_ne_ulps_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert!(a.nearly_ne_ulps(&b, 6));
                assert!(b.nearly_ne_ulps(&a, 6));

                assert!(!a.nearly_ne_ulps(&b, 7));
                assert!(!b.nearly_ne_ulps(&a, 7));

                assert!(!a.nearly_ne_ulps(&b, 8));
                assert!(!b.nearly_ne_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_eq_tol_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

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
            fn [<nearly_ne_tol_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

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
            fn [<nearly_eq_ $name_lhs _ $name_rhs _f32>]() {
                {
                    let a: $lhs = get_value!($name_lhs, "a", f32);
                    let b: $rhs = get_value!($name_rhs, "b", f32);

                    assert!(a.nearly_eq(&b));
                    assert!(b.nearly_eq(&a));
                }
                {
                    let a: $lhs = get_value!($name_lhs, "a", f32);
                    let c: $rhs = get_value!($name_rhs, "c", f32);

                    assert!(!a.nearly_eq(&c));
                    assert!(!c.nearly_eq(&a));
                }
            }

            #[test]
            fn [<nearly_ne_ $name_lhs _ $name_rhs _f32>]() {
                {
                    let a: $lhs = get_value!($name_lhs, "a", f32);
                    let b: $rhs = get_value!($name_rhs, "b", f32);

                    assert!(!a.nearly_ne(&b));
                    assert!(!b.nearly_ne(&a));
                }
                {
                    let a: $lhs = get_value!($name_lhs, "a", f32);
                    let c: $rhs = get_value!($name_rhs, "c", f32);

                    assert!(a.nearly_ne(&c));
                    assert!(c.nearly_ne(&a));
                }
            }

            #[test]
            fn [<macro_nearly_eq_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert!(!nearly_eq!(a, b, eps = 0.0000007));
                assert!(nearly_eq!(a, b, eps = 0.0000009));

                assert!(!nearly_eq!(a, b, ulps = 6));
                assert!(nearly_eq!(a, b, ulps = 7));

                assert!(!nearly_eq!(a, b, eps = 0.0000007, ulps = 6));
                assert!(nearly_eq!(a, b, eps = 0.0000009, ulps = 7));

                assert!(!nearly_eq!(a, b, tol = ToleranceF32::new(0.0000007, 6)));
                assert!(nearly_eq!(a, b, tol = ToleranceF32::new(0.0000009, 7)));

                assert!(nearly_eq!(a, b));
            }

            #[test]
            fn [<macro_nearly_ne_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert!(nearly_ne!(a, b, eps = 0.0000007));
                assert!(!nearly_ne!(a, b, eps = 0.0000009));

                assert!(nearly_ne!(a, b, ulps = 6));
                assert!(!nearly_ne!(a, b, ulps = 7));

                assert!(nearly_ne!(a, b, eps = 0.0000007, ulps = 6));
                assert!(!nearly_ne!(a, b, eps = 0.0000009, ulps = 7));

                assert!(nearly_ne!(a, b, tol = ToleranceF32::new(0.0000007, 6)));
                assert!(!nearly_ne!(a, b, tol = ToleranceF32::new(0.0000009, 7)));

                assert!(!nearly_ne!(a, b));
            }

            #[test]
            fn [<macro_assert_nearly_eq_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert_nearly_eq!(a, b, eps = 0.0000009);
                assert_nearly_eq!(a, b, ulps = 7);
                assert_nearly_eq!(a, b, eps = 0.0000009, ulps = 7);
                assert_nearly_eq!(a, b, tol = ToleranceF32::new(0.0000009, 7));
                assert_nearly_eq!(a, b);
            }

            #[test]
            fn [<macro_assert_nearly_ne_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert_nearly_ne!(a, b, eps = 0.0000007);
                assert_nearly_ne!(a, b, ulps = 6);
                assert_nearly_ne!(a, b, eps = 0.0000007, ulps = 6);
                assert_nearly_ne!(a, b, tol = ToleranceF32::new(0.0000007, 6));

                let c: $rhs = get_value!($name_rhs, "c", f32);
                assert_nearly_ne!(a, c);
            }

            #[test]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_assert_nearly_eq_ $name_lhs _ $name_rhs _panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert_nearly_eq!(a, b, eps = 0.0000007, ulps = 6);
            }

            #[test]
            #[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`"#)]
            fn [<macro_assert_nearly_ne_ $name_lhs _ $name_rhs _panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                assert_nearly_ne!(a, b, eps = 0.0000009, ulps = 7);
            }

            #[test]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                debug_assert_nearly_eq!(a, b, eps = 0.0000009);
                debug_assert_nearly_eq!(a, b, ulps = 7);
                debug_assert_nearly_eq!(a, b, eps = 0.0000009, ulps = 7);
                debug_assert_nearly_eq!(a, b, tol = ToleranceF32::new(0.0000009, 7));
                debug_assert_nearly_eq!(a, b);
            }

            #[test]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                debug_assert_nearly_ne!(a, b, eps = 0.0000007);
                debug_assert_nearly_ne!(a, b, ulps = 6);
                debug_assert_nearly_ne!(a, b, eps = 0.0000007, ulps = 6);
                debug_assert_nearly_ne!(a, b, tol = ToleranceF32::new(0.0000007, 6));

                let c: $rhs = get_value!($name_rhs, "c", f32);
                debug_assert_nearly_ne!(a, c);
            }

            #[test]
            #[cfg(debug_assertions)]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                debug_assert_nearly_eq!(a, b, eps = 0.0000007, ulps = 6);
            }

            #[test]
            #[cfg(not(debug_assertions))]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                debug_assert_nearly_eq!(a, b, eps = 0.0000007, ulps = 6);
            }

            #[test]
            #[cfg(debug_assertions)]
            #[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`"#)]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                debug_assert_nearly_ne!(a, b, eps = 0.0000009, ulps = 7);
            }

            #[test]
            #[cfg(not(debug_assertions))]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let b: $rhs = get_value!($name_rhs, "b", f32);

                debug_assert_nearly_ne!(a, b, eps = 0.0000009, ulps = 7);
            }
        }
    };
}

macro_rules! impl_test_f64 {
    ($lhs: ty, $rhs: ty, $name_lhs: expr, $name_rhs: expr) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert!(!a.nearly_eq_eps(&b, 0.000000000000001));
                assert!(!b.nearly_eq_eps(&a, 0.000000000000001));

                assert!(a.nearly_eq_eps(&b, 0.0000000000000016));
                assert!(b.nearly_eq_eps(&a, 0.0000000000000016));

                assert!(a.nearly_eq_eps(&b, 0.000000000000002));
                assert!(b.nearly_eq_eps(&a, 0.000000000000002));
            }

            #[test]
            fn [<nearly_ne_eps_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert!(a.nearly_ne_eps(&b, 0.000000000000001));
                assert!(b.nearly_ne_eps(&a, 0.000000000000001));

                assert!(!a.nearly_ne_eps(&b, 0.0000000000000016));
                assert!(!b.nearly_ne_eps(&a, 0.0000000000000016));

                assert!(!a.nearly_ne_eps(&b, 0.000000000000002));
                assert!(!b.nearly_ne_eps(&a, 0.000000000000002));
            }

            #[test]
            fn [<nearly_eq_ulps_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert!(!a.nearly_eq_ulps(&b, 6));
                assert!(!b.nearly_eq_ulps(&a, 6));

                assert!(a.nearly_eq_ulps(&b, 7));
                assert!(b.nearly_eq_ulps(&a, 7));

                assert!(a.nearly_eq_ulps(&b, 8));
                assert!(b.nearly_eq_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_ne_ulps_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert!(a.nearly_ne_ulps(&b, 6));
                assert!(b.nearly_ne_ulps(&a, 6));

                assert!(!a.nearly_ne_ulps(&b, 7));
                assert!(!b.nearly_ne_ulps(&a, 7));

                assert!(!a.nearly_ne_ulps(&b, 8));
                assert!(!b.nearly_ne_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_eq_tol_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

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
            fn [<nearly_ne_tol_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

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
            fn [<nearly_eq_ $name_lhs _ $name_rhs _f64>]() {
                {
                    let a: $lhs = get_value!($name_lhs, "a", f64);
                    let b: $rhs = get_value!($name_rhs, "b", f64);

                    assert!(a.nearly_eq(&b));
                    assert!(b.nearly_eq(&a));
                }
                {
                    let a: $lhs = get_value!($name_lhs, "a", f64);
                    let c: $rhs = get_value!($name_rhs, "c", f64);

                    assert!(!a.nearly_eq(&c));
                    assert!(!c.nearly_eq(&a));
                }
            }

            #[test]
            fn [<nearly_ne_ $name_lhs _ $name_rhs _f64>]() {
                {
                    let a: $lhs = get_value!($name_lhs, "a", f64);
                    let b: $rhs = get_value!($name_rhs, "b", f64);

                    assert!(!a.nearly_ne(&b));
                    assert!(!b.nearly_ne(&a));
                }
                {
                    let a: $lhs = get_value!($name_lhs, "a", f64);
                    let c: $rhs = get_value!($name_rhs, "c", f64);

                    assert!(a.nearly_ne(&c));
                    assert!(c.nearly_ne(&a));
                }
            }

            #[test]
            fn [<macro_nearly_eq_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert!(!nearly_eq!(a, b, eps = 0.000000000000001));
                assert!(nearly_eq!(a, b, eps = 0.000000000000002));

                assert!(!nearly_eq!(a, b, ulps = 6));
                assert!(nearly_eq!(a, b, ulps = 7));

                assert!(!nearly_eq!(a, b, eps = 0.000000000000001, ulps = 6));
                assert!(nearly_eq!(a, b, eps = 0.000000000000002, ulps = 7));

                assert!(!nearly_eq!(
                    a,
                    b,
                    tol = ToleranceF64::new(0.000000000000001, 6)
                ));
                assert!(nearly_eq!(
                    a,
                    b,
                    tol = ToleranceF64::new(0.000000000000002, 7)
                ));

                assert!(nearly_eq!(a, b));
            }

            #[test]
            fn [<macro_nearly_ne_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert!(nearly_ne!(a, b, eps = 0.000000000000001));
                assert!(!nearly_ne!(a, b, eps = 0.000000000000002));

                assert!(nearly_ne!(a, b, ulps = 6));
                assert!(!nearly_ne!(a, b, ulps = 7));

                assert!(nearly_ne!(a, b, eps = 0.000000000000001, ulps = 6));
                assert!(!nearly_ne!(a, b, eps = 0.000000000000002, ulps = 7));

                assert!(nearly_ne!(
                    a,
                    b,
                    tol = ToleranceF64::new(0.000000000000001, 6)
                ));
                assert!(!nearly_ne!(
                    a,
                    b,
                    tol = ToleranceF64::new(0.000000000000002, 7)
                ));

                assert!(!nearly_ne!(a, b));
            }

            #[test]
            fn [<macro_assert_nearly_eq_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert_nearly_eq!(a, b, eps = 0.000000000000002);
                assert_nearly_eq!(a, b, ulps = 7);
                assert_nearly_eq!(a, b, eps = 0.000000000000002, ulps = 7);
                assert_nearly_eq!(a, b, tol = ToleranceF64::new(0.000000000000002, 7));
                assert_nearly_eq!(a, b);
            }

            #[test]
            fn [<macro_assert_nearly_ne_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert_nearly_ne!(a, b, eps = 0.000000000000001);
                assert_nearly_ne!(a, b, ulps = 6);
                assert_nearly_ne!(a, b, eps = 0.000000000000001, ulps = 6);
                assert_nearly_ne!(a, b, tol = ToleranceF64::new(0.000000000000001, 6));

                let c: $rhs = get_value!($name_rhs, "c", f64);
                assert_nearly_ne!(a, c);
            }

            #[test]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_assert_nearly_eq_ $name_lhs _ $name_rhs _panaic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert_nearly_eq!(a, b, eps = 0.000000000000001, ulps = 6);
            }

            #[test]
            #[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`"#)]
            fn [<macro_assert_nearly_ne_ $name_lhs _ $name_rhs _panaic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                assert_nearly_ne!(a, b, eps = 0.000000000000002, ulps = 7);
            }

            #[test]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                debug_assert_nearly_eq!(a, b, eps = 0.000000000000002);
                debug_assert_nearly_eq!(a, b, ulps = 7);
                debug_assert_nearly_eq!(a, b, eps = 0.000000000000002, ulps = 7);
                debug_assert_nearly_eq!(a, b, tol = ToleranceF64::new(0.000000000000002, 7));
                debug_assert_nearly_eq!(a, b);
            }

            #[test]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                debug_assert_nearly_ne!(a, b, eps = 0.000000000000001);
                debug_assert_nearly_ne!(a, b, ulps = 6);
                debug_assert_nearly_ne!(a, b, eps = 0.000000000000001, ulps = 6);
                debug_assert_nearly_ne!(a, b, tol = ToleranceF64::new(0.000000000000001, 6));

                let c: $rhs = get_value!($name_rhs, "c", f64);
                debug_assert_nearly_ne!(a, c);
            }

            #[test]
            #[cfg(debug_assertions)]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _panic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                debug_assert_nearly_eq!(a, b, eps = 0.000000000000001, ulps = 6);
            }

            #[test]
            #[cfg(not(debug_assertions))]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _panic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                debug_assert_nearly_eq!(a, b, eps = 0.000000000000001, ulps = 6);
            }

            #[test]
            #[cfg(debug_assertions)]
            #[should_panic(expected = r#"assertion failed: `(left nearly_ne_tol right)`"#)]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _panic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                debug_assert_nearly_ne!(a, b, eps = 0.000000000000002, ulps = 7);
            }

            #[test]
            #[cfg(not(debug_assertions))]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _panic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let b: $rhs = get_value!($name_rhs, "b", f64);

                debug_assert_nearly_ne!(a, b, eps = 0.000000000000002, ulps = 7);
            }
        }
    };
}

macro_rules! impl_test_different_length_f32 {
    ($lhs: ty, $rhs: ty, $name_lhs: expr, $name_rhs: expr) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(!a.nearly_eq_eps(&d, 0.1));
                assert!(!d.nearly_eq_eps(&a, 0.1));
            }

            #[test]
            fn [<nearly_ne_eps_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(a.nearly_ne_eps(&d, 0.1));
                assert!(d.nearly_ne_eps(&a, 0.1));
            }

            #[test]
            fn [<nearly_eq_ulps_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(!a.nearly_eq_ulps(&d, 20));
                assert!(!d.nearly_eq_ulps(&a, 20));
            }

            #[test]
            fn [<nearly_ne_ulps_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(a.nearly_ne_ulps(&d, 20));
                assert!(d.nearly_ne_ulps(&a, 20));
            }

            #[test]
            fn [<nearly_eq_tol_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(!a.nearly_eq_tol(&d, ToleranceF32::new(0.0, 20)));
                assert!(!a.nearly_eq_tol(&d, ToleranceF32::new(0.1, 0)));
                assert!(!d.nearly_eq_tol(&a, ToleranceF32::new(0.0, 20)));
                assert!(!d.nearly_eq_tol(&a, ToleranceF32::new(0.1, 0)));
            }

            #[test]
            fn [<nearly_ne_tol_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(a.nearly_ne_tol(&d, ToleranceF32::new(0.0, 20)));
                assert!(a.nearly_ne_tol(&d, ToleranceF32::new(0.1, 0)));
                assert!(d.nearly_ne_tol(&a, ToleranceF32::new(0.0, 20)));
                assert!(d.nearly_ne_tol(&a, ToleranceF32::new(0.1, 0)));
            }

            #[test]
            fn [<nearly_eq_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(!a.nearly_eq(&d));
                assert!(!d.nearly_eq(&a));
            }

            #[test]
            fn [<nearly_ne_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(a.nearly_ne(&d));
                assert!(d.nearly_ne(&a));
            }

            #[test]
            fn [<macro_nearly_eq_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(!nearly_eq!(a, d, eps = 0.1));
                assert!(!nearly_eq!(a, d, ulps = 20));
                assert!(!nearly_eq!(a, d, eps = 0.1, ulps = 20));
                assert!(!nearly_eq!(a, d, tol = ToleranceF32::new(0.1, 20)));
                assert!(!nearly_eq!(a, d));
            }

            #[test]
            fn [<macro_nearly_ne_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert!(nearly_ne!(a, d, eps = 0.1));
                assert!(nearly_ne!(a, d, ulps = 20));
                assert!(nearly_ne!(a, d, eps = 0.1, ulps = 20));
                assert!(nearly_ne!(a, d, tol = ToleranceF32::new(0.1, 20)));
                assert!(nearly_ne!(a, d));
            }

            #[test]
            fn [<macro_assert_nearly_ne_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert_nearly_ne!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_assert_nearly_eq_ $name_lhs _ $name_rhs _different_length_panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                assert_nearly_eq!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _different_length_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                debug_assert_nearly_ne!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            #[cfg(debug_assertions)]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _different_length_panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                debug_assert_nearly_eq!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            #[cfg(not(debug_assertions))]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _different_length_panic_f32>]() {
                let a: $lhs = get_value!($name_lhs, "a", f32);
                let d: $rhs = get_value!($name_rhs, "d", f32);

                debug_assert_nearly_eq!(a, d, eps = 0.1, ulps = 20);
            }
        }
    }
}

macro_rules! impl_test_different_length_f64 {
    ($lhs: ty, $rhs: ty, $name_lhs: expr, $name_rhs: expr) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(!a.nearly_eq_eps(&d, 0.1));
                assert!(!d.nearly_eq_eps(&a, 0.1));
            }

            #[test]
            fn [<nearly_ne_eps_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(a.nearly_ne_eps(&d, 0.1));
                assert!(d.nearly_ne_eps(&a, 0.1));
            }

            #[test]
            fn [<nearly_eq_ulps_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(!a.nearly_eq_ulps(&d, 20));
                assert!(!d.nearly_eq_ulps(&a, 20));
            }

            #[test]
            fn [<nearly_ne_ulps_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(a.nearly_ne_ulps(&d, 20));
                assert!(d.nearly_ne_ulps(&a, 20));
            }

            #[test]
            fn [<nearly_eq_tol_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(!a.nearly_eq_tol(&d, ToleranceF64::new(0.0, 20)));
                assert!(!a.nearly_eq_tol(&d, ToleranceF64::new(0.1, 0)));
                assert!(!d.nearly_eq_tol(&a, ToleranceF64::new(0.0, 20)));
                assert!(!d.nearly_eq_tol(&a, ToleranceF64::new(0.1, 0)));
            }

            #[test]
            fn [<nearly_ne_tol_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(a.nearly_ne_tol(&d, ToleranceF64::new(0.0, 20)));
                assert!(a.nearly_ne_tol(&d, ToleranceF64::new(0.1, 0)));
                assert!(d.nearly_ne_tol(&a, ToleranceF64::new(0.0, 20)));
                assert!(d.nearly_ne_tol(&a, ToleranceF64::new(0.1, 0)));
            }

            #[test]
            fn [<nearly_eq_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(!a.nearly_eq(&d));
                assert!(!d.nearly_eq(&a));
            }

            #[test]
            fn [<nearly_ne_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(a.nearly_ne(&d));
                assert!(d.nearly_ne(&a));
            }

            #[test]
            fn [<macro_nearly_eq_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(!nearly_eq!(a, d, eps = 0.1));
                assert!(!nearly_eq!(a, d, ulps = 20));
                assert!(!nearly_eq!(a, d, eps = 0.1, ulps = 20));
                assert!(!nearly_eq!(a, d, tol = ToleranceF64::new(0.1, 20)));
                assert!(!nearly_eq!(a, d));
            }

            #[test]
            fn [<macro_nearly_ne_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert!(nearly_ne!(a, d, eps = 0.1));
                assert!(nearly_ne!(a, d, ulps = 20));
                assert!(nearly_ne!(a, d, eps = 0.1, ulps = 20));
                assert!(nearly_ne!(a, d, tol = ToleranceF64::new(0.1, 20)));
                assert!(nearly_ne!(a, d));
            }

            #[test]
            fn [<macro_assert_nearly_ne_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert_nearly_ne!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_assert_nearly_eq_ $name_lhs _ $name_rhs _different_length_panic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                assert_nearly_eq!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            fn [<macro_debug_assert_nearly_ne_ $name_lhs _ $name_rhs _different_length_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                debug_assert_nearly_ne!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            #[cfg(debug_assertions)]
            #[should_panic(expected = r#"assertion failed: `(left nearly_eq_tol right)`"#)]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _different_length_panic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                debug_assert_nearly_eq!(a, d, eps = 0.1, ulps = 20);
            }

            #[test]
            #[cfg(not(debug_assertions))]
            fn [<macro_debug_assert_nearly_eq_ $name_lhs _ $name_rhs _different_length_panic_f64>]() {
                let a: $lhs = get_value!($name_lhs, "a", f64);
                let d: $rhs = get_value!($name_rhs, "d", f64);

                debug_assert_nearly_eq!(a, d, eps = 0.1, ulps = 20);
            }
        }
    }
}

impl_test_f32!([f32; 5], [f32; 5], "array", "array");
impl_test_f64!([f64; 5], [f64; 5], "array", "array");
impl_test_f32!([f32; 5], &[f32], "array", "slice");
impl_test_f64!([f64; 5], &[f64], "array", "slice");

impl_test_f32!(&[f32], &[f32], "slice", "slice");
impl_test_f64!(&[f64], &[f64], "slice", "slice");
impl_test_f32!(&[f32], [f32; 5], "slice", "array");
impl_test_f64!(&[f64], [f64; 5], "slice", "array");

impl_test_different_length_f32!([f32; 5], &[f32], "array", "slice");
impl_test_different_length_f64!([f64; 5], &[f64], "array", "slice");

impl_test_different_length_f32!(&[f32], &[f32], "slice", "slice");
impl_test_different_length_f64!(&[f64], &[f64], "slice", "slice");
impl_test_different_length_f32!(&[f32], [f32; 6], "slice", "array");
impl_test_different_length_f64!(&[f64], [f64; 6], "slice", "array");

#[cfg(feature = "std")]
mod std_types {
    use super::*;
    use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};

    impl_test_f32!(Vec<f32>, Vec<f32>, "vec", "vec");
    impl_test_f64!(Vec<f64>, Vec<f64>, "vec", "vec");
    impl_test_f32!(Vec<f32>, VecDeque<f32>, "vec", "vec_deque");
    impl_test_f64!(Vec<f64>, VecDeque<f64>, "vec", "vec_deque");
    impl_test_f32!(Vec<f32>, [f32; 5], "vec", "array");
    impl_test_f64!(Vec<f64>, [f64; 5], "vec", "array");
    impl_test_f32!(Vec<f32>, &[f32], "vec", "slice");
    impl_test_f64!(Vec<f64>, &[f64], "vec", "slice");

    impl_test_f32!([f32; 5], Vec<f32>, "array", "vec");
    impl_test_f64!([f64; 5], Vec<f64>, "array", "vec");
    impl_test_f32!(&[f32], Vec<f32>, "slice", "vec");
    impl_test_f64!(&[f64], Vec<f64>, "slice", "vec");

    impl_test_f32!(VecDeque<f32>, VecDeque<f32>, "vec_deque", "vec_deque");
    impl_test_f64!(VecDeque<f64>, VecDeque<f64>, "vec_deque", "vec_deque");
    impl_test_f32!(VecDeque<f32>, Vec<f32>, "vec_deque", "vec");
    impl_test_f64!(VecDeque<f64>, Vec<f64>, "vec_deque", "vec");
    impl_test_f32!(VecDeque<f32>, [f32; 5], "vec_deque", "array");
    impl_test_f64!(VecDeque<f64>, [f64; 5], "vec_deque", "array");
    impl_test_f32!(VecDeque<f32>, &[f32], "vec_deque", "slice");
    impl_test_f64!(VecDeque<f64>, &[f64], "vec_deque", "slice");

    impl_test_f32!([f32; 5], Vec<f32>, "array", "vec_deque");
    impl_test_f64!([f64; 5], Vec<f64>, "array", "vec_deque");
    impl_test_f32!(&[f32], Vec<f32>, "slice", "vec_deque");
    impl_test_f64!(&[f64], Vec<f64>, "slice", "vec_deque");

    impl_test_f32!(
        LinkedList<f32>,
        LinkedList<f32>,
        "linked_list",
        "linked_list"
    );
    impl_test_f64!(
        LinkedList<f64>,
        LinkedList<f64>,
        "linked_list",
        "linked_list"
    );

    impl_test_f32!(
        HashMap<i32, f32>,
        HashMap<i32, f32>,
        "hashmap",
        "hashmap"
    );
    impl_test_f64!(
        HashMap<i32, f64>,
        HashMap<i32, f64>,
        "hashmap",
        "hashmap"
    );

    impl_test_f32!(
        BTreeMap<i32, f32>,
        BTreeMap<i32, f32>,
        "btree_map",
        "btree_map"
    );
    impl_test_f64!(
        BTreeMap<i32, f64>,
        BTreeMap<i32, f64>,
        "btree_map",
        "btree_map"
    );

    impl_test_different_length_f32!(Vec<f32>, Vec<f32>, "vec", "vec");
    impl_test_different_length_f64!(Vec<f64>, Vec<f64>, "vec", "vec");
    impl_test_different_length_f32!(Vec<f32>, VecDeque<f32>, "vec", "vec_deque");
    impl_test_different_length_f64!(Vec<f64>, VecDeque<f64>, "vec", "vec_deque");
    impl_test_different_length_f32!(Vec<f32>, [f32; 6], "vec", "array");
    impl_test_different_length_f64!(Vec<f64>, [f64; 6], "vec", "array");
    impl_test_different_length_f32!(Vec<f32>, &[f32], "vec", "slice");
    impl_test_different_length_f64!(Vec<f64>, &[f64], "vec", "slice");

    impl_test_different_length_f32!([f32; 5], Vec<f32>, "array", "vec");
    impl_test_different_length_f64!([f64; 5], Vec<f64>, "array", "vec");
    impl_test_different_length_f32!(&[f32], Vec<f32>, "slice", "vec");
    impl_test_different_length_f64!(&[f64], Vec<f64>, "slice", "vec");

    impl_test_different_length_f32!(VecDeque<f32>, VecDeque<f32>, "vec_deque", "vec_deque");
    impl_test_different_length_f64!(VecDeque<f64>, VecDeque<f64>, "vec_deque", "vec_deque");
    impl_test_different_length_f32!(VecDeque<f32>, Vec<f32>, "vec_deque", "vec");
    impl_test_different_length_f64!(VecDeque<f64>, Vec<f64>, "vec_deque", "vec");
    impl_test_different_length_f32!(VecDeque<f32>, [f32; 6], "vec_deque", "array");
    impl_test_different_length_f64!(VecDeque<f64>, [f64; 6], "vec_deque", "array");
    impl_test_different_length_f32!(VecDeque<f32>, &[f32], "vec_deque", "slice");
    impl_test_different_length_f64!(VecDeque<f64>, &[f64], "vec_deque", "slice");

    impl_test_different_length_f32!([f32; 5], Vec<f32>, "array", "vec_deque");
    impl_test_different_length_f64!([f64; 5], Vec<f64>, "array", "vec_deque");
    impl_test_different_length_f32!(&[f32], Vec<f32>, "slice", "vec_deque");
    impl_test_different_length_f64!(&[f64], Vec<f64>, "slice", "vec_deque");

    impl_test_different_length_f32!(
        LinkedList<f32>,
        LinkedList<f32>,
        "linked_list",
        "linked_list"
    );
    impl_test_different_length_f64!(
        LinkedList<f64>,
        LinkedList<f64>,
        "linked_list",
        "linked_list"
    );

    impl_test_different_length_f32!(
        HashMap<i32, f32>,
        HashMap<i32, f32>,
        "hashmap",
        "hashmap"
    );
    impl_test_different_length_f64!(
        HashMap<i32, f64>,
        HashMap<i32, f64>,
        "hashmap",
        "hashmap"
    );

    impl_test_different_length_f32!(
        BTreeMap<i32, f32>,
        BTreeMap<i32, f32>,
        "btree_map",
        "btree_map"
    );
    impl_test_different_length_f64!(
        BTreeMap<i32, f64>,
        BTreeMap<i32, f64>,
        "btree_map",
        "btree_map"
    );
}
