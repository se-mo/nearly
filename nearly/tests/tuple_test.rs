use mockall::predicate::eq;
use mockall::Sequence;
use nearly::{NearlyEqEps, NearlyEqUlps};
use paste::paste;

mod common;
use common::{MockLhs, Rhs};

macro_rules! lhs_type {
    (1) => {
        (MockLhs,)
    };
    (2) => {
        (MockLhs, MockLhs)
    };
    (3) => {
        (MockLhs, MockLhs, MockLhs)
    };
    (4) => {
        (MockLhs, MockLhs, MockLhs, MockLhs)
    };
    (5) => {
        (MockLhs, MockLhs, MockLhs, MockLhs, MockLhs)
    };
    (6) => {
        (MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs)
    };
    (7) => {
        (
            MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs,
        )
    };
    (8) => {
        (
            MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs,
        )
    };
    (9) => {
        (
            MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs,
        )
    };
    (10) => {
        (
            MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs,
            MockLhs,
        )
    };
    (11) => {
        (
            MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs,
            MockLhs, MockLhs,
        )
    };
    (12) => {
        (
            MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs, MockLhs,
            MockLhs, MockLhs, MockLhs,
        )
    };
}

macro_rules! rhs_type {
    (1) => {
        (Rhs,)
    };
    (2) => {
        (Rhs, Rhs)
    };
    (3) => {
        (Rhs, Rhs, Rhs)
    };
    (4) => {
        (Rhs, Rhs, Rhs, Rhs)
    };
    (5) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs)
    };
    (6) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs, Rhs)
    };
    (7) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs)
    };
    (8) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs)
    };
    (9) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs)
    };
    (10) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs)
    };
    (11) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs)
    };
    (12) => {
        (Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs, Rhs)
    };
}

macro_rules! lhs_value {
    (1) => {
        (MockLhs::new(),)
    };
    (2) => {
        (MockLhs::new(), MockLhs::new())
    };
    (3) => {
        (MockLhs::new(), MockLhs::new(), MockLhs::new())
    };
    (4) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (5) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (6) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (7) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (8) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (9) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (10) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (11) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
    (12) => {
        (
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
            MockLhs::new(),
        )
    };
}

macro_rules! rhs_value {
    (1) => {
        (Rhs(0),)
    };
    (2) => {
        (Rhs(0), Rhs(1))
    };
    (3) => {
        (Rhs(0), Rhs(1), Rhs(2))
    };
    (4) => {
        (Rhs(0), Rhs(1), Rhs(2), Rhs(3))
    };
    (5) => {
        (Rhs(0), Rhs(1), Rhs(2), Rhs(3), Rhs(4))
    };
    (6) => {
        (Rhs(0), Rhs(1), Rhs(2), Rhs(3), Rhs(4), Rhs(5))
    };
    (7) => {
        (Rhs(0), Rhs(1), Rhs(2), Rhs(3), Rhs(4), Rhs(5), Rhs(6))
    };
    (8) => {
        (
            Rhs(0),
            Rhs(1),
            Rhs(2),
            Rhs(3),
            Rhs(4),
            Rhs(5),
            Rhs(6),
            Rhs(7),
        )
    };
    (9) => {
        (
            Rhs(0),
            Rhs(1),
            Rhs(2),
            Rhs(3),
            Rhs(4),
            Rhs(5),
            Rhs(6),
            Rhs(7),
            Rhs(8),
        )
    };
    (10) => {
        (
            Rhs(0),
            Rhs(1),
            Rhs(2),
            Rhs(3),
            Rhs(4),
            Rhs(5),
            Rhs(6),
            Rhs(7),
            Rhs(8),
            Rhs(9),
        )
    };
    (11) => {
        (
            Rhs(0),
            Rhs(1),
            Rhs(2),
            Rhs(3),
            Rhs(4),
            Rhs(5),
            Rhs(6),
            Rhs(7),
            Rhs(8),
            Rhs(9),
            Rhs(10),
        )
    };
    (12) => {
        (
            Rhs(0),
            Rhs(1),
            Rhs(2),
            Rhs(3),
            Rhs(4),
            Rhs(5),
            Rhs(6),
            Rhs(7),
            Rhs(8),
            Rhs(9),
            Rhs(10),
            Rhs(11),
        )
    };
}

macro_rules! expect_call {
    ($tuple: ident, $seq: ident, $func: ident, $ret: expr, $tol: expr, $idx: tt) => {
        expect_call!(@impl $tuple, $seq, $func, $ret, $tol, $idx);
    };
    ($tuple: ident, $seq: ident, $func: ident, $ret: expr, $tol: expr, $idx: tt $( $idx_tail: tt )+) => {
        expect_call!($tuple, $seq, $func, $ret, $tol, $( $idx_tail )+);
        expect_call!(@impl $tuple, $seq, $func, $ret, $tol, $idx);
    };
    (@impl $tuple: ident, $seq: ident, $func: ident, $ret: expr, $tol: expr, $( $idx: tt )+) => {
        $($tuple.$idx.$func()
            .with(eq(Rhs($idx)), eq($tol))
            .times(1)
            .in_sequence(&mut $seq)
            .return_const($ret))+;
    }
}

macro_rules! expect_no_call {
    ($tuple: ident, $func: ident, $idx: tt) => {
        expect_no_call!(@impl $tuple, $func, $idx);
    };
    ($tuple: ident, $func: ident, $idx: tt $( $idx_tail: tt )+) => {
        expect_no_call!($tuple, $func, $( $idx_tail )+);
        expect_no_call!(@impl $tuple, $func, $idx);
    };
    (@impl $tuple: ident, $func: ident, $( $idx: tt )+) => {
        $($tuple.$idx.$func().times(0);)+
    }
}

macro_rules! checkpoint {
    ($tuple: ident, $idx: tt) => {
        checkpoint!(@impl $tuple, $idx);
    };
    ($tuple: ident, $idx: tt $( $idx_tail: tt )+) => {
        checkpoint!($tuple, $( $idx_tail )+);
        checkpoint!(@impl $tuple, $idx);
    };
    (@impl $tuple: ident, $( $idx: tt )+) => {
        $($tuple.$idx.checkpoint();)+
    }
}

macro_rules! impl_test {
    ($func: ident, $tol: expr) => {
        paste! {
            #[test]
            fn [<$func _tuple_1>]() {
                let mut a: lhs_type!(1) = lhs_value!(1);
                let b: rhs_type!(1) = rhs_value!(1);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0);
            }

            #[test]
            fn [<$func _tuple_2>]() {
                let mut a: lhs_type!(2) = lhs_value!(2);
                let b: rhs_type!(2) = rhs_value!(2);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1);
            }

            #[test]
            fn [<$func _tuple_3>]() {
                let mut a: lhs_type!(3) = lhs_value!(3);
                let b: rhs_type!(3) = rhs_value!(3);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2);
            }

            #[test]
            fn [<$func _tuple_4>]() {
                let mut a: lhs_type!(4) = lhs_value!(4);
                let b: rhs_type!(4) = rhs_value!(4);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3);
            }

            #[test]
            fn [<$func _tuple_5>]() {
                let mut a: lhs_type!(5) = lhs_value!(5);
                let b: rhs_type!(5) = rhs_value!(5);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4);
            }

            #[test]
            fn [<$func _tuple_6>]() {
                let mut a: lhs_type!(6) = lhs_value!(6);
                let b: rhs_type!(6) = rhs_value!(6);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4 5);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 5);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 5);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4 5);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4 5);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4 5);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4 5);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5);
            }

            #[test]
            fn [<$func _tuple_7>]() {
                let mut a: lhs_type!(7) = lhs_value!(7);
                let b: rhs_type!(7) = rhs_value!(7);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4 5 6);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 6);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 6);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 5);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 5 6);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4 5 6);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4 5 6);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4 5 6);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4 5 6);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6);
            }

            #[test]
            fn [<$func _tuple_8>]() {
                let mut a: lhs_type!(8) = lhs_value!(8);
                let b: rhs_type!(8) = rhs_value!(8);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4 5 6 7);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 7);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 7);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 6);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 6 7);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 5);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 5 6 7);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4 5 6 7);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4 5 6 7);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4 5 6 7);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4 5 6 7);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7);
            }

            #[test]
            fn [<$func _tuple_9>]() {
                let mut a: lhs_type!(9) = lhs_value!(9);
                let b: rhs_type!(9) = rhs_value!(9);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4 5 6 7 8);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 8);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 7);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 7 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 6);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 6 7 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 5);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 5 6 7 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4 5 6 7 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4 5 6 7 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4 5 6 7 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4 5 6 7 8);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8);
            }

            #[test]
            fn [<$func _tuple_10>]() {
                let mut a: lhs_type!(10) = lhs_value!(10);
                let b: rhs_type!(10) = rhs_value!(10);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4 5 6 7 8 9);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 9);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7 8);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 8);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 7);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 7 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 6);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 6 7 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 5);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 5 6 7 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4 5 6 7 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4 5 6 7 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4 5 6 7 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4 5 6 7 8 9);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9);
            }

            #[test]
            fn [<$func _tuple_11>]() {
                let mut a: lhs_type!(11) = lhs_value!(11);
                let b: rhs_type!(11) = rhs_value!(11);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4 5 6 7 8 9 10);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 10);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7 8 9);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 9);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7 8);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 8);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 7);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 7 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 6);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 6 7 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 5);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 5 6 7 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4 5 6 7 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4 5 6 7 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4 5 6 7 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4 5 6 7 8 9 10);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10);
            }

            #[test]
            fn [<$func _tuple_12>]() {
                let mut a: lhs_type!(12) = lhs_value!(12);
                let b: rhs_type!(12) = rhs_value!(12);
                let mut seq = Sequence::new();

                expect_call!(a, seq, [<expect_$func>], true, $tol, 0 1 2 3 4 5 6 7 8 9 10 11);
                assert!(a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], false, $tol, 11);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7 8 9 10);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 10);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7 8 9);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 9);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7 8);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 8);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6 7);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 7);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5 6);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 7 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 6);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4 5);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 6 7 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 5);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3 4);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 5 6 7 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 4);
                expect_no_call!(a, [<expect_$func>], 0 1 2 3);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 4 5 6 7 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 3);
                expect_no_call!(a, [<expect_$func>], 0 1 2);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 3 4 5 6 7 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 2);
                expect_no_call!(a, [<expect_$func>], 0 1);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 2 3 4 5 6 7 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 1);
                expect_no_call!(a, [<expect_$func>], 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);

                expect_call!(a, seq, [<expect_$func>], true, $tol, 1 2 3 4 5 6 7 8 9 10 11);
                expect_call!(a, seq, [<expect_$func>], false, $tol, 0);
                assert!(!a.$func(&b, $tol));
                checkpoint!(a, 0 1 2 3 4 5 6 7 8 9 10 11);
            }
        }
    };
}

impl_test!(nearly_eq_eps, 0.1);
impl_test!(nearly_eq_ulps, 5);
