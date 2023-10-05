use mockall::predicate::eq;
use mockall::{mock, Sequence};
use nearly::{
    EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance,
    UlpsTolerance, UlpsToleranceType,
};
use paste::paste;

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

macro_rules! checkpoint {
    ($container: expr) => {
        for i in $container.iter_mut() {
            i.checkpoint();
        }
    };
}

macro_rules! lhs_value {
    (array) => {
        [MockLhs::new(), MockLhs::new(), MockLhs::new()]
    };
    (slice) => {
        &mut [MockLhs::new(), MockLhs::new(), MockLhs::new()][0..3]
    };
    (vec) => {
        [MockLhs::new(), MockLhs::new(), MockLhs::new()].into()
    };
    (vec_deque) => {
        [MockLhs::new(), MockLhs::new(), MockLhs::new()].into()
    };
    (linked_list) => {
        LinkedList::from([MockLhs::new(), MockLhs::new(), MockLhs::new()])
    };
}

macro_rules! lhs_value_short {
    (array) => {
        [MockLhs::new(), MockLhs::new()]
    };
    (slice) => {
        &mut [MockLhs::new(), MockLhs::new()][0..2]
    };
    (vec) => {
        [MockLhs::new(), MockLhs::new()].into()
    };
    (vec_deque) => {
        [MockLhs::new(), MockLhs::new()].into()
    };
    (linked_list) => {
        LinkedList::from([MockLhs::new(), MockLhs::new()])
    };
}

macro_rules! rhs_value {
    (array) => {
        [Rhs(3), Rhs(7), Rhs(11)]
    };
    (slice) => {
        &[Rhs(3), Rhs(7), Rhs(11)][0..3]
    };
    (vec) => {
        [Rhs(3), Rhs(7), Rhs(11)].into()
    };
    (vec_deque) => {
        [Rhs(3), Rhs(7), Rhs(11)].into()
    };
    (linked_list) => {
        LinkedList::from([Rhs(3), Rhs(7), Rhs(11)])
    };
}

macro_rules! rhs_value_short {
    (array) => {
        [Rhs(3), Rhs(7)]
    };
    (slice) => {
        &[Rhs(3), Rhs(7)][0..2]
    };
    (vec) => {
        [Rhs(3), Rhs(7)].into()
    };
    (vec_deque) => {
        [Rhs(3), Rhs(7)].into()
    };
    (linked_list) => {
        LinkedList::from([Rhs(3), Rhs(7)])
    };
}

macro_rules! get_type {
    ($inner: ty, array) => {
        [$inner; 3]
    };
    ($inner: ty, vec) => {
        Vec<$inner>
    };
    ($inner: ty, vec_deque) => {
        VecDeque<$inner>
    };
    ($inner: ty, linked_list) => {
        LinkedList<$inner>
    }
}

macro_rules! lhs_type {
    (slice) => {
        &mut [MockLhs]
    };
    ($coll: tt) => {
        get_type!(MockLhs, $coll)
    };
}

macro_rules! lhs_type_short {
    (array) => {
        [MockLhs; 2]
    };
    ($coll: tt) => {
        lhs_type!($coll)
    };
}

macro_rules! rhs_type {
    (slice) => {
        &[Rhs]
    };
    ($coll: tt) => {
        get_type!(Rhs, $coll)
    };
}

macro_rules! rhs_type_short {
    (array) => {
        [Rhs; 2]
    };
    ($coll: tt) => {
        rhs_type!($coll)
    };
}

macro_rules! get_element {
    ($container: expr, $idx: expr, linked_list) => {{
        let mut iter = $container.iter_mut();
        #[allow(clippy::reversed_empty_ranges)]
        for _ in 0..$idx {
            iter.next().expect("No next element");
        }
        iter.next().expect("No next element")
    }};
    ($container: expr, $idx: expr, $_coll: tt) => {
        $container[$idx]
    };
}

macro_rules! impl_test {
    (array, array) => {
        impl_test_same_length!(array, array);
    };
    ($lhs: tt, $rhs: tt) => {
        impl_test_same_length!($lhs, $rhs);
        impl_test_different_length!($lhs, $rhs);
    };
}

macro_rules! impl_test_same_length {
    ($lhs: tt, $rhs: tt) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_ $lhs _ $rhs>]() {
                #[allow(unused_mut)]
                let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                let b: rhs_type!($rhs)  = rhs_value!($rhs);

                let mut seq = Sequence::new();

                get_element!(a, 0, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(11)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);

                assert!(a.nearly_eq_eps(&b, 0.1));

                checkpoint!(a);
                get_element!(a, 0, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 1, $lhs).expect_nearly_eq_eps().times(0);
                get_element!(a, 2, $lhs).expect_nearly_eq_eps().times(0);

                assert!(!a.nearly_eq_eps(&b, 0.1));

                checkpoint!(a);
                get_element!(a, 0, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 2, $lhs).expect_nearly_eq_eps().times(0);

                assert!(!a.nearly_eq_eps(&b, 0.1));

                checkpoint!(a);
                get_element!(a, 0, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).expect_nearly_eq_eps()
                    .with(eq(Rhs(11)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);

                assert!(!a.nearly_eq_eps(&b, 0.1));
            }

            #[test]
            fn [<nearly_eq_ulps_ $lhs _ $rhs>]() {
                #[allow(unused_mut)]
                let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                let b: rhs_type!($rhs)  = rhs_value!($rhs);

                let mut seq = Sequence::new();

                get_element!(a, 0, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(11)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);

                assert!(a.nearly_eq_ulps(&b, 5));

                checkpoint!(a);
                get_element!(a, 0, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 1, $lhs).expect_nearly_eq_ulps().times(0);
                get_element!(a, 2, $lhs).expect_nearly_eq_ulps().times(0);

                assert!(!a.nearly_eq_ulps(&b, 5));

                checkpoint!(a);
                get_element!(a, 0, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 2, $lhs).expect_nearly_eq_ulps().times(0);

                assert!(!a.nearly_eq_ulps(&b, 5));

                checkpoint!(a);
                get_element!(a, 0, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).expect_nearly_eq_ulps()
                    .with(eq(Rhs(11)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);

                assert!(!a.nearly_eq_ulps(&b, 5));
            }
        }
    };
}

macro_rules! impl_test_different_length {
    ($lhs: tt, $rhs: tt) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_different_length_ $lhs _ $rhs>]() {
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                    let b: rhs_type_short!($rhs)  = rhs_value_short!($rhs);

                    assert!(a.len() > b.len());

                    get_element!(a, 0, $lhs).expect_nearly_eq_eps().times(0);
                    get_element!(a, 1, $lhs).expect_nearly_eq_eps().times(0);
                    get_element!(a, 2, $lhs).expect_nearly_eq_eps().times(0);

                    assert!(!a.nearly_eq_eps(&b, 0.1));
                }
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type_short!($lhs) = lhs_value_short!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value!($rhs);

                    assert!(a.len() < b.len());

                    get_element!(a, 0, $lhs).expect_nearly_eq_eps().times(0);
                    get_element!(a, 1, $lhs).expect_nearly_eq_eps().times(0);

                    assert!(!a.nearly_eq_eps(&b, 0.1));
                }
            }

            #[test]
            fn [<nearly_eq_ulps_different_length_ $lhs _ $rhs>]() {
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                    let b: rhs_type_short!($rhs)  = rhs_value_short!($rhs);

                    assert!(a.len() > b.len());

                    get_element!(a, 0, $lhs).expect_nearly_eq_ulps().times(0);
                    get_element!(a, 1, $lhs).expect_nearly_eq_ulps().times(0);
                    get_element!(a, 2, $lhs).expect_nearly_eq_ulps().times(0);

                    assert!(!a.nearly_eq_ulps(&b, 5));
                }
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type_short!($lhs) = lhs_value_short!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value!($rhs);

                    assert!(a.len() < b.len());

                    get_element!(a, 0, $lhs).expect_nearly_eq_ulps().times(0);
                    get_element!(a, 1, $lhs).expect_nearly_eq_ulps().times(0);

                    assert!(!a.nearly_eq_ulps(&b, 5));
                }
            }
        }
    };
}

impl_test!(array, array);
impl_test!(array, slice);
impl_test!(slice, slice);
impl_test!(slice, array);

#[cfg(feature = "std")]
mod std_types {
    use super::*;
    use std::collections::{LinkedList, VecDeque};

    impl_test!(vec, vec);
    impl_test!(vec, vec_deque);
    impl_test!(vec, array);
    impl_test!(vec, slice);
    impl_test!(array, vec);
    impl_test!(slice, vec);

    impl_test!(vec_deque, vec_deque);
    impl_test!(vec_deque, vec);
    impl_test!(vec_deque, array);
    impl_test!(vec_deque, slice);
    impl_test!(array, vec_deque);
    impl_test!(slice, vec_deque);

    impl_test!(linked_list, linked_list);
}
