#![cfg(feature = "std")]

use mockall::predicate::eq;
use nearly::{NearlyEqEps, NearlyEqUlps};
use paste::paste;
use std::collections::{BTreeMap, HashMap};

mod common;
use common::{MockLhs, Rhs};

macro_rules! checkpoint {
    ($container: expr) => {
        for i in $container.iter_mut() {
            i.1.checkpoint();
        }
    };
}

macro_rules! lhs_value {
    (hashmap) => {
        HashMap::from([
            (0, MockLhs::new()),
            (1, MockLhs::new()),
            (2, MockLhs::new()),
        ])
    };
    (btree_map) => {
        BTreeMap::from([
            (0, MockLhs::new()),
            (1, MockLhs::new()),
            (2, MockLhs::new()),
        ])
    };
}

macro_rules! lhs_value_short {
    (hashmap) => {
        HashMap::from([(0, MockLhs::new()), (1, MockLhs::new())])
    };
    (btree_map) => {
        BTreeMap::from([(0, MockLhs::new()), (1, MockLhs::new())])
    };
}

macro_rules! rhs_value {
    (hashmap) => {
        HashMap::from([(0, Rhs(3)), (1, Rhs(7)), (2, Rhs(11))])
    };
    (btree_map) => {
        BTreeMap::from([(0, Rhs(3)), (1, Rhs(7)), (2, Rhs(11))])
    };
}

macro_rules! rhs_value_short {
    (hashmap) => {
        HashMap::from([(0, Rhs(3)), (1, Rhs(7))])
    };
    (btree_map) => {
        BTreeMap::from([(0, Rhs(3)), (1, Rhs(7))])
    };
}

macro_rules! get_type {
    ($inner: ty, hashmap) => {
        HashMap<i32, $inner>
    };
    ($inner: ty, btree_map) => {
        BTreeMap<i32, $inner>
    }
}

macro_rules! lhs_type {
    ($coll: tt) => {
        get_type!(MockLhs, $coll)
    };
}

macro_rules! rhs_type {
    ($coll: tt) => {
        get_type!(Rhs, $coll)
    };
}

macro_rules! get_element {
    ($container: expr, $idx: expr) => {
        *$container.get_mut(&$idx).expect("Invalid key")
    };
}

macro_rules! impl_test {
    ($lhs: tt, $rhs: tt) => {
        paste! {
            #[test]
            fn [<nearly_eq_eps_ $lhs _ $rhs>]() {
                let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                let b: rhs_type!($rhs)  = rhs_value!($rhs);

                get_element!(a, 0).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .return_const(true);
                get_element!(a, 1).expect_nearly_eq_eps()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .return_const(true);
                get_element!(a, 2).expect_nearly_eq_eps()
                    .with(eq(Rhs(11)), eq(0.1))
                    .times(1)
                    .return_const(true);

                assert!(a.nearly_eq_eps(&b, 0.1));

                checkpoint!(a);
                get_element!(a, 0).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .times(1)
                    .return_const(false);
                get_element!(a, 1).expect_nearly_eq_eps()
                    .with(eq(Rhs(7)), eq(0.1))
                    .return_const(true);
                get_element!(a, 2).expect_nearly_eq_eps()
                    .with(eq(Rhs(11)), eq(0.1))
                    .return_const(true);

                assert!(!a.nearly_eq_eps(&b, 0.1));

                checkpoint!(a);
                get_element!(a, 0).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .return_const(true);
                get_element!(a, 1).expect_nearly_eq_eps()
                    .with(eq(Rhs(7)), eq(0.1))
                    .times(1)
                    .return_const(false);
                get_element!(a, 2).expect_nearly_eq_eps()
                    .with(eq(Rhs(11)), eq(0.1))
                    .return_const(true);

                assert!(!a.nearly_eq_eps(&b, 0.1));

                checkpoint!(a);
                get_element!(a, 0).expect_nearly_eq_eps()
                    .with(eq(Rhs(3)), eq(0.1))
                    .return_const(true);
                get_element!(a, 1).expect_nearly_eq_eps()
                    .with(eq(Rhs(7)), eq(0.1))
                    .return_const(true);
                get_element!(a, 2).expect_nearly_eq_eps()
                    .with(eq(Rhs(11)), eq(0.1))
                    .times(1)
                    .return_const(false);

                assert!(!a.nearly_eq_eps(&b, 0.1));
            }

            #[test]
            fn [<nearly_eq_ulps_ $lhs _ $rhs>]() {
                #[allow(unused_mut)]
                let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                let b: rhs_type!($rhs)  = rhs_value!($rhs);

                get_element!(a, 0).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .return_const(true);
                get_element!(a, 1).expect_nearly_eq_ulps()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .return_const(true);
                get_element!(a, 2).expect_nearly_eq_ulps()
                    .with(eq(Rhs(11)), eq(5))
                    .times(1)
                    .return_const(true);

                assert!(a.nearly_eq_ulps(&b, 5));

                checkpoint!(a);
                get_element!(a, 0).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .times(1)
                    .return_const(false);
                get_element!(a, 1).expect_nearly_eq_ulps()
                    .with(eq(Rhs(7)), eq(5))
                    .return_const(true);
                get_element!(a, 2).expect_nearly_eq_ulps()
                    .with(eq(Rhs(11)), eq(5))
                    .return_const(true);

                assert!(!a.nearly_eq_ulps(&b, 5));

                checkpoint!(a);
                get_element!(a, 0).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .return_const(true);
                get_element!(a, 1).expect_nearly_eq_ulps()
                    .with(eq(Rhs(7)), eq(5))
                    .times(1)
                    .return_const(false);
                get_element!(a, 2).expect_nearly_eq_ulps()
                    .with(eq(Rhs(11)), eq(5))
                    .return_const(true);

                assert!(!a.nearly_eq_ulps(&b, 5));

                checkpoint!(a);
                get_element!(a, 0).expect_nearly_eq_ulps()
                    .with(eq(Rhs(3)), eq(5))
                    .return_const(true);
                get_element!(a, 1).expect_nearly_eq_ulps()
                    .with(eq(Rhs(7)), eq(5))
                    .return_const(true);
                get_element!(a, 2).expect_nearly_eq_ulps()
                    .with(eq(Rhs(11)), eq(5))
                    .times(1)
                    .return_const(false);

                assert!(!a.nearly_eq_ulps(&b, 5));
            }

            #[test]
            fn [<nearly_eq_eps_different_length_ $lhs _ $rhs>]() {
                {
                    let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value_short!($rhs);

                    assert!(a.len() > b.len());

                    get_element!(a, 0).expect_nearly_eq_eps().times(0);
                    get_element!(a, 1).expect_nearly_eq_eps().times(0);
                    get_element!(a, 2).expect_nearly_eq_eps().times(0);

                    assert!(!a.nearly_eq_eps(&b, 0.1));
                }
                {
                    let mut a: lhs_type!($lhs) = lhs_value_short!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value!($rhs);

                    assert!(a.len() < b.len());

                    get_element!(a, 0).expect_nearly_eq_eps().times(0);
                    get_element!(a, 1).expect_nearly_eq_eps().times(0);

                    assert!(!a.nearly_eq_eps(&b, 0.1));
                }
            }

            #[test]
            fn [<nearly_eq_ulps_different_length_ $lhs _ $rhs>]() {
                {
                    let mut a: lhs_type!($lhs) = lhs_value!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value_short!($rhs);

                    assert!(a.len() > b.len());

                    get_element!(a, 0).expect_nearly_eq_ulps().times(0);
                    get_element!(a, 1).expect_nearly_eq_ulps().times(0);
                    get_element!(a, 2).expect_nearly_eq_ulps().times(0);

                    assert!(!a.nearly_eq_ulps(&b, 5));
                }
                {
                    let mut a: lhs_type!($lhs) = lhs_value_short!($lhs);
                    let b: rhs_type!($rhs)  = rhs_value!($rhs);

                    assert!(a.len() < b.len());

                    get_element!(a, 0).expect_nearly_eq_ulps().times(0);
                    get_element!(a, 1).expect_nearly_eq_ulps().times(0);

                    assert!(!a.nearly_eq_ulps(&b, 5));
                }
            }
        }
    };
}

impl_test!(btree_map, btree_map);
impl_test!(hashmap, hashmap);
