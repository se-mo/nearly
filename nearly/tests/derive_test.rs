use mockall::mock;
use mockall::predicate::{always, eq};
use paste::paste;

macro_rules! inner_value {
    ($target: expr, $var: path) => {{
        if let $var(ref mut inner) = $target {
            inner
        } else {
            panic!("mismatch variant when extracting inner value")
        }
    }};
}

macro_rules! impl_eps_test {
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

macro_rules! impl_ulps_test {
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

macro_rules! impl_tol_test {
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

mod nearly_eq_eps {
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

    impl_eps_test!(eq);
}

mod nearly_eq_ulps {
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

    impl_ulps_test!(eq);
}

mod nearly_eq_tol {
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

    impl_tol_test!(eq);
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

mod nearly_ord_eps {
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

    impl_eps_test!(lt);
    impl_eps_test!(le);
    impl_eps_test!(gt);
    impl_eps_test!(ge);
}

mod nearly_ord_ulps {
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

    impl_ulps_test!(lt);
    impl_ulps_test!(le);
    impl_ulps_test!(gt);
    impl_ulps_test!(ge);
}

mod nearly_ord_tol {
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

    impl_tol_test!(lt);
    impl_tol_test!(le);
    impl_tol_test!(gt);
    impl_tol_test!(ge);
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
