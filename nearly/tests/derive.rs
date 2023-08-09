mod nearly_eq_eps {
    use nearly::{nearly, NearlyEqEps};

    #[derive(NearlyEqEps)]
    struct NamedStructSameType {
        x: f32,
        y: f32,
        z: f32,
    }

    #[test]
    fn derive_nearly_eq_eps_named_struct_same_type() {
        let a = NamedStructSameType {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = NamedStructSameType {
            x: 1.0000008,
            y: 2.0000008,
            z: 3.0000008,
        };
        let c = NamedStructSameType {
            x: 1.000002,
            y: 2.000002,
            z: 3.000002,
        };

        assert!(nearly!(a == b, eps = 0.0000009));
        assert!(nearly!(a != c, eps = 0.0000009));
    }

    #[derive(NearlyEqEps)]
    struct UnnamedStructSameType(f32, f32, f32);

    #[test]
    fn derive_nearly_eq_eps_unnamed_struct_same_type() {
        let a = UnnamedStructSameType(1.0, 2.0, 3.0);
        let b = UnnamedStructSameType(1.0000008, 2.0000008, 3.0000008);
        let c = UnnamedStructSameType(1.000002, 2.000002, 3.000002);

        assert!(nearly!(a == b, eps = 0.0000009));
        assert!(nearly!(a != c, eps = 0.0000009));
    }

    #[derive(NearlyEqEps)]
    struct NamedPair {
        a: f32,
        b: f64,
    }

    #[derive(NearlyEqEps)]
    struct NamedStructDifferentType {
        x: f32,
        y: f64,
        z: NamedPair,
    }

    #[test]
    fn derive_nearly_eq_eps_named_struct_different_type() {
        let a = NamedStructDifferentType {
            x: 1.0,
            y: 2.0,
            z: NamedPair { a: 3.0, b: 4.0 },
        };
        let b = NamedStructDifferentType {
            x: 1.0000008,
            y: 2.0000000000000016,
            z: NamedPair {
                a: 3.0000008,
                b: 4.000000000000003,
            },
        };
        let c = NamedStructDifferentType {
            x: 1.000002,
            y: 2.000000000000002,
            z: NamedPair {
                a: 3.000002,
                b: 4.000000000000002,
            },
        };

        let eps = (0.0000009, 0.0000000000003, (0.0000009, 0.0000000000003));
        assert!(nearly!(a == b, eps = eps));
        assert!(nearly!(a != c, eps = eps));
    }

    #[derive(NearlyEqEps)]
    struct UnnamedPair(f32, f64);

    #[derive(NearlyEqEps)]
    struct UnnamedStructDifferentType(f32, f64, UnnamedPair);

    #[test]
    fn derive_nearly_eq_eps_unnamed_struct_different_type() {
        let a = UnnamedStructDifferentType(1.0, 2.0, UnnamedPair(3.0, 4.0));
        let b = UnnamedStructDifferentType(
            1.0000008,
            2.0000000000000016,
            UnnamedPair(3.0000008, 4.000000000000003),
        );
        let c = UnnamedStructDifferentType(
            1.000002,
            2.000000000000002,
            UnnamedPair(3.000002, 4.000000000000002),
        );

        let eps = (0.0000009, 0.0000000000003, (0.0000009, 0.0000000000003));
        assert!(nearly!(a == b, eps = eps));
        assert!(nearly!(a != c, eps = eps));
    }

    #[derive(NearlyEqEps)]
    struct UnitStruct;

    #[test]
    fn derive_nearly_eq_eps_unit_struct() {
        let a = UnitStruct;
        let b = UnitStruct;

        assert!(nearly!(a == b, eps = ()));
    }

    #[derive(NearlyEqEps)]
    enum EnumSameType {
        X(f32),
        Y,
        Z(f32),
    }

    #[test]
    fn derive_nearly_eq_eps_enum_same_type() {
        let a = EnumSameType::X(1.0);
        let b = EnumSameType::Y;
        let c = EnumSameType::Y;
        let d = EnumSameType::Z(1.0);
        let e = EnumSameType::Z(1.0000008);
        let f = EnumSameType::Z(1.000002);

        assert!(nearly!(a != b, eps = 0.0000009));
        assert!(nearly!(b == c, eps = 0.0000009));
        assert!(nearly!(b != d, eps = 0.0000009));
        assert!(nearly!(a != d, eps = 0.0000009));
        assert!(nearly!(d == e, eps = 0.0000009));
        assert!(nearly!(d != f, eps = 0.0000009));
    }

    #[derive(NearlyEqEps)]
    enum EnumDifferentType {
        X(f64),
        Y,
        Z(f32),
    }

    #[test]
    fn derive_nearly_eq_eps_enum_different_type() {
        let a = EnumDifferentType::X(1.0);
        let b = EnumDifferentType::Y;
        let c = EnumDifferentType::Y;
        let d = EnumDifferentType::Z(1.0);
        let e = EnumDifferentType::Z(1.0000008);
        let f = EnumDifferentType::Z(1.000002);

        let eps = (0.0000000000003, 0.0000009);
        assert!(nearly!(a != b, eps = eps));
        assert!(nearly!(b == c, eps = eps));
        assert!(nearly!(b != d, eps = eps));
        assert!(nearly!(a != d, eps = eps));
        assert!(nearly!(d == e, eps = eps));
        assert!(nearly!(d != f, eps = eps));
    }
}

mod nearly_eq_ulps {
    use nearly::{nearly, NearlyEqUlps};

    #[derive(NearlyEqUlps)]
    struct NamedStructSameType {
        x: f32,
        y: f32,
        z: f32,
    }

    #[test]
    fn derive_nearly_eq_ulps_named_struct_same_type() {
        let a = NamedStructSameType {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = NamedStructSameType {
            x: 1.0000008,
            y: 2.0000008,
            z: 3.0000008,
        };
        let c = NamedStructSameType {
            x: 1.000002,
            y: 2.000002,
            z: 3.000002,
        };

        assert!(nearly!(a == b, ulps = 7));
        assert!(nearly!(a != c, ulps = 7));
    }

    #[derive(NearlyEqUlps)]
    struct UnnamedStructSameType(f32, f32, f32);

    #[test]
    fn derive_nearly_eq_ulps_unnamed_struct_same_type() {
        let a = UnnamedStructSameType(1.0, 2.0, 3.0);
        let b = UnnamedStructSameType(1.0000008, 2.0000008, 3.0000008);
        let c = UnnamedStructSameType(1.000002, 2.000002, 3.000002);

        assert!(nearly!(a == b, ulps = 7));
        assert!(nearly!(a != c, ulps = 7));
    }

    #[derive(NearlyEqUlps)]
    struct NamedPair {
        a: f32,
        b: f64,
    }

    #[derive(NearlyEqUlps)]
    struct NamedStructDifferentType {
        x: f32,
        y: f64,
        z: NamedPair,
    }

    #[test]
    fn derive_nearly_eq_ulps_named_struct_different_type() {
        let a = NamedStructDifferentType {
            x: 1.0,
            y: 2.0,
            z: NamedPair { a: 3.0, b: 4.0 },
        };
        let b = NamedStructDifferentType {
            x: 1.0000008,
            y: 2.0000000000000016,
            z: NamedPair {
                a: 3.0000008,
                b: 4.000000000000003,
            },
        };
        let c = NamedStructDifferentType {
            x: 1.000002,
            y: 2.000000000000002,
            z: NamedPair {
                a: 3.000002,
                b: 4.000000000000002,
            },
        };

        let ulps = (7, 7, (7, 7));
        assert!(nearly!(a == b, ulps = ulps));
        assert!(nearly!(a != c, ulps = ulps));
    }

    #[derive(NearlyEqUlps)]
    struct UnnamedPair(f32, f64);

    #[derive(NearlyEqUlps)]
    struct UnnamedStructDifferentType(f32, f64, UnnamedPair);

    #[test]
    fn derive_nearly_eq_ulps_unnamed_struct_different_type() {
        let a = UnnamedStructDifferentType(1.0, 2.0, UnnamedPair(3.0, 4.0));
        let b = UnnamedStructDifferentType(
            1.0000008,
            2.0000000000000016,
            UnnamedPair(3.0000008, 4.000000000000003),
        );
        let c = UnnamedStructDifferentType(
            1.000002,
            2.000000000000002,
            UnnamedPair(3.000002, 4.000000000000002),
        );

        let ulps = (7, 7, (7, 7));
        assert!(nearly!(a == b, ulps = ulps));
        assert!(nearly!(a != c, ulps = ulps));
    }

    #[derive(NearlyEqUlps)]
    struct UnitStruct;

    #[test]
    fn derive_nearly_eq_ulps_unit_struct() {
        let a = UnitStruct;
        let b = UnitStruct;

        assert!(nearly!(a == b, ulps = ()));
    }

    #[derive(NearlyEqUlps)]
    enum EnumSameType {
        X(f32),
        Y,
        Z(f32),
    }

    #[test]
    fn derive_nearly_eq_ulps_enum_same_type() {
        let a = EnumSameType::X(1.0);
        let b = EnumSameType::Y;
        let c = EnumSameType::Y;
        let d = EnumSameType::Z(1.0);
        let e = EnumSameType::Z(1.0000008);
        let f = EnumSameType::Z(1.000002);

        assert!(nearly!(a != b, ulps = 7));
        assert!(nearly!(b == c, ulps = 7));
        assert!(nearly!(b != d, ulps = 7));
        assert!(nearly!(a != d, ulps = 7));
        assert!(nearly!(d == e, ulps = 7));
        assert!(nearly!(d != f, ulps = 7));
    }

    #[derive(NearlyEqUlps)]
    enum EnumDifferentType {
        X(f64),
        Y,
        Z(f32),
    }

    #[test]
    fn derive_nearly_eq_ulps_enum_different_type() {
        let a = EnumDifferentType::X(1.0);
        let b = EnumDifferentType::Y;
        let c = EnumDifferentType::Y;
        let d = EnumDifferentType::Z(1.0);
        let e = EnumDifferentType::Z(1.0000008);
        let f = EnumDifferentType::Z(1.000002);

        let ulps = (7, 7);
        assert!(nearly!(a != b, ulps = ulps));
        assert!(nearly!(b == c, ulps = ulps));
        assert!(nearly!(b != d, ulps = ulps));
        assert!(nearly!(a != d, ulps = ulps));
        assert!(nearly!(d == e, ulps = ulps));
        assert!(nearly!(d != f, ulps = ulps));
    }
}

mod nearly_eq {
    use nearly::{nearly, NearlyEq, Tolerance};

    #[derive(NearlyEq, Clone, Copy)]
    struct NamedStructSameType {
        x: f32,
        y: f32,
        z: f32,
    }

    #[test]
    fn derive_nearly_eq_named_struct_same_type() {
        let a = NamedStructSameType {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = NamedStructSameType {
            x: 1.0000008,
            y: 2.0000008,
            z: 3.0000008,
        };
        let c = NamedStructSameType {
            x: 1.000002,
            y: 2.000002,
            z: 3.000002,
        };

        assert!(nearly!(a == b, eps = 0.0000009));
        assert!(nearly!(a != c, eps = 0.0000009));

        assert!(nearly!(a == b, ulps = 7));
        assert!(nearly!(a != c, ulps = 7));

        assert!(nearly!(a == b, eps = 0.0000009, ulps = 7));
        assert!(nearly!(a != c, eps = 0.0000009, ulps = 7));

        assert!(nearly!(
            a == b,
            tol = Tolerance::<NamedStructSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(
            a != c,
            tol = Tolerance::<NamedStructSameType>::new(0.0000009, 7)
        ));

        assert!(nearly!(a == b));
        assert!(nearly!(a != c));
    }

    #[derive(NearlyEq, Clone, Copy)]
    struct UnnamedStructSameType(f32, f32, f32);

    #[test]
    fn derive_nearly_eq_unnamed_struct_same_type() {
        let a = UnnamedStructSameType(1.0, 2.0, 3.0);
        let b = UnnamedStructSameType(1.0000008, 2.0000008, 3.0000008);
        let c = UnnamedStructSameType(1.000002, 2.000002, 3.000002);

        assert!(nearly!(a == b, eps = 0.0000009));
        assert!(nearly!(a != c, eps = 0.0000009));

        assert!(nearly!(a == b, ulps = 7));
        assert!(nearly!(a != c, ulps = 7));

        assert!(nearly!(a == b, eps = 0.0000009, ulps = 7));
        assert!(nearly!(a != c, eps = 0.0000009, ulps = 7));

        assert!(nearly!(
            a == b,
            tol = Tolerance::<UnnamedStructSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(
            a != c,
            tol = Tolerance::<UnnamedStructSameType>::new(0.0000009, 7)
        ));

        assert!(nearly!(a == b));
        assert!(nearly!(a != c));
    }

    #[derive(NearlyEq, Clone, Copy)]
    struct NamedPair {
        a: f32,
        b: f64,
    }

    #[derive(NearlyEq, Clone, Copy)]
    struct NamedStructDifferentType {
        x: f32,
        y: f64,
        z: NamedPair,
    }

    #[test]
    fn derive_nearly_eq_named_struct_different_type() {
        let a = NamedStructDifferentType {
            x: 1.0,
            y: 2.0,
            z: NamedPair { a: 3.0, b: 4.0 },
        };
        let b = NamedStructDifferentType {
            x: 1.0000008,
            y: 2.0000000000000016,
            z: NamedPair {
                a: 3.0000008,
                b: 4.000000000000003,
            },
        };
        let c = NamedStructDifferentType {
            x: 1.000002,
            y: 2.000000000000002,
            z: NamedPair {
                a: 3.000002,
                b: 4.000000000000002,
            },
        };

        let eps = (0.0000009, 0.0000000000003, (0.0000009, 0.0000000000003));
        let ulps = (7, 7, (7, 7));

        assert!(nearly!(a == b, eps = eps));
        assert!(nearly!(a != c, eps = eps));

        assert!(nearly!(a == b, ulps = ulps));
        assert!(nearly!(a != c, ulps = ulps));

        assert!(nearly!(a == b, eps = eps, ulps = ulps));
        assert!(nearly!(a != c, eps = eps, ulps = ulps));

        assert!(nearly!(
            a == b,
            tol = Tolerance::<NamedStructDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(
            a != c,
            tol = Tolerance::<NamedStructDifferentType>::new(eps, ulps)
        ));

        assert!(nearly!(a == b));
        assert!(nearly!(a != c));
    }

    #[derive(NearlyEq, Clone, Copy)]
    struct UnnamedPair(f32, f64);

    #[derive(NearlyEq, Clone, Copy)]
    struct UnnamedStructDifferentType(f32, f64, UnnamedPair);

    #[test]
    fn derive_nearly_eq_unnamed_struct_different_type() {
        let a = UnnamedStructDifferentType(1.0, 2.0, UnnamedPair(3.0, 4.0));
        let b = UnnamedStructDifferentType(
            1.0000008,
            2.0000000000000016,
            UnnamedPair(3.0000008, 4.000000000000003),
        );
        let c = UnnamedStructDifferentType(
            1.000002,
            2.000000000000002,
            UnnamedPair(3.000002, 4.000000000000002),
        );

        let eps = (0.0000009, 0.0000000000003, (0.0000009, 0.0000000000003));
        let ulps = (7, 7, (7, 7));

        assert!(nearly!(a == b, eps = eps));
        assert!(nearly!(a != c, eps = eps));

        assert!(nearly!(a == b, ulps = ulps));
        assert!(nearly!(a != c, ulps = ulps));

        assert!(nearly!(a == b, eps = eps, ulps = ulps));
        assert!(nearly!(a != c, eps = eps, ulps = ulps));

        assert!(nearly!(
            a == b,
            tol = Tolerance::<UnnamedStructDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(
            a != c,
            tol = Tolerance::<UnnamedStructDifferentType>::new(eps, ulps)
        ));

        assert!(nearly!(a == b));
        assert!(nearly!(a != c));
    }

    #[derive(NearlyEq, Clone, Copy)]
    struct UnitStruct;

    #[test]
    fn derive_nearly_eq_unit_struct() {
        let a = UnitStruct;
        let b = UnitStruct;

        assert!(nearly!(a == b, eps = ()));
        assert!(nearly!(a == b, ulps = ()));
        assert!(nearly!(a == b, eps = (), ulps = ()));
        assert!(nearly!(a == b, tol = Tolerance::<UnitStruct>::new((), ())));
        assert!(nearly!(a == b));
    }

    #[derive(NearlyEq, Clone, Copy)]
    enum EnumSameType {
        X(f32),
        Y,
        Z(f32),
    }

    #[test]
    fn derive_nearly_eq_enum_same_type() {
        let a = EnumSameType::X(1.0);
        let b = EnumSameType::Y;
        let c = EnumSameType::Y;
        let d = EnumSameType::Z(1.0);
        let e = EnumSameType::Z(1.0000008);
        let f = EnumSameType::Z(1.000002);

        assert!(nearly!(a != b, eps = 0.0000009));
        assert!(nearly!(a != b, ulps = 7));
        assert!(nearly!(a != b, eps = 0.0000009, ulps = 7));
        assert!(nearly!(
            a != b,
            tol = Tolerance::<EnumSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(a != b));

        assert!(nearly!(b == c, eps = 0.0000009));
        assert!(nearly!(b == c, ulps = 7));
        assert!(nearly!(b == c, eps = 0.0000009, ulps = 7));
        assert!(nearly!(
            b == c,
            tol = Tolerance::<EnumSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(b == c));

        assert!(nearly!(b != d, eps = 0.0000009));
        assert!(nearly!(b != d, ulps = 7));
        assert!(nearly!(b != d, eps = 0.0000009, ulps = 7));
        assert!(nearly!(
            b != d,
            tol = Tolerance::<EnumSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(b != d));

        assert!(nearly!(a != d, eps = 0.0000009));
        assert!(nearly!(a != d, ulps = 7));
        assert!(nearly!(a != d, eps = 0.0000009, ulps = 7));
        assert!(nearly!(
            a != d,
            tol = Tolerance::<EnumSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(a != d));

        assert!(nearly!(d == e, eps = 0.0000009));
        assert!(nearly!(d == e, ulps = 7));
        assert!(nearly!(d == e, eps = 0.0000009, ulps = 7));
        assert!(nearly!(
            d == e,
            tol = Tolerance::<EnumSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(d == e));

        assert!(nearly!(d != f, eps = 0.0000009));
        assert!(nearly!(d != f, ulps = 7));
        assert!(nearly!(d != f, eps = 0.0000009, ulps = 7));
        assert!(nearly!(
            d != f,
            tol = Tolerance::<EnumSameType>::new(0.0000009, 7)
        ));
        assert!(nearly!(d != f));
    }

    #[derive(NearlyEq, Clone, Copy)]
    enum EnumDifferentType {
        X(f64),
        Y,
        Z(f32),
    }

    #[test]
    fn derive_nearly_eq_enum_different_type() {
        let a = EnumDifferentType::X(1.0);
        let b = EnumDifferentType::Y;
        let c = EnumDifferentType::Y;
        let d = EnumDifferentType::Z(1.0);
        let e = EnumDifferentType::Z(1.0000008);
        let f = EnumDifferentType::Z(1.000002);

        let eps = (0.0000000000003, 0.0000009);
        let ulps = (7, 7);

        assert!(nearly!(a != b, eps = eps));
        assert!(nearly!(a != b, ulps = ulps));
        assert!(nearly!(a != b, eps = eps, ulps = ulps));
        assert!(nearly!(
            a != b,
            tol = Tolerance::<EnumDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(a != b));

        assert!(nearly!(b == c, eps = eps));
        assert!(nearly!(b == c, ulps = ulps));
        assert!(nearly!(b == c, eps = eps, ulps = ulps));
        assert!(nearly!(
            b == c,
            tol = Tolerance::<EnumDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(b == c));

        assert!(nearly!(b != d, eps = eps));
        assert!(nearly!(b != d, ulps = ulps));
        assert!(nearly!(b != d, eps = eps, ulps = ulps));
        assert!(nearly!(
            b != d,
            tol = Tolerance::<EnumDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(b != d));

        assert!(nearly!(a != d, eps = eps));
        assert!(nearly!(a != d, ulps = ulps));
        assert!(nearly!(a != d, eps = eps, ulps = ulps));
        assert!(nearly!(
            a != d,
            tol = Tolerance::<EnumDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(a != d));

        assert!(nearly!(d == e, eps = eps));
        assert!(nearly!(d == e, ulps = ulps));
        assert!(nearly!(d == e, eps = eps, ulps = ulps));
        assert!(nearly!(
            d == e,
            tol = Tolerance::<EnumDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(d == e));

        assert!(nearly!(d != f, eps = eps));
        assert!(nearly!(d != f, ulps = ulps));
        assert!(nearly!(d != f, eps = eps, ulps = ulps));
        assert!(nearly!(
            d != f,
            tol = Tolerance::<EnumDifferentType>::new(eps, ulps)
        ));
        assert!(nearly!(d != f));
    }
}
