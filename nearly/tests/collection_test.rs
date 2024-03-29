use mockall::predicate::eq;
use mockall::Sequence;
use nearly::{
    NearlyEqEps, NearlyEqTol, NearlyEqUlps, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps, Tolerance,
};
use paste::paste;

mod common;
use common::{MockLhs, Rhs};

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
    ($lhs: tt, $rhs: tt) => {
        impl_test_fn!($lhs, $rhs, eq);
        impl_test_fn!($lhs, $rhs, lt);
        impl_test_fn!($lhs, $rhs, le);
        impl_test_fn!($lhs, $rhs, gt);
        impl_test_fn!($lhs, $rhs, ge);
    };
}

macro_rules! impl_test_fn {
    (array, array, $fn: ident) => {
        impl_test_same_length!(array, array, $fn);
    };
    ($lhs: tt, $rhs: tt, $fn: ident) => {
        impl_test_same_length!($lhs, $rhs, $fn);
        impl_test_different_length!($lhs, $rhs, $fn);
    };
}

macro_rules! impl_test_same_length {
    ($lhs: tt, $rhs: tt, $fn: ident) => {
        paste! {
            #[test]
            fn [<nearly_ $fn _eps_ $lhs _ $rhs>]() {
                #[allow(unused_mut)]
                let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                let b: rhs_type!($rhs)  = rhs_value!($rhs);

                let mut seq = Sequence::new();

                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(11)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _eps>](&b, &0.1));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _eps>]().times(0);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _eps>]()
                    .with(eq(Rhs(11)), eq(0.1))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1));
            }

            #[test]
            fn [<nearly_ $fn _ulps_ $lhs _ $rhs>]() {
                #[allow(unused_mut)]
                let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                let b: rhs_type!($rhs)  = rhs_value!($rhs);

                let mut seq = Sequence::new();

                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(11)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _ulps>](&b, &5));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _ulps>]()
                    .with(eq(Rhs(11)), eq(5))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5));
            }

            #[test]
            fn [<nearly_ $fn _tol_ $lhs _ $rhs>]() {
                #[allow(unused_mut)]
                let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                let b: rhs_type!($rhs)  = rhs_value!($rhs);

                let mut seq = Sequence::new();

                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(3)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(7)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(11)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _tol>](&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(3)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _tol>]().times(0);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(3)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(7)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));

                checkpoint!(a);
                get_element!(a, 0, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(3)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 1, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(7)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(true);
                get_element!(a, 2, $lhs).[<expect_nearly_ $fn _tol>]()
                    .with(eq(Rhs(11)), eq(Tolerance::<MockLhs, Rhs>::new(0.1, 5)))
                    .times(1)
                    .in_sequence(&mut seq)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<MockLhs, Rhs>::new(0.1, 5)));
            }
        }
    };
}

macro_rules! impl_test_different_length {
    ($lhs: tt, $rhs: tt, $fn: ident) => {
        paste! {
            #[test]
            fn [<nearly_ $fn _eps_different_length_ $lhs _ $rhs>]() {
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                    let b: rhs_type_short!($rhs)  = rhs_value_short!($rhs);

                    assert!(a.len() > b.len());

                    get_element!(a, 0, $lhs).[<expect_nearly_ $fn _eps>]().times(0);
                    get_element!(a, 1, $lhs).[<expect_nearly_ $fn _eps>]().times(0);
                    get_element!(a, 2, $lhs).[<expect_nearly_ $fn _eps>]().times(0);

                    assert!(!a.[<nearly_ $fn _eps>](&b, &0.1));
                }
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type_short!($lhs) = lhs_value_short!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value!($rhs);

                    assert!(a.len() < b.len());

                    get_element!(a, 0, $lhs).[<expect_nearly_ $fn _eps>]().times(0);
                    get_element!(a, 1, $lhs).[<expect_nearly_ $fn _eps>]().times(0);

                    assert!(!a.[<nearly_ $fn _eps>](&b, &0.1));
                }
            }

            #[test]
            fn [<nearly_ $fn _ulps_different_length_ $lhs _ $rhs>]() {
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                    let b: rhs_type_short!($rhs)  = rhs_value_short!($rhs);

                    assert!(a.len() > b.len());

                    get_element!(a, 0, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);
                    get_element!(a, 1, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);
                    get_element!(a, 2, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);

                    assert!(!a.[<nearly_ $fn _ulps>](&b, &5));
                }
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type_short!($lhs) = lhs_value_short!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value!($rhs);

                    assert!(a.len() < b.len());

                    get_element!(a, 0, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);
                    get_element!(a, 1, $lhs).[<expect_nearly_ $fn _ulps>]().times(0);

                    assert!(!a.[<nearly_ $fn _ulps>](&b, &5));
                }
            }

            #[test]
            fn [<nearly_ $fn _tol_different_length_ $lhs _ $rhs>]() {
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                    let b: rhs_type_short!($rhs)  = rhs_value_short!($rhs);

                    assert!(a.len() > b.len());

                    get_element!(a, 0, $lhs).[<expect_nearly_ $fn _tol>]().times(0);
                    get_element!(a, 1, $lhs).[<expect_nearly_ $fn _tol>]().times(0);
                    get_element!(a, 2, $lhs).[<expect_nearly_ $fn _tol>]().times(0);

                    assert!(!a.[<nearly_ $fn _eps>](&b, &0.1));
                }
                {
                    #[allow(unused_mut)]
                    let mut a: lhs_type_short!($lhs) = lhs_value_short!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value!($rhs);

                    assert!(a.len() < b.len());

                    get_element!(a, 0, $lhs).[<expect_nearly_ $fn _tol>]().times(0);
                    get_element!(a, 1, $lhs).[<expect_nearly_ $fn _tol>]().times(0);

                    assert!(!a.[<nearly_ $fn _eps>](&b, &0.1));
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
