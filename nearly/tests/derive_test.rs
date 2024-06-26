use mockall::mock;
use mockall::predicate::{always, eq};
use paste::paste;

mod common;

macro_rules! impl_test {
    ($fn: ident) => {
        paste! {
            #[test]
            fn [<derive_nearly_ $fn _named_struct_same_type>]() {
                let mut a = NamedStructSameType {
                    x: MockA::new(),
                    y: MockA::new(),
                    z: MockA::new(),
                };
                let b = NamedStructSameType {
                    x: MockA::new(),
                    y: MockA::new(),
                    z: MockA::new(),
                };

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn>](&b));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.z.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn>](&b));
            }

            #[test]
            fn [<derive_nearly_ $fn _unnamed_struct_same_type>]() {
                let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
                let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn>](&b));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _tol>]().times(0);
                a.2.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.2.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn>](&b));
            }

            #[test]
            fn [<derive_nearly_ $fn _named_struct_different_type>]() {
                let mut a = NamedStructDifferentType {
                    x: MockA::new(),
                    y: MockB::new(),
                    z: NamedPair {
                        a: MockA::new(),
                        b: MockB::new(),
                    },
                };
                let b = NamedStructDifferentType {
                    x: MockA::new(),
                    y: MockB::new(),
                    z: NamedPair {
                        a: MockA::new(),
                        b: MockB::new(),
                    },
                };

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn>](&b));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.a.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.b.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(false);
                a.z.a.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.b.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.z.b.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn>](&b));
            }

            #[test]
            fn [<derive_nearly_ $fn _unnamed_struct_different_type>]() {
                let mut a = UnnamedStructDifferentType(
                    MockA::new(),
                    MockB::new(),
                    UnnamedPair(MockA::new(), MockB::new()),
                );
                let b = UnnamedStructDifferentType(
                    MockA::new(),
                    MockB::new(),
                    UnnamedPair(MockA::new(), MockB::new()),
                );

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn>](&b));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _tol>]().times(0);
                a.2 .0.[<expect_nearly_ $fn _tol>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(false);
                a.2 .0.[<expect_nearly_ $fn _tol>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                a.2 .1.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn>](&b));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn>](&b));
            }

            #[test]
            fn [<derive_nearly_ $fn _unit_struct>]() {
                let a = UnitStruct;
                let b = UnitStruct;

                assert!(a.[<nearly_ $fn>](&b));
            }

            #[test]
            fn [<derive_nearly_ $fn _enum_same_type>]() {
                let mut a = EnumSameType::X(MockA::new());
                let mut b = EnumSameType::X(MockA::new());
                let c = EnumSameType::Y;
                let d = EnumSameType::Y;
                let mut e = EnumSameType::Z(MockA::new());
                let f = EnumSameType::Z(MockA::new());

                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn>](&b));

                inner_value!(a, EnumSameType::X).checkpoint();
                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn>](&b));

                inner_value!(b, EnumSameType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn>](&c));
                assert!(c.[<nearly_ $fn>](&d));
                assert!(!b.[<nearly_ $fn>](&e));
                assert!(!c.[<nearly_ $fn>](&e));

                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn>](&f));

                inner_value!(e, EnumSameType::Z).checkpoint();
                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn>](&f));
            }

            #[test]
            fn [<derive_nearly_ $fn _enum_different_type>]() {
                let mut a = EnumDifferentType::X(MockA::new());
                let mut b = EnumDifferentType::X(MockA::new());
                let c = EnumDifferentType::Y;
                let d = EnumDifferentType::Y;
                let mut e = EnumDifferentType::Z(MockB::new());
                let f = EnumDifferentType::Z(MockB::new());

                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn>](&b));

                inner_value!(a, EnumDifferentType::X).checkpoint();
                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn>](&b));

                inner_value!(b, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn>](&c));
                assert!(c.[<nearly_ $fn>](&d));
                assert!(!b.[<nearly_ $fn>](&e));
                assert!(!c.[<nearly_ $fn>](&e));

                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn>](&f));

                inner_value!(e, EnumDifferentType::Z).checkpoint();
                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn>](&f));
            }
        }
    };
}

mod nearly_eq {
    use super::*;
    use nearly::{
        EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps,
        Tolerance, UlpsTolerance, UlpsToleranceType,
    };

    mock!(
        pub A{}

        impl EpsTolerance for A {
            type T = f32;
            const DEFAULT: f32 = 0.5;
        }

        impl UlpsTolerance for A {
            type T = i32;
            const DEFAULT: i32 = 7;
        }

        impl NearlyEqEps for A {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for A {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for A {
            fn nearly_eq_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
        }

        impl NearlyEq for A {
            fn nearly_eq(&self, other: &Self) -> bool;
        }
    );

    mock!(
        pub B{}

        impl EpsTolerance for B {
            type T = f64;
            const DEFAULT: f64 = 0.05;
        }

        impl UlpsTolerance for B {
            type T = i64;
            const DEFAULT: i64 = 13;
        }

        impl NearlyEqEps for B {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for B {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for B {
            fn nearly_eq_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
        }

        impl NearlyEq for B {
            fn nearly_eq(&self, other: &Self) -> bool;
        }
    );

    #[derive(NearlyEq)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEq)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEq)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEq)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEq)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEq)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEq)]
    struct UnitStruct;

    #[derive(NearlyEq)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEq)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    impl_test!(eq);
}

mod nearly_ord {
    use super::*;
    use nearly::{
        EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps,
        NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps, Tolerance, UlpsTolerance,
        UlpsToleranceType,
    };

    mock!(
        pub A{}

        impl EpsTolerance for A {
            type T = f32;
            const DEFAULT: f32 = 0.5;
        }

        impl UlpsTolerance for A {
            type T = i32;
            const DEFAULT: i32 = 7;
        }

        impl NearlyEqEps for A {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for A {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for A {
            fn nearly_eq_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
        }

        impl NearlyEq for A {
            fn nearly_eq(&self, other: &Self) -> bool;
        }

        impl NearlyOrdEps for A {
            fn nearly_lt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_le_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_gt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_ge_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdUlps for A {
            fn nearly_lt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_le_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_gt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_ge_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdTol for A {
            fn nearly_lt_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
            fn nearly_le_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
            fn nearly_gt_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
            fn nearly_ge_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
        }

        impl NearlyOrd for A {
            fn nearly_lt(&self, other: &Self) -> bool;
            fn nearly_le(&self, other: &Self) -> bool;
            fn nearly_gt(&self, other: &Self) -> bool;
            fn nearly_ge(&self, other: &Self) -> bool;
        }
    );

    mock!(
        pub B{}

        impl EpsTolerance for B {
            type T = f64;
            const DEFAULT: f64 = 0.05;
        }

        impl UlpsTolerance for B {
            type T = i64;
            const DEFAULT: i64 = 13;
        }

        impl NearlyEqEps for B {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for B {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for B {
            fn nearly_eq_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
        }

        impl NearlyEq for B {
            fn nearly_eq(&self, other: &Self) -> bool;
        }

        impl NearlyOrdEps for B {
            fn nearly_lt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_le_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_gt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_ge_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdUlps for B {
            fn nearly_lt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_le_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_gt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_ge_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdTol for B {
            fn nearly_lt_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
            fn nearly_le_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
            fn nearly_gt_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
            fn nearly_ge_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool;
        }

        impl NearlyOrd for B {
            fn nearly_lt(&self, other: &Self) -> bool;
            fn nearly_le(&self, other: &Self) -> bool;
            fn nearly_gt(&self, other: &Self) -> bool;
            fn nearly_ge(&self, other: &Self) -> bool;
        }
    );

    #[derive(NearlyEq, NearlyOrd)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEq, NearlyOrd)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEq, NearlyOrd)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEq, NearlyOrd)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEq, NearlyOrd)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEq, NearlyOrd)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEq, NearlyOrd)]
    struct UnitStruct;

    #[derive(NearlyEq, NearlyOrd)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEq, NearlyOrd)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    impl_test!(lt);
    impl_test!(le);
    impl_test!(gt);
    impl_test!(ge);
}
