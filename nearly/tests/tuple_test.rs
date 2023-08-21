use nearly::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use nearly::{ToleranceF32, ToleranceF64};
use paste::paste;

static A_ARRAY_F32: [f32; 12] = [
    1.0000008, 1.0, 1.0, 1.0000008, 1.0, 1.0000008, 1.0, 1.0, 1.0, 1.0000008, 1.0, 1.0000008,
];
static B_ARRAY_F32: [f32; 12] = [
    1.0, 1.0000008, 1.0, 1.0, 1.0, 1.0000008, 1.0, 1.0000008, 1.0, 1.0, 1.0, 1.0000008,
];
static C_ARRAY_F32: [f32; 12] = [
    1.1, 1.0, 1.0, 1.0000008, 1.0, 1.0000008, 1.0, 1.0, 1.1, 1.0000008, 1.0, 1.0000008,
];

static A_ARRAY_F64: [f64; 12] = [
    1.0000000000000016,
    1.0,
    1.0,
    1.0000000000000016,
    1.0,
    1.0000000000000016,
    1.0,
    1.0,
    1.0,
    1.0000000000000016,
    1.0,
    1.0000000000000016,
];
static B_ARRAY_F64: [f64; 12] = [
    1.0,
    1.0000000000000016,
    1.0,
    1.0,
    1.0,
    1.0000000000000016,
    1.0,
    1.0000000000000016,
    1.0,
    1.0,
    1.0,
    1.0000000000000016,
];
static C_ARRAY_F64: [f64; 12] = [
    1.1,
    1.0,
    1.0,
    1.0000000000000016,
    1.0,
    1.0000000000000016,
    1.0,
    1.0,
    1.1,
    1.0000000000000016,
    1.0,
    1.0000000000000016,
];

macro_rules! get_value {
    (f32, "a", $size: tt) => {
        tuple_from_array!(A_ARRAY_F32, $size)
    };
    (f32, "b", $size: tt) => {
        tuple_from_array!(B_ARRAY_F32, $size)
    };
    (f32, "c", $size: tt) => {
        tuple_from_array!(C_ARRAY_F32, $size)
    };
    (f64, "a", $size: tt) => {
        tuple_from_array!(A_ARRAY_F64, $size)
    };
    (f64, "b", $size: tt) => {
        tuple_from_array!(B_ARRAY_F64, $size)
    };
    (f64, "c", $size: tt) => {
        tuple_from_array!(C_ARRAY_F64, $size)
    };
}

macro_rules! tuple_from_array {
    ($arr: expr, 1) => {
        ($arr[0],)
    };
    ($arr: expr, 2) => {
        ($arr[0], $arr[1])
    };
    ($arr: expr, 3) => {
        ($arr[0], $arr[1], $arr[2])
    };
    ($arr: expr, 4) => {
        ($arr[0], $arr[1], $arr[2], $arr[3])
    };
    ($arr: expr, 5) => {
        ($arr[0], $arr[1], $arr[2], $arr[3], $arr[4])
    };
    ($arr: expr, 6) => {
        ($arr[0], $arr[1], $arr[2], $arr[3], $arr[4], $arr[5])
    };
    ($arr: expr, 7) => {
        (
            $arr[0], $arr[1], $arr[2], $arr[3], $arr[4], $arr[5], $arr[6],
        )
    };
    ($arr: expr, 8) => {
        (
            $arr[0], $arr[1], $arr[2], $arr[3], $arr[4], $arr[5], $arr[6], $arr[7],
        )
    };
    ($arr: expr, 9) => {
        (
            $arr[0], $arr[1], $arr[2], $arr[3], $arr[4], $arr[5], $arr[6], $arr[7], $arr[8],
        )
    };
    ($arr: expr, 10) => {
        (
            $arr[0], $arr[1], $arr[2], $arr[3], $arr[4], $arr[5], $arr[6], $arr[7], $arr[8],
            $arr[9],
        )
    };
    ($arr: expr, 11) => {
        (
            $arr[0], $arr[1], $arr[2], $arr[3], $arr[4], $arr[5], $arr[6], $arr[7], $arr[8],
            $arr[9], $arr[10],
        )
    };
    ($arr: expr, 12) => {
        (
            $arr[0], $arr[1], $arr[2], $arr[3], $arr[4], $arr[5], $arr[6], $arr[7], $arr[8],
            $arr[9], $arr[10], $arr[11],
        )
    };
}

macro_rules! impl_test {
    ($macro: ident, $type:ident, $size: tt) => {
        $macro!($type, $size);
    };
    ($macro: ident, $type:ident $( $typeTail:ident )+, $size:tt $( $sizeTail:tt )+) => {
        impl_test!($macro, $( $typeTail )+, $( $sizeTail )+);
        $macro!($type $( $typeTail )+, $size);
    };
}

macro_rules! impl_test_f32 {
    ($( $type: ident )+, $size: tt) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_tuple $size _f32>]() {
                let a: ($($type,)+) = get_value!(f32, "a", $size);
                let b: ($($type,)+) = get_value!(f32, "b", $size);
                assert_ne!(a, b);

                assert!(!a.nearly_eq_eps(&b, 0.0000007));
                assert!(!b.nearly_eq_eps(&a, 0.0000007));

                assert!(a.nearly_eq_eps(&b, 0.0000009));
                assert!(b.nearly_eq_eps(&a, 0.0000009));

                assert!(a.nearly_eq_eps(&b, 0.0000011));
                assert!(b.nearly_eq_eps(&a, 0.0000011));
            }

            #[test]
            fn [<nearly_ne_eps_tuple $size _f32>]() {
                let a: ($($type,)+) = get_value!(f32, "a", $size);
                let b: ($($type,)+) = get_value!(f32, "b", $size);
                assert_ne!(a, b);

                assert!(a.nearly_ne_eps(&b, 0.0000007));
                assert!(b.nearly_ne_eps(&a, 0.0000007));

                assert!(!a.nearly_ne_eps(&b, 0.0000009));
                assert!(!b.nearly_ne_eps(&a, 0.0000009));

                assert!(!a.nearly_ne_eps(&b, 0.0000011));
                assert!(!b.nearly_ne_eps(&a, 0.0000011));
            }

            #[test]
            fn [<nearly_eq_ulps_tuple $size _f32>]() {
                let a: ($($type,)+) = get_value!(f32, "a", $size);
                let b: ($($type,)+) = get_value!(f32, "b", $size);
                assert_ne!(a, b);

                assert!(!a.nearly_eq_ulps(&b, 6));
                assert!(!b.nearly_eq_ulps(&a, 6));

                assert!(a.nearly_eq_ulps(&b, 7));
                assert!(b.nearly_eq_ulps(&a, 7));

                assert!(a.nearly_eq_ulps(&b, 8));
                assert!(b.nearly_eq_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_ne_ulps_tuple $size _f32>]() {
                let a: ($($type,)+) = get_value!(f32, "a", $size);
                let b: ($($type,)+) = get_value!(f32, "b", $size);
                assert_ne!(a, b);

                assert!(a.nearly_ne_ulps(&b, 6));
                assert!(b.nearly_ne_ulps(&a, 6));

                assert!(!a.nearly_ne_ulps(&b, 7));
                assert!(!b.nearly_ne_ulps(&a, 7));

                assert!(!a.nearly_ne_ulps(&b, 8));
                assert!(!b.nearly_ne_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_eq_tol_tuple $size _f32>]() {
                let a: ($($type,)+) = get_value!(f32, "a", $size);
                let b: ($($type,)+) = get_value!(f32, "b", $size);
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
            fn [<nearly_ne_tol_tuple $size _f32>]() {
                let a: ($($type,)+) = get_value!(f32, "a", $size);
                let b: ($($type,)+) = get_value!(f32, "b", $size);
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
            fn [<nearly_eq_tuple $size _f32>]() {
                {
                    let a: ($($type,)+) = get_value!(f32, "a", $size);
                    let b: ($($type,)+) = get_value!(f32, "b", $size);
                    assert_ne!(a, b);

                    assert!(a.nearly_eq(&b));
                    assert!(b.nearly_eq(&a));
                }
                {
                    let a: ($($type,)+) = get_value!(f32, "a", $size);
                    let c: ($($type,)+) = get_value!(f32, "c", $size);
                    assert_ne!(a, c);

                    assert!(!a.nearly_eq(&c));
                    assert!(!c.nearly_eq(&a));
                }
            }

            #[test]
            fn [<nearly_ne_tuple $size _f32>]() {
                {
                    let a: ($($type,)+) = get_value!(f32, "a", $size);
                    let b: ($($type,)+) = get_value!(f32, "b", $size);
                    assert_ne!(a, b);

                    assert!(!a.nearly_ne(&b));
                    assert!(!b.nearly_ne(&a));
                }
                {
                    let a: ($($type,)+) = get_value!(f32, "a", $size);
                    let c: ($($type,)+) = get_value!(f32, "c", $size);
                    assert_ne!(a, c);

                    assert!(a.nearly_ne(&c));
                    assert!(c.nearly_ne(&a));
                }
            }
        }
    }
}

macro_rules! impl_test_f64 {
    ($( $type: ident )+, $size: tt) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_tuple $size _f64>]() {
                let a: ($($type,)+) = get_value!(f64, "a", $size);
                let b: ($($type,)+) = get_value!(f64, "b", $size);
                assert_ne!(a, b);

                assert!(!a.nearly_eq_eps(&b, 0.000000000000001));
                assert!(!b.nearly_eq_eps(&a, 0.000000000000001));

                assert!(a.nearly_eq_eps(&b, 0.0000000000000016));
                assert!(b.nearly_eq_eps(&a, 0.0000000000000016));

                assert!(a.nearly_eq_eps(&b, 0.000000000000002));
                assert!(b.nearly_eq_eps(&a, 0.000000000000002));
            }

            #[test]
            fn [<nearly_ne_eps_tuple $size _f64>]() {
                let a: ($($type,)+) = get_value!(f64, "a", $size);
                let b: ($($type,)+) = get_value!(f64, "b", $size);
                assert_ne!(a, b);

                assert!(a.nearly_ne_eps(&b, 0.000000000000001));
                assert!(b.nearly_ne_eps(&a, 0.000000000000001));

                assert!(!a.nearly_ne_eps(&b, 0.0000000000000016));
                assert!(!b.nearly_ne_eps(&a, 0.0000000000000016));

                assert!(!a.nearly_ne_eps(&b, 0.000000000000002));
                assert!(!b.nearly_ne_eps(&a, 0.000000000000002));
            }

            #[test]
            fn [<nearly_eq_ulps_tuple $size _f64>]() {
                let a: ($($type,)+) = get_value!(f64, "a", $size);
                let b: ($($type,)+) = get_value!(f64, "b", $size);
                assert_ne!(a, b);

                assert!(!a.nearly_eq_ulps(&b, 6));
                assert!(!b.nearly_eq_ulps(&a, 6));

                assert!(a.nearly_eq_ulps(&b, 7));
                assert!(b.nearly_eq_ulps(&a, 7));

                assert!(a.nearly_eq_ulps(&b, 8));
                assert!(b.nearly_eq_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_ne_ulps_tuple $size _f64>]() {
                let a: ($($type,)+) = get_value!(f64, "a", $size);
                let b: ($($type,)+) = get_value!(f64, "b", $size);
                assert_ne!(a, b);

                assert!(a.nearly_ne_ulps(&b, 6));
                assert!(b.nearly_ne_ulps(&a, 6));

                assert!(!a.nearly_ne_ulps(&b, 7));
                assert!(!b.nearly_ne_ulps(&a, 7));

                assert!(!a.nearly_ne_ulps(&b, 8));
                assert!(!b.nearly_ne_ulps(&a, 8));
            }

            #[test]
            fn [<nearly_eq_tol_tuple $size _f64>]() {
                let a: ($($type,)+) = get_value!(f64, "a", $size);
                let b: ($($type,)+) = get_value!(f64, "b", $size);
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
            fn [<nearly_ne_tol_tuple $size _f64>]() {
                let a: ($($type,)+) = get_value!(f64, "a", $size);
                let b: ($($type,)+) = get_value!(f64, "b", $size);
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
            fn [<nearly_eq_tuple $size _f64>]() {
                {
                    let a: ($($type,)+) = get_value!(f64, "a", $size);
                    let b: ($($type,)+) = get_value!(f64, "b", $size);
                    assert_ne!(a, b);

                    assert!(a.nearly_eq(&b));
                    assert!(b.nearly_eq(&a));
                }
                {
                    let a: ($($type,)+) = get_value!(f64, "a", $size);
                    let c: ($($type,)+) = get_value!(f64, "c", $size);
                    assert_ne!(a, c);

                    assert!(!a.nearly_eq(&c));
                    assert!(!c.nearly_eq(&a));
                }
            }

            #[test]
            fn [<nearly_ne_tuple $size _f64>]() {
                {
                    let a: ($($type,)+) = get_value!(f64, "a", $size);
                    let b: ($($type,)+) = get_value!(f64, "b", $size);
                    assert_ne!(a, b);

                    assert!(!a.nearly_ne(&b));
                    assert!(!b.nearly_ne(&a));
                }
                {
                    let a: ($($type,)+) = get_value!(f64, "a", $size);
                    let c: ($($type,)+) = get_value!(f64, "c", $size);
                    assert_ne!(a, c);

                    assert!(a.nearly_ne(&c));
                    assert!(c.nearly_ne(&a));
                }
            }
        }
    }
}

impl_test!(
    impl_test_f32,
    f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32,
    12 11 10 9 8 7 6 5 4 3 2 1);

impl_test!(
    impl_test_f64,
    f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64,
    12 11 10 9 8 7 6 5 4 3 2 1);
