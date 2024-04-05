use mockall::mock;
use mockall::predicate::{always, eq};
use paste::paste;

mod common;

macro_rules! impl_test {
    ($fn: ident) => {
        paste! {
            #[test]
            fn [<derive_nearly_ $fn _ulps_named_struct_same_type>]() {
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

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _ulps>](&b, &5_i32));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _ulps>]().times(0);
                a.z.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5_i32));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                a.z.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5_i32));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.checkpoint();

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.z.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5_i32));
            }

            #[test]
            fn [<derive_nearly_ $fn _ulps_unnamed_struct_same_type>]() {
                let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
                let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _ulps>](&b, &5_i32));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _ulps>]().times(0);
                a.2.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5_i32));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                a.2.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5_i32));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2.checkpoint();

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.2.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &5_i32));
            }

            #[test]
            fn [<derive_nearly_ $fn _ulps_named_struct_different_type>]() {
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

                let ulps = (5_i32, 10_i64, (6_i32, 12_i64));

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(6))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(12))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                a.y.[<expect_nearly_ $fn _ulps>]().times(0);
                a.z.a.[<expect_nearly_ $fn _ulps>]().times(0);
                a.z.b.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(false);
                a.z.a.[<expect_nearly_ $fn _ulps>]().times(0);
                a.z.b.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(6))
                    .times(1)
                    .return_const(false);
                a.z.b.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.x.checkpoint();
                a.y.checkpoint();
                a.z.a.checkpoint();
                a.z.b.checkpoint();

                a.x.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.y.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(true);
                a.z.a
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(6))
                    .times(1)
                    .return_const(true);
                a.z.b
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(12))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));
            }

            #[test]
            fn [<derive_nearly_ $fn _ulps_unnamed_struct_different_type>]() {
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

                let ulps = (5_i32, 10_i64, (6_i32, 12_i64));

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(6))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(12))
                    .times(1)
                    .return_const(true);

                assert!(a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                a.1.[<expect_nearly_ $fn _ulps>]().times(0);
                a.2 .0.[<expect_nearly_ $fn _ulps>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(false);
                a.2 .0.[<expect_nearly_ $fn _ulps>]().times(0);
                a.2 .1.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(6))
                    .times(1)
                    .return_const(false);
                a.2 .1.[<expect_nearly_ $fn _ulps>]().times(0);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));

                a.0.checkpoint();
                a.1.checkpoint();
                a.2 .0.checkpoint();
                a.2 .1.checkpoint();

                a.0.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                a.1.[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(true);
                a.2 .0
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(6))
                    .times(1)
                    .return_const(true);
                a.2 .1
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(12))
                    .times(1)
                    .return_const(false);

                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));
            }

            #[test]
            fn [<derive_nearly_ $fn _ulps_unit_struct>]() {
                let a = UnitStruct;
                let b = UnitStruct;

                assert!(a.[<nearly_ $fn _ulps>](&b, &()));
            }

            #[test]
            fn [<derive_nearly_ $fn _ulps_enum_same_type>]() {
                let mut a = EnumSameType::X(MockA::new());
                let mut b = EnumSameType::X(MockA::new());
                let c = EnumSameType::Y;
                let d = EnumSameType::Y;
                let mut e = EnumSameType::Z(MockA::new());
                let f = EnumSameType::Z(MockA::new());

                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn _ulps>](&b, &5_i32));

                inner_value!(a, EnumSameType::X).checkpoint();
                inner_value!(a, EnumSameType::X)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn _ulps>](&b, &5_i32));

                inner_value!(b, EnumSameType::X)
                    .[<expect_nearly_ $fn _ulps>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn _ulps>](&c, &5_i32));
                assert!(c.[<nearly_ $fn _ulps>](&d, &5_i32));
                assert!(!b.[<nearly_ $fn _ulps>](&e, &5_i32));
                assert!(!c.[<nearly_ $fn _ulps>](&e, &5_i32));

                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn _ulps>](&f, &5_i32));

                inner_value!(e, EnumSameType::Z).checkpoint();
                inner_value!(e, EnumSameType::Z)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn _ulps>](&f, &5_i32));
            }

            #[test]
            fn [<derive_nearly_ $fn _ulps_enum_different_type>]() {
                let mut a = EnumDifferentType::X(MockA::new());
                let mut b = EnumDifferentType::X(MockA::new());
                let c = EnumDifferentType::Y;
                let d = EnumDifferentType::Y;
                let mut e = EnumDifferentType::Z(MockB::new());
                let f = EnumDifferentType::Z(MockB::new());

                let ulps = (5_i32, 10_i64);

                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(true);
                assert!(a.[<nearly_ $fn _ulps>](&b, &ulps));

                inner_value!(a, EnumDifferentType::X).checkpoint();
                inner_value!(a, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(5))
                    .times(1)
                    .return_const(false);
                assert!(!a.[<nearly_ $fn _ulps>](&b, &ulps));

                inner_value!(b, EnumDifferentType::X)
                    .[<expect_nearly_ $fn _ulps>]()
                    .times(0);

                assert!(!b.[<nearly_ $fn _ulps>](&c, &ulps));
                assert!(c.[<nearly_ $fn _ulps>](&d, &ulps));
                assert!(!b.[<nearly_ $fn _ulps>](&e, &ulps));
                assert!(!c.[<nearly_ $fn _ulps>](&e, &ulps));

                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(true);
                assert!(e.[<nearly_ $fn _ulps>](&f, &ulps));

                inner_value!(e, EnumDifferentType::Z).checkpoint();
                inner_value!(e, EnumDifferentType::Z)
                    .[<expect_nearly_ $fn _ulps>]()
                    .with(always(), eq(10))
                    .times(1)
                    .return_const(false);
                assert!(!e.[<nearly_ $fn _ulps>](&f, &ulps));
            }
        }
    };
}

mod nearly_eq {
    use super::*;
    use nearly::{NearlyEqUlps, UlpsTolerance, UlpsToleranceType};

    mock!(
        pub A{}

        impl UlpsTolerance for A {
            type T = i32;
            const DEFAULT: i32 = 7;
        }

        impl NearlyEqUlps for A {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }
    );

    mock!(
        pub B{}

        impl UlpsTolerance for B {
            type T = i64;
            const DEFAULT: i64 = 13;
        }

        impl NearlyEqUlps for B {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }
    );

    #[derive(NearlyEqUlps)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEqUlps)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEqUlps)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEqUlps)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEqUlps)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqUlps)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEqUlps)]
    struct UnitStruct;

    #[derive(NearlyEqUlps)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEqUlps)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    impl_test!(eq);
}

mod nearly_ord {
    use super::*;
    use nearly::{NearlyEqUlps, NearlyOrdUlps, UlpsTolerance, UlpsToleranceType};

    mock!(
        pub A{}

        impl UlpsTolerance for A {
            type T = i32;
            const DEFAULT: i32 = 7;
        }

        impl NearlyEqUlps for A {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdUlps for A {
            fn nearly_lt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_le_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_gt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_ge_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }
    );

    mock!(
        pub B{}

        impl UlpsTolerance for B {
            type T = i64;
            const DEFAULT: i64 = 13;
        }

        impl NearlyEqUlps for B {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyOrdUlps for B {
            fn nearly_lt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_le_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_gt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
            fn nearly_ge_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool;
        }
    );

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    struct NamedPair {
        a: MockA,
        b: MockB,
    }

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    struct NamedStructDifferentType {
        x: MockA,
        y: MockB,
        z: NamedPair,
    }

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    struct UnitStruct;

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[derive(NearlyEqUlps, NearlyOrdUlps)]
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
