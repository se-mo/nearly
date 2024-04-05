use mockall::mock;
use mockall::predicate::{always, eq};
use paste::paste;

mod common;

macro_rules! impl_test {
    ($fn: ident) => {
        paste! {
            #[test]
            fn [<derive_nearly_ $fn _tol_named_struct_same_type>]() {
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
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _tol>](&b, &Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                a.z.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));
            }

            #[test]
            fn [<derive_nearly_eq_ $fn _unnamed_struct_same_type>]() {
                let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
                let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _tol>](&b, &Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _tol>]().times(0);
                a.2.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                a.2.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));
            }

            #[test]
            fn [<derive_nearly_ $fn _tol_named_struct_different_type>]() {
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

                let eps = (0.1_f32, 0.01_f64, (0.2_f32, 0.02_f64));
                let ulps = (5_i32, 10_i64, (6_i32, 12_i64));
                let tol = Tolerance::<NamedStructDifferentType>::new(eps, ulps);

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _tol>](&b, &tol));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.a.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.b.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(false);
                a.z.a.[<expect_nearly_ $fn _tol>]().times(0);
                a.z.b.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
                    .times(1)
                    .return_const(false);
                a.z.b.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));
            }

            #[test]
            fn [<derive_nearly_ $fn _tol_unnamed_struct_different_type>]() {
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

                let eps = (0.1_f32, 0.01_f64, (0.2_f32, 0.02_f64));
                let ulps = (5_i32, 10_i64, (6_i32, 12_i64));
                let tol = Tolerance::<UnnamedStructDifferentType>::new(eps, ulps);

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _tol>](&b, &tol));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _tol>]().times(0);
                a.2 .0.[<expect_nearly_ $fn _tol>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(false);
                a.2 .0.[<expect_nearly_ $fn _tol>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
                    .times(1)
                    .return_const(false);
                a.2 .1.[<expect_nearly_ $fn _tol>]().times(0);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));
            }

            #[test]
            fn [<derive_nearly_ $fn _tol_unit_struct>]() {
                let a = UnitStruct;
                let b = UnitStruct;

                assert!(a.[<nearly_ $fn _tol>](&b, &Tolerance::<UnitStruct>::new((), ())));
            }

            #[test]
            fn [<derive_nearly_ $fn _tol_enum_same_type>]() {
                let mut a = EnumSameType::X(MockA::new());
                let mut b = EnumSameType::X(MockA::new());
                let c = EnumSameType::Y;
                let d = EnumSameType::Y;
                let mut e = EnumSameType::Z(MockA::new());
                let f = EnumSameType::Z(MockA::new());

                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn _tol>](&b, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

                inner_value!(a, EnumSameType::X).checkpoint();
                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn _tol>](&b, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

                inner_value!(b, EnumSameType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn _tol>](&c, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
                assert!(c.[<nearly_ $fn _tol>](&d, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
                assert!(!b.[<nearly_ $fn _tol>](&e, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
                assert!(!c.[<nearly_ $fn _tol>](&e, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn _tol>](&f, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

                inner_value!(e, EnumSameType::Z).checkpoint();
                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn _tol>](&f, &Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
            }

            #[test]
            fn [<derive_nearly_ $fn _tol_enum_different_type>]() {
                let mut a = EnumDifferentType::X(MockA::new());
                let mut b = EnumDifferentType::X(MockA::new());
                let c = EnumDifferentType::Y;
                let d = EnumDifferentType::Y;
                let mut e = EnumDifferentType::Z(MockB::new());
                let f = EnumDifferentType::Z(MockB::new());

                let eps = (0.1_f32, 0.01_f64);
                let ulps = (5_i32, 10_i64);
                let tol = Tolerance::<EnumDifferentType>::new(eps, ulps);

                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn _tol>](&b, &tol));

                inner_value!(a, EnumDifferentType::X).checkpoint();
                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn _tol>](&b, &tol));

                inner_value!(b, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _tol>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn _tol>](&c, &tol));
                assert!(c.[<nearly_ $fn _tol>](&d, &tol));
                assert!(!b.[<nearly_ $fn _tol>](&e, &tol));
                assert!(!c.[<nearly_ $fn _tol>](&e, &tol));

                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn _tol>](&f, &tol));

                inner_value!(e, EnumDifferentType::Z).checkpoint();
                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _tol>]()
                    .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn _tol>](&f, &tol));
            }
        }
    }
}

mod nearly_eq {
    use super::*;
    use nearly::{
        EpsTolerance, EpsToleranceType, NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance,
        UlpsTolerance, UlpsToleranceType,
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
    );

    #[derive(NearlyEqTol)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEqTol)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEqTol)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEqTol)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEqTol)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqTol)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEqTol)]
    struct UnitStruct;

    #[derive(NearlyEqTol)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEqTol)]
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
        EpsTolerance, EpsToleranceType, NearlyEqEps, NearlyEqTol, NearlyEqUlps, NearlyOrdEps,
        NearlyOrdTol, NearlyOrdUlps, Tolerance, UlpsTolerance, UlpsToleranceType,
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
    );

    #[derive(NearlyEqTol, NearlyOrdTol)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEqTol, NearlyOrdTol)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEqTol, NearlyOrdTol)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEqTol, NearlyOrdTol)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEqTol, NearlyOrdTol)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqTol, NearlyOrdTol)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEqTol, NearlyOrdTol)]
    struct UnitStruct;

    #[derive(NearlyEqTol, NearlyOrdTol)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEqTol, NearlyOrdTol)]
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
