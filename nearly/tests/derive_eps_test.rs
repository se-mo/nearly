use mockall::mock;
use mockall::predicate::{always, eq};
use paste::paste;

mod common;

macro_rules! impl_test {
    ($fn: ident) => {
        paste! {
            #[test]
            fn [<derive_nearly_ $fn _eps_named_struct_same_type>]() {
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

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _eps>]().times(0);
                a.z.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                a.z.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1_f32));
            }

            #[test]
            fn [<derive_nearly_ $fn _eps_unnamed_struct_same_type>]() {
                let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
                let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _eps>]().times(0);
                a.2.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                a.2.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1_f32));
            }

            #[test]
            fn [<derive_nearly_ $fn _eps_named_struct_different_type>]() {
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

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.2))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.02))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _eps>](&b, &eps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _eps>]().times(0);
                a.z.a.[<expect_nearly_ $fn _eps>]().times(0);
                a.z.b.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(false);
                a.z.a.[<expect_nearly_ $fn _eps>]().times(0);
                a.z.b.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.2))
                    .times(1)
                    .return_const(false);
                a.z.b.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.2))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.02))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));
            }

            #[test]
            fn [<derive_nearly_ $fn _eps_unnamed_struct_different_type>]() {
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

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.2))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.02))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _eps>](&b, &eps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _eps>]().times(0);
                a.2 .0.[<expect_nearly_ $fn _eps>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(false);
                a.2 .0.[<expect_nearly_ $fn _eps>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.2))
                    .times(1)
                    .return_const(false);
                a.2 .1.[<expect_nearly_ $fn _eps>]().times(0);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.2))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.02))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));
            }

            #[test]
            fn [<derive_nearly_ $fn _eps_unit_struct>]() {
                let a = UnitStruct;
                let b = UnitStruct;

                assert!(a.[<nearly_ $fn _eps>](&b, &()));
            }

            #[test]
            fn [<derive_nearly_ $fn _eps_enum_same_type>]() {
                let mut a = EnumSameType::X(MockA::new());
                let mut b = EnumSameType::X(MockA::new());
                let c = EnumSameType::Y;
                let d = EnumSameType::Y;
                let mut e = EnumSameType::Z(MockA::new());
                let f = EnumSameType::Z(MockA::new());

                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                inner_value!(a, EnumSameType::X).checkpoint();
                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn _eps>](&b, &0.1_f32));

                inner_value!(b, EnumSameType::X)
                    .[<expect_nearly_ $fn _eps>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn _eps>](&c, &0.1_f32));
                assert!(c.[<nearly_ $fn _eps>](&d, &0.1_f32));
                assert!(!b.[<nearly_ $fn _eps>](&e, &0.1_f32));
                assert!(!c.[<nearly_ $fn _eps>](&e, &0.1_f32));

                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn _eps>](&f, &0.1_f32));

                inner_value!(e, EnumSameType::Z).checkpoint();
                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn _eps>](&f, &0.1_f32));
            }

            #[test]
            fn [<derive_nearly_ $fn _eps_enum_different_type>]() {
                let mut a = EnumDifferentType::X(MockA::new());
                let mut b = EnumDifferentType::X(MockA::new());
                let c = EnumDifferentType::Y;
                let d = EnumDifferentType::Y;
                let mut e = EnumDifferentType::Z(MockB::new());
                let f = EnumDifferentType::Z(MockB::new());

                let eps = (0.1_f32, 0.01_f64);

                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn _eps>](&b, &eps));

                inner_value!(a, EnumDifferentType::X).checkpoint();
                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.1))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn _eps>](&b, &eps));

                inner_value!(b, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _eps>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn _eps>](&c, &eps));
                assert!(c.[<nearly_ $fn _eps>](&d, &eps));
                assert!(!b.[<nearly_ $fn _eps>](&e, &eps));
                assert!(!c.[<nearly_ $fn _eps>](&e, &eps));

                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn _eps>](&f, &eps));

                inner_value!(e, EnumDifferentType::Z).checkpoint();
                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _eps>]()
                    .with(always(), eq(0.01))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn _eps>](&f, &eps));
            }
        }
    };
}

mod nearly_eq {
    use super::*;
    use nearly::{EpsTolerance, EpsToleranceType, NearlyEqEps};

    mock!(
        pub A{}

        impl EpsTolerance for A {
            type T = f32;
            const DEFAULT: f32 = 0.5;
        }

        impl NearlyEqEps for A {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }
    );

    mock!(
        pub B{}

        impl EpsTolerance for B {
            type T = f64;
            const DEFAULT: f64 = 0.05;
        }

        impl NearlyEqEps for B {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }
    );

    #[derive(NearlyEqEps)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEqEps)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEqEps)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEqEps)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEqEps)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqEps)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEqEps)]
    struct UnitStruct;

    #[derive(NearlyEqEps)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEqEps)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    impl_test!(eq);
}

mod nearly_ord {
    use super::*;
    use nearly::{EpsTolerance, EpsToleranceType, NearlyEqEps, NearlyOrdEps};

    mock!(
        pub A{}

        impl EpsTolerance for A {
            type T = f32;
            const DEFAULT: f32 = 0.5;
        }

        impl NearlyEqEps for A {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdEps for A {
            fn nearly_lt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_le_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_gt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_ge_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }
    );

    mock!(
        pub B{}

        impl EpsTolerance for B {
            type T = f64;
            const DEFAULT: f64 = 0.05;
        }

        impl NearlyEqEps for B {
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdEps for B {
            fn nearly_lt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_le_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_gt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
            fn nearly_ge_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool;
        }
    );

    #[derive(NearlyEqEps, NearlyOrdEps)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEqEps, NearlyOrdEps)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEqEps, NearlyOrdEps)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEqEps, NearlyOrdEps)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEqEps, NearlyOrdEps)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqEps, NearlyOrdEps)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEqEps, NearlyOrdEps)]
    struct UnitStruct;

    #[derive(NearlyEqEps, NearlyOrdEps)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEqEps, NearlyOrdEps)]
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
