use mockall::mock;
use mockall::predicate::{always, eq};

macro_rules! inner_value {
    ($target: expr, $var: path) => {{
        if let $var(ref mut inner) = $target {
            inner
        } else {
            panic!("mismatch variant when extracting inner value")
        }
    }};
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
            fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool;
        }
    );

    mock!(
        pub B{}

        impl EpsTolerance for B {
            type T = f64;
            const DEFAULT: f64 = 0.05;
        }

        impl NearlyEqEps for B {
            fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool;
        }
    );

    #[derive(NearlyEqEps)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[test]
    fn derive_nearly_eq_eps_named_struct_same_type() {
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

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_eps(&b, 0.1_f32));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_eps().times(0);
        a.z.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, 0.1_f32));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        a.z.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, 0.1_f32));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_eps(&b, 0.1_f32));
    }

    #[derive(NearlyEqEps)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[test]
    fn derive_nearly_eq_eps_unnamed_struct_same_type() {
        let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
        let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_eps(&b, 0.1_f32));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_eps().times(0);
        a.2.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, 0.1_f32));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        a.2.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, 0.1_f32));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_eps(&b, 0.1_f32));
    }

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

    #[test]
    fn derive_nearly_eq_eps_named_struct_different_type() {
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

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_eps()
            .with(always(), eq(0.2))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_eps()
            .with(always(), eq(0.02))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_eps(&b, eps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_eps().times(0);
        a.z.a.expect_nearly_eq_eps().times(0);
        a.z.b.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, eps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(false);
        a.z.a.expect_nearly_eq_eps().times(0);
        a.z.b.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, eps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_eps()
            .with(always(), eq(0.2))
            .times(1)
            .return_const(false);
        a.z.b.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, eps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_eps()
            .with(always(), eq(0.2))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_eps()
            .with(always(), eq(0.02))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_eps(&b, eps));
    }

    #[derive(NearlyEqEps)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqEps)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[test]
    fn derive_nearly_eq_eps_unnamed_struct_different_type() {
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

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_eps()
            .with(always(), eq(0.2))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_eps()
            .with(always(), eq(0.02))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_eps(&b, eps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_eps().times(0);
        a.2 .0.expect_nearly_eq_eps().times(0);
        a.2 .1.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, eps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(false);
        a.2 .0.expect_nearly_eq_eps().times(0);
        a.2 .1.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, eps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_eps()
            .with(always(), eq(0.2))
            .times(1)
            .return_const(false);
        a.2 .1.expect_nearly_eq_eps().times(0);

        assert!(!a.nearly_eq_eps(&b, eps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_eps()
            .with(always(), eq(0.2))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_eps()
            .with(always(), eq(0.02))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_eps(&b, eps));
    }

    #[derive(NearlyEqEps)]
    struct UnitStruct;

    #[test]
    fn derive_nearly_eq_eps_unit_struct() {
        let a = UnitStruct;
        let b = UnitStruct;

        assert!(a.nearly_eq_eps(&b, ()));
    }

    #[derive(NearlyEqEps)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[test]
    fn derive_nearly_eq_eps_enum_same_type() {
        let mut a = EnumSameType::X(MockA::new());
        let mut b = EnumSameType::X(MockA::new());
        let c = EnumSameType::Y;
        let d = EnumSameType::Y;
        let mut e = EnumSameType::Z(MockA::new());
        let f = EnumSameType::Z(MockA::new());

        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq_eps(&b, 0.1_f32));

        inner_value!(a, EnumSameType::X).checkpoint();
        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq_eps(&b, 0.1_f32));

        inner_value!(b, EnumSameType::X)
            .expect_nearly_eq_eps()
            .times(0);

        assert!(!b.nearly_eq_eps(&c, 0.1_f32));
        assert!(c.nearly_eq_eps(&d, 0.1_f32));
        assert!(!b.nearly_eq_eps(&e, 0.1_f32));
        assert!(!c.nearly_eq_eps(&e, 0.1_f32));

        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq_eps(&f, 0.1_f32));

        inner_value!(e, EnumSameType::Z).checkpoint();
        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq_eps(&f, 0.1_f32));
    }

    #[derive(NearlyEqEps)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    #[test]
    fn derive_nearly_eq_eps_enum_different_type() {
        let mut a = EnumDifferentType::X(MockA::new());
        let mut b = EnumDifferentType::X(MockA::new());
        let c = EnumDifferentType::Y;
        let d = EnumDifferentType::Y;
        let mut e = EnumDifferentType::Z(MockB::new());
        let f = EnumDifferentType::Z(MockB::new());

        let eps = (0.1_f32, 0.01_f64);

        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq_eps(&b, eps));

        inner_value!(a, EnumDifferentType::X).checkpoint();
        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.1))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq_eps(&b, eps));

        inner_value!(b, EnumDifferentType::X)
            .expect_nearly_eq_eps()
            .times(0);

        assert!(!b.nearly_eq_eps(&c, eps));
        assert!(c.nearly_eq_eps(&d, eps));
        assert!(!b.nearly_eq_eps(&e, eps));
        assert!(!c.nearly_eq_eps(&e, eps));

        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq_eps(&f, eps));

        inner_value!(e, EnumDifferentType::Z).checkpoint();
        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_eps()
            .with(always(), eq(0.01))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq_eps(&f, eps));
    }
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
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool;
        }
    );

    mock!(
        pub B{}

        impl UlpsTolerance for B {
            type T = i64;
            const DEFAULT: i64 = 13;
        }

        impl NearlyEqUlps for B {
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool;
        }
    );

    #[derive(NearlyEqUlps)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[test]
    fn derive_nearly_eq_ulps_named_struct_same_type() {
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

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_ulps(&b, 5_i32));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_ulps().times(0);
        a.z.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, 5_i32));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        a.z.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, 5_i32));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_ulps(&b, 5_i32));
    }

    #[derive(NearlyEqUlps)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[test]
    fn derive_nearly_eq_ulps_unnamed_struct_same_type() {
        let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
        let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_ulps(&b, 5_i32));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_ulps().times(0);
        a.2.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, 5_i32));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        a.2.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, 5_i32));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_ulps(&b, 5_i32));
    }

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

    #[test]
    fn derive_nearly_eq_ulps_named_struct_different_type() {
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

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_ulps()
            .with(always(), eq(6))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_ulps()
            .with(always(), eq(12))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_ulps(&b, ulps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_ulps().times(0);
        a.z.a.expect_nearly_eq_ulps().times(0);
        a.z.b.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, ulps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(false);
        a.z.a.expect_nearly_eq_ulps().times(0);
        a.z.b.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, ulps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_ulps()
            .with(always(), eq(6))
            .times(1)
            .return_const(false);
        a.z.b.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, ulps));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_ulps()
            .with(always(), eq(6))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_ulps()
            .with(always(), eq(12))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_ulps(&b, ulps));
    }

    #[derive(NearlyEqUlps)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqUlps)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[test]
    fn derive_nearly_eq_ulps_unnamed_struct_different_type() {
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

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_ulps()
            .with(always(), eq(6))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_ulps()
            .with(always(), eq(12))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_ulps(&b, ulps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_ulps().times(0);
        a.2 .0.expect_nearly_eq_ulps().times(0);
        a.2 .1.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, ulps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(false);
        a.2 .0.expect_nearly_eq_ulps().times(0);
        a.2 .1.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, ulps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_ulps()
            .with(always(), eq(6))
            .times(1)
            .return_const(false);
        a.2 .1.expect_nearly_eq_ulps().times(0);

        assert!(!a.nearly_eq_ulps(&b, ulps));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_ulps()
            .with(always(), eq(6))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_ulps()
            .with(always(), eq(12))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_ulps(&b, ulps));
    }

    #[derive(NearlyEqUlps)]
    struct UnitStruct;

    #[test]
    fn derive_nearly_eq_ulps_unit_struct() {
        let a = UnitStruct;
        let b = UnitStruct;

        assert!(a.nearly_eq_ulps(&b, ()));
    }

    #[derive(NearlyEqUlps)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[test]
    fn derive_nearly_eq_ulps_enum_same_type() {
        let mut a = EnumSameType::X(MockA::new());
        let mut b = EnumSameType::X(MockA::new());
        let c = EnumSameType::Y;
        let d = EnumSameType::Y;
        let mut e = EnumSameType::Z(MockA::new());
        let f = EnumSameType::Z(MockA::new());

        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq_ulps(&b, 5_i32));

        inner_value!(a, EnumSameType::X).checkpoint();
        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq_ulps(&b, 5_i32));

        inner_value!(b, EnumSameType::X)
            .expect_nearly_eq_ulps()
            .times(0);

        assert!(!b.nearly_eq_ulps(&c, 5_i32));
        assert!(c.nearly_eq_ulps(&d, 5_i32));
        assert!(!b.nearly_eq_ulps(&e, 5_i32));
        assert!(!c.nearly_eq_ulps(&e, 5_i32));

        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq_ulps(&f, 5_i32));

        inner_value!(e, EnumSameType::Z).checkpoint();
        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq_ulps(&f, 5_i32));
    }

    #[derive(NearlyEqUlps)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    #[test]
    fn derive_nearly_eq_ulps_enum_different_type() {
        let mut a = EnumDifferentType::X(MockA::new());
        let mut b = EnumDifferentType::X(MockA::new());
        let c = EnumDifferentType::Y;
        let d = EnumDifferentType::Y;
        let mut e = EnumDifferentType::Z(MockB::new());
        let f = EnumDifferentType::Z(MockB::new());

        let ulps = (5_i32, 10_i64);

        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq_ulps(&b, ulps));

        inner_value!(a, EnumDifferentType::X).checkpoint();
        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_ulps()
            .with(always(), eq(5))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq_ulps(&b, ulps));

        inner_value!(b, EnumDifferentType::X)
            .expect_nearly_eq_ulps()
            .times(0);

        assert!(!b.nearly_eq_ulps(&c, ulps));
        assert!(c.nearly_eq_ulps(&d, ulps));
        assert!(!b.nearly_eq_ulps(&e, ulps));
        assert!(!c.nearly_eq_ulps(&e, ulps));

        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq_ulps(&f, ulps));

        inner_value!(e, EnumDifferentType::Z).checkpoint();
        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_ulps()
            .with(always(), eq(10))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq_ulps(&f, ulps));
    }
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
            fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for A {
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for A {
            fn nearly_eq_tol(&self, other: &Self, tolerance: Tolerance<Self>) -> bool;
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
            fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for B {
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for B {
            fn nearly_eq_tol(&self, other: &Self, tolerance: Tolerance<Self>) -> bool;
        }
    );

    #[derive(NearlyEqTol)]
    struct NamedStructSameType {
        x: MockA,
        y: MockA,
        z: MockA,
    }

    #[test]
    fn derive_nearly_eq_tol_named_struct_same_type() {
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

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_tol(&b, Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_tol().times(0);
        a.z.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        a.z.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_tol(&b, Tolerance::<NamedStructSameType>::new(0.1_f32, 5_i32)));
    }

    #[derive(NearlyEqTol)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[test]
    fn derive_nearly_eq_tol_unnamed_struct_same_type() {
        let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
        let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_tol(&b, Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_tol().times(0);
        a.2.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        a.2.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_tol(&b, Tolerance::<UnnamedStructSameType>::new(0.1_f32, 5_i32)));
    }

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

    #[test]
    fn derive_nearly_eq_tol_named_struct_different_type() {
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
        let tolerance = Tolerance::<NamedStructDifferentType>::new(eps, ulps);

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_tol(&b, tolerance));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_tol().times(0);
        a.z.a.expect_nearly_eq_tol().times(0);
        a.z.b.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, tolerance));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(false);
        a.z.a.expect_nearly_eq_tol().times(0);
        a.z.b.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, tolerance));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
            .times(1)
            .return_const(false);
        a.z.b.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, tolerance));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_tol(&b, tolerance));
    }

    #[derive(NearlyEqTol)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEqTol)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[test]
    fn derive_nearly_eq_tol_unnamed_struct_different_type() {
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
        let tolerance = Tolerance::<UnnamedStructDifferentType>::new(eps, ulps);

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq_tol(&b, tolerance));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_tol().times(0);
        a.2 .0.expect_nearly_eq_tol().times(0);
        a.2 .1.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, tolerance));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(false);
        a.2 .0.expect_nearly_eq_tol().times(0);
        a.2 .1.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, tolerance));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
            .times(1)
            .return_const(false);
        a.2 .1.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq_tol(&b, tolerance));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.2_f32, 6_i32)))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.02_f64, 12_i64)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq_tol(&b, tolerance));
    }

    #[derive(NearlyEqTol)]
    struct UnitStruct;

    #[test]
    fn derive_nearly_eq_tol_unit_struct() {
        let a = UnitStruct;
        let b = UnitStruct;

        assert!(a.nearly_eq_tol(&b, Tolerance::<UnitStruct>::new((), ())));
    }

    #[derive(NearlyEqTol)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[test]
    fn derive_nearly_eq_tol_enum_same_type() {
        let mut a = EnumSameType::X(MockA::new());
        let mut b = EnumSameType::X(MockA::new());
        let c = EnumSameType::Y;
        let d = EnumSameType::Y;
        let mut e = EnumSameType::Z(MockA::new());
        let f = EnumSameType::Z(MockA::new());

        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq_tol(&b, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

        inner_value!(a, EnumSameType::X).checkpoint();
        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq_tol(&b, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

        inner_value!(b, EnumSameType::X)
            .expect_nearly_eq_tol()
            .times(0);

        assert!(!b.nearly_eq_tol(&c, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
        assert!(c.nearly_eq_tol(&d, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
        assert!(!b.nearly_eq_tol(&e, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
        assert!(!c.nearly_eq_tol(&e, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq_tol(&f, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));

        inner_value!(e, EnumSameType::Z).checkpoint();
        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq_tol(&f, Tolerance::<EnumSameType>::new(0.1_f32, 5_i32)));
    }

    #[derive(NearlyEqTol)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    #[test]
    fn derive_nearly_eq_tol_enum_different_type() {
        let mut a = EnumDifferentType::X(MockA::new());
        let mut b = EnumDifferentType::X(MockA::new());
        let c = EnumDifferentType::Y;
        let d = EnumDifferentType::Y;
        let mut e = EnumDifferentType::Z(MockB::new());
        let f = EnumDifferentType::Z(MockB::new());

        let eps = (0.1_f32, 0.01_f64);
        let ulps = (5_i32, 10_i64);
        let tolerance = Tolerance::<EnumDifferentType>::new(eps, ulps);

        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq_tol(&b, tolerance));

        inner_value!(a, EnumDifferentType::X).checkpoint();
        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.1_f32, 5_i32)))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq_tol(&b, tolerance));

        inner_value!(b, EnumDifferentType::X)
            .expect_nearly_eq_tol()
            .times(0);

        assert!(!b.nearly_eq_tol(&c, tolerance));
        assert!(c.nearly_eq_tol(&d, tolerance));
        assert!(!b.nearly_eq_tol(&e, tolerance));
        assert!(!c.nearly_eq_tol(&e, tolerance));

        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq_tol(&f, tolerance));

        inner_value!(e, EnumDifferentType::Z).checkpoint();
        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.01_f64, 10_i64)))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq_tol(&f, tolerance));
    }
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
            fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for A {
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for A {
            fn nearly_eq_tol(&self, other: &Self, tolerance: Tolerance<Self>) -> bool;
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
            fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqUlps for B {
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool;
        }

        impl NearlyEqTol for B {
            fn nearly_eq_tol(&self, other: &Self, tolerance: Tolerance<Self>) -> bool;
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

    #[test]
    fn derive_nearly_eq_named_struct_same_type() {
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

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq(&b));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_tol().times(0);
        a.z.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.z.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.z.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq(&b));
    }

    #[derive(NearlyEq)]
    struct UnnamedStructSameType(MockA, MockA, MockA);

    #[test]
    fn derive_nearly_eq_unnamed_struct_same_type() {
        let mut a = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());
        let b = UnnamedStructSameType(MockA::new(), MockA::new(), MockA::new());

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq(&b));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_tol().times(0);
        a.2.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.2.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.2.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq(&b));
    }

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

    #[test]
    fn derive_nearly_eq_named_struct_different_type() {
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

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq(&b));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.y.expect_nearly_eq_tol().times(0);
        a.z.a.expect_nearly_eq_tol().times(0);
        a.z.b.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(false);
        a.z.a.expect_nearly_eq_tol().times(0);
        a.z.b.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.z.b.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.x.checkpoint();
        a.y.checkpoint();
        a.z.a.checkpoint();
        a.z.b.checkpoint();

        a.x.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.y.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);
        a.z.a
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.z.b
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq(&b));
    }

    #[derive(NearlyEq)]
    struct UnnamedPair(MockA, MockB);

    #[derive(NearlyEq)]
    struct UnnamedStructDifferentType(MockA, MockB, UnnamedPair);

    #[test]
    fn derive_nearly_eq_unnamed_struct_different_type() {
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

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);

        assert!(a.nearly_eq(&b));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.1.expect_nearly_eq_tol().times(0);
        a.2 .0.expect_nearly_eq_tol().times(0);
        a.2 .1.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(false);
        a.2 .0.expect_nearly_eq_tol().times(0);
        a.2 .1.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        a.2 .1.expect_nearly_eq_tol().times(0);

        assert!(!a.nearly_eq(&b));

        a.0.checkpoint();
        a.1.checkpoint();
        a.2 .0.checkpoint();
        a.2 .1.checkpoint();

        a.0.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.1.expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);
        a.2 .0
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        a.2 .1
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(false);

        assert!(!a.nearly_eq(&b));
    }

    #[derive(NearlyEq)]
    struct UnitStruct;

    #[test]
    fn derive_nearly_eq_unit_struct() {
        let a = UnitStruct;
        let b = UnitStruct;

        assert!(a.nearly_eq(&b));
    }

    #[derive(NearlyEq)]
    enum EnumSameType {
        X(MockA),
        Y,
        Z(MockA),
    }

    #[test]
    fn derive_nearly_eq_enum_same_type() {
        let mut a = EnumSameType::X(MockA::new());
        let mut b = EnumSameType::X(MockA::new());
        let c = EnumSameType::Y;
        let d = EnumSameType::Y;
        let mut e = EnumSameType::Z(MockA::new());
        let f = EnumSameType::Z(MockA::new());

        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq(&b));

        inner_value!(a, EnumSameType::X).checkpoint();
        inner_value!(a, EnumSameType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq(&b));

        inner_value!(b, EnumSameType::X)
            .expect_nearly_eq_tol()
            .times(0);

        assert!(!b.nearly_eq(&c));
        assert!(c.nearly_eq(&d));
        assert!(!b.nearly_eq(&e));
        assert!(!c.nearly_eq(&e));

        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq(&f));

        inner_value!(e, EnumSameType::Z).checkpoint();
        inner_value!(e, EnumSameType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq(&f));
    }

    #[derive(NearlyEq)]
    enum EnumDifferentType {
        X(MockA),
        Y,
        Z(MockB),
    }

    #[test]
    fn derive_nearly_eq_enum_different_type() {
        let mut a = EnumDifferentType::X(MockA::new());
        let mut b = EnumDifferentType::X(MockA::new());
        let c = EnumDifferentType::Y;
        let d = EnumDifferentType::Y;
        let mut e = EnumDifferentType::Z(MockB::new());
        let f = EnumDifferentType::Z(MockB::new());

        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(true);
        assert!(a.nearly_eq(&b));

        inner_value!(a, EnumDifferentType::X).checkpoint();
        inner_value!(a, EnumDifferentType::X)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockA>::new(0.5_f32, 7_i32)))
            .times(1)
            .return_const(false);
        assert!(!a.nearly_eq(&b));

        inner_value!(b, EnumDifferentType::X)
            .expect_nearly_eq_tol()
            .times(0);

        assert!(!b.nearly_eq(&c));
        assert!(c.nearly_eq(&d));
        assert!(!b.nearly_eq(&e));
        assert!(!c.nearly_eq(&e));

        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(true);
        assert!(e.nearly_eq(&f));

        inner_value!(e, EnumDifferentType::Z).checkpoint();
        inner_value!(e, EnumDifferentType::Z)
            .expect_nearly_eq_tol()
            .with(always(), eq(Tolerance::<MockB>::new(0.05_f64, 13_i64)))
            .times(1)
            .return_const(false);
        assert!(!e.nearly_eq(&f));
    }
}
