use nearly::NearlyOrdEps;

///////////////////
// nearly_lt_eps //
///////////////////

#[test]
fn nearly_lt_eps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(!a.nearly_lt_eps(&b, &0.0));
    assert!(!b.nearly_lt_eps(&b, &0.0));
}

#[test]
fn nearly_lt_eps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(!a.nearly_lt_eps(&b, &0.0));
    assert!(!b.nearly_lt_eps(&b, &0.0));
}

#[test]
fn nearly_lt_eps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(!a.nearly_lt_eps(&b, &0.0));
    assert!(!b.nearly_lt_eps(&a, &0.0));
}

#[test]
fn nearly_lt_eps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(!a.nearly_lt_eps(&b, &0.0));
    assert!(!b.nearly_lt_eps(&a, &0.0));
}

#[test]
fn nearly_lt_eps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;

    assert!(a.nearly_lt_eps(&b, &0.0000007));
    assert!(!b.nearly_lt_eps(&a, &0.0000007));

    assert!(a.nearly_lt_eps(&b, &0.0000008));
    assert!(!b.nearly_lt_eps(&a, &0.0000008));
    
    assert!(!a.nearly_lt_eps(&b, &0.0000009));
    assert!(!b.nearly_lt_eps(&a, &0.0000009));
}

#[test]
fn nearly_lt_eps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000003;

    assert!(a.nearly_lt_eps(&b, &0.0000000000002));
    assert!(!b.nearly_lt_eps(&a, &0.0000000000002));

    assert!(!a.nearly_lt_eps(&b, &0.0000000000003));
    assert!(!b.nearly_lt_eps(&a, &0.0000000000003));
    
    assert!(!a.nearly_lt_eps(&b, &0.0000000000004));
    assert!(!b.nearly_lt_eps(&a, &0.0000000000004));
}

#[test]
fn nearly_lt_eps_different_sign_f32() {
    let a: f32 = -0.01;
    let b: f32 = 0.01;

    assert!(a.nearly_lt_eps(&b, &0.019));
    assert!(!b.nearly_lt_eps(&a, &0.019));

    assert!(!a.nearly_lt_eps(&b, &0.02));
    assert!(!b.nearly_lt_eps(&a, &0.02));
}

#[test]
fn nearly_lt_eps_different_sign_f64() {
    let a: f64 = -0.01;
    let b: f64 = 0.01;

    assert!(a.nearly_lt_eps(&b, &0.019));
    assert!(!b.nearly_lt_eps(&a, &0.019));

    assert!(!a.nearly_lt_eps(&b, &0.02));
    assert!(!b.nearly_lt_eps(&a, &0.02));
}

#[test]
fn nearly_lt_eps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = f32::INFINITY;
        assert!(a.nearly_lt_eps(&b, &f32::MAX));
        assert!(!b.nearly_lt_eps(&a, &f32::MAX));
    }
    {
        let a: f32 = -f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_lt_eps(&b, &f32::MAX));
        assert!(!b.nearly_lt_eps(&a, &f32::MAX));
    }

    assert!(!f32::INFINITY.nearly_lt_eps(&f32::INFINITY, &0.0));
    assert!(!(-f32::INFINITY).nearly_lt_eps(&-f32::INFINITY, &0.0));
    assert!(!f32::INFINITY.nearly_lt_eps(&-f32::INFINITY, &f32::MAX));
    assert!((-f32::INFINITY).nearly_lt_eps(&f32::INFINITY, &f32::MAX));
}

#[test]
fn nearly_lt_eps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = f64::INFINITY;
        assert!(a.nearly_lt_eps(&b, &f64::MAX));
        assert!(!b.nearly_lt_eps(&a, &f64::MAX));
    }
    {
        let a: f64 = -f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_lt_eps(&b, &f64::MAX));
        assert!(!b.nearly_lt_eps(&a, &f64::MAX));
    }

    assert!(!f64::INFINITY.nearly_lt_eps(&f64::INFINITY, &0.0));
    assert!(!(-f64::INFINITY).nearly_lt_eps(&-f64::INFINITY, &0.0));
    assert!(!f64::INFINITY.nearly_lt_eps(&-f64::INFINITY, &f64::MAX));
    assert!((-f64::INFINITY).nearly_lt_eps(&f64::INFINITY, &f64::MAX));
}

#[test]
fn nearly_lt_eps_nan_f32() {
    let a: f32 = f32::NAN;
    let b: f32 = 1.0;
    assert!(!a.nearly_lt_eps(&b, &0.0));
    assert!(!b.nearly_lt_eps(&a, &0.0));

    assert!(!f32::NAN.nearly_lt_eps(&f32::NAN, &0.0));
    assert!(!f32::NAN.nearly_lt_eps(&-f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_lt_eps(&f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_lt_eps(&-f32::NAN, &0.0));
}

#[test]
fn nearly_lt_eps_nan_f64() {
    let a: f64 = f64::NAN;
    let b: f64 = 1.0;
    assert!(!a.nearly_lt_eps(&b, &0.0));
    assert!(!b.nearly_lt_eps(&a, &0.0));

    assert!(!f64::NAN.nearly_lt_eps(&f64::NAN, &0.0));
    assert!(!f64::NAN.nearly_lt_eps(&-f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_lt_eps(&f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_lt_eps(&-f64::NAN, &0.0));
}

///////////////////
// nearly_le_eps //
///////////////////

#[test]
fn nearly_le_eps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(a.nearly_le_eps(&b, &0.0));
    assert!(b.nearly_le_eps(&a, &0.0));
}

#[test]
fn nearly_le_eps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(a.nearly_le_eps(&b, &0.0));
    assert!(b.nearly_le_eps(&a, &0.0));
}

#[test]
fn nearly_le_eps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(a.nearly_le_eps(&b, &0.0));
    assert!(b.nearly_le_eps(&a, &0.0));
}

#[test]
fn nearly_le_eps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(a.nearly_le_eps(&b, &0.0));
    assert!(b.nearly_le_eps(&a, &0.0));
}

#[test]
fn nearly_le_eps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;

    assert!(a.nearly_le_eps(&b, &0.0000007));
    assert!(!b.nearly_le_eps(&a, &0.0000007));

    assert!(a.nearly_le_eps(&b, &0.0000008));
    assert!(!b.nearly_le_eps(&a, &0.0000008));
    
    assert!(a.nearly_le_eps(&b, &0.0000009));
    assert!(b.nearly_le_eps(&a, &0.0000009));
}

#[test]
fn nearly_le_eps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000003;

    assert!(a.nearly_le_eps(&b, &0.0000000000002));
    assert!(!b.nearly_le_eps(&a, &0.0000000000002));

    assert!(a.nearly_le_eps(&b, &0.0000000000003));
    assert!(b.nearly_le_eps(&a, &0.0000000000003));
    
    assert!(a.nearly_le_eps(&b, &0.0000000000004));
    assert!(b.nearly_le_eps(&a, &0.0000000000004));
}

#[test]
fn nearly_le_eps_different_sign_f32() {
    let a: f32 = -0.01;
    let b: f32 = 0.01;

    assert!(a.nearly_le_eps(&b, &0.019));
    assert!(!b.nearly_le_eps(&a, &0.019));

    assert!(a.nearly_le_eps(&b, &0.02));
    assert!(b.nearly_le_eps(&a, &0.02));
}

#[test]
fn nearly_le_eps_different_sign_f64() {
    let a: f64 = -0.01;
    let b: f64 = 0.01;

    assert!(a.nearly_le_eps(&b, &0.019));
    assert!(!b.nearly_le_eps(&a, &0.019));

    assert!(a.nearly_le_eps(&b, &0.02));
    assert!(b.nearly_le_eps(&a, &0.02));
}

#[test]
fn nearly_le_eps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = f32::INFINITY;
        assert!(a.nearly_le_eps(&b, &f32::MAX));
        assert!(!b.nearly_le_eps(&a, &f32::MAX));
    }
    {
        let a: f32 = -f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_le_eps(&b, &f32::MAX));
        assert!(!b.nearly_le_eps(&a, &f32::MAX));
    }

    assert!(f32::INFINITY.nearly_le_eps(&f32::INFINITY, &0.0));
    assert!((-f32::INFINITY).nearly_le_eps(&-f32::INFINITY, &0.0));
    assert!(!f32::INFINITY.nearly_le_eps(&-f32::INFINITY, &f32::MAX));
    assert!((-f32::INFINITY).nearly_le_eps(&f32::INFINITY, &f32::MAX));
}

#[test]
fn nearly_le_eps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = f64::INFINITY;
        assert!(a.nearly_le_eps(&b, &f64::MAX));
        assert!(!b.nearly_le_eps(&a, &f64::MAX));
    }
    {
        let a: f64 = -f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_le_eps(&b, &f64::MAX));
        assert!(!b.nearly_le_eps(&a, &f64::MAX));
    }

    assert!(f64::INFINITY.nearly_le_eps(&f64::INFINITY, &0.0));
    assert!((-f64::INFINITY).nearly_le_eps(&-f64::INFINITY, &0.0));
    assert!(!f64::INFINITY.nearly_le_eps(&-f64::INFINITY, &f64::MAX));
    assert!((-f64::INFINITY).nearly_le_eps(&f64::INFINITY, &f64::MAX));
}

#[test]
fn nearly_le_eps_nan_f32() {
    let a: f32 = f32::NAN;
    let b: f32 = 1.0;
    assert!(!a.nearly_le_eps(&b, &0.0));
    assert!(!b.nearly_le_eps(&a, &0.0));

    assert!(!f32::NAN.nearly_le_eps(&f32::NAN, &0.0));
    assert!(!f32::NAN.nearly_le_eps(&-f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_le_eps(&f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_le_eps(&-f32::NAN, &0.0));
}

#[test]
fn nearly_le_eps_nan_f64() {
    let a: f64 = f64::NAN;
    let b: f64 = 1.0;
    assert!(!a.nearly_le_eps(&b, &0.0));
    assert!(!b.nearly_le_eps(&a, &0.0));

    assert!(!f64::NAN.nearly_le_eps(&f64::NAN, &0.0));
    assert!(!f64::NAN.nearly_le_eps(&-f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_le_eps(&f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_le_eps(&-f64::NAN, &0.0));
}

///////////////////
// nearly_gt_eps //
///////////////////

#[test]
fn nearly_gt_eps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(!a.nearly_gt_eps(&b, &0.0));
    assert!(!b.nearly_gt_eps(&b, &0.0));   
}

#[test]
fn nearly_gt_eps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(!a.nearly_gt_eps(&b, &0.0));
    assert!(!b.nearly_gt_eps(&b, &0.0));   
}

#[test]
fn nearly_gt_eps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(!a.nearly_gt_eps(&b, &0.0));
    assert!(!b.nearly_gt_eps(&a, &0.0));
}

#[test]
fn nearly_gt_eps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(!a.nearly_gt_eps(&b, &0.0));
    assert!(!b.nearly_gt_eps(&a, &0.0));
}

#[test]
fn nearly_gt_eps_different_f32() {
    let a: f32 = 1.0000008;
    let b: f32 = 1.0;

    assert!(a.nearly_gt_eps(&b, &0.0000007));
    assert!(!b.nearly_gt_eps(&a, &0.0000007));

    assert!(a.nearly_gt_eps(&b, &0.0000008));
    assert!(!b.nearly_gt_eps(&a, &0.0000008));
    
    assert!(!a.nearly_gt_eps(&b, &0.0000009));
    assert!(!b.nearly_gt_eps(&a, &0.0000009));
}

#[test]
fn nearly_gt_eps_different_f64() {
    let a: f64 = 1.0000000000003;
    let b: f64 = 1.0;

    assert!(a.nearly_gt_eps(&b, &0.0000000000002));
    assert!(!b.nearly_gt_eps(&a, &0.0000000000002));

    assert!(!a.nearly_gt_eps(&b, &0.0000000000003));
    assert!(!b.nearly_gt_eps(&a, &0.0000000000003));
    
    assert!(!a.nearly_gt_eps(&b, &0.0000000000004));
    assert!(!b.nearly_gt_eps(&a, &0.0000000000004));
}

#[test]
fn nearly_gt_eps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(a.nearly_gt_eps(&b, &0.019));
    assert!(!b.nearly_gt_eps(&a, &0.019));

    assert!(!a.nearly_gt_eps(&b, &0.02));
    assert!(!b.nearly_gt_eps(&a, &0.02));
}

#[test]
fn nearly_gt_eps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(a.nearly_gt_eps(&b, &0.019));
    assert!(!b.nearly_gt_eps(&a, &0.019));

    assert!(!a.nearly_gt_eps(&b, &0.02));
    assert!(!b.nearly_gt_eps(&a, &0.02));
}

#[test]
fn nearly_gt_eps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = -f32::INFINITY;
        assert!(a.nearly_gt_eps(&b, &f32::MAX));
        assert!(!b.nearly_gt_eps(&a, &f32::MAX));
    }
    {
        let a: f32 = f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_gt_eps(&b, &f32::MAX));
        assert!(!b.nearly_gt_eps(&a, &f32::MAX));
    }

    assert!(!f32::INFINITY.nearly_gt_eps(&f32::INFINITY, &0.0));
    assert!(!(-f32::INFINITY).nearly_gt_eps(&-f32::INFINITY, &0.0));
    assert!(f32::INFINITY.nearly_gt_eps(&-f32::INFINITY, &f32::MAX));
    assert!(!(-f32::INFINITY).nearly_gt_eps(&f32::INFINITY, &f32::MAX));
}

#[test]
fn nearly_gt_eps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = -f64::INFINITY;
        assert!(a.nearly_gt_eps(&b, &f64::MAX));
        assert!(!b.nearly_gt_eps(&a, &f64::MAX));
    }
    {
        let a: f64 = f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_gt_eps(&b, &f64::MAX));
        assert!(!b.nearly_gt_eps(&a, &f64::MAX));
    }

    assert!(!f64::INFINITY.nearly_gt_eps(&f64::INFINITY, &0.0));
    assert!(!(-f64::INFINITY).nearly_gt_eps(&-f64::INFINITY, &0.0));
    assert!(f64::INFINITY.nearly_gt_eps(&-f64::INFINITY, &f64::MAX));
    assert!(!(-f64::INFINITY).nearly_gt_eps(&f64::INFINITY, &f64::MAX));
}

#[test]
fn nearly_gt_eps_nan_f32() {
    let a: f32 = 1.0;
    let b: f32 = f32::NAN;
    assert!(!a.nearly_gt_eps(&b, &0.0));
    assert!(!b.nearly_gt_eps(&a, &0.0));

    assert!(!f32::NAN.nearly_gt_eps(&f32::NAN, &0.0));
    assert!(!f32::NAN.nearly_gt_eps(&-f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_gt_eps(&f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_gt_eps(&-f32::NAN, &0.0));
}

#[test]
fn nearly_gt_eps_nan_f64() {
    let a: f64 = 1.0;
    let b: f64 = f64::NAN;
    assert!(!a.nearly_gt_eps(&b, &0.0));
    assert!(!b.nearly_gt_eps(&a, &0.0));

    assert!(!f64::NAN.nearly_gt_eps(&f64::NAN, &0.0));
    assert!(!f64::NAN.nearly_gt_eps(&-f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_gt_eps(&f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_gt_eps(&-f64::NAN, &0.0));
}

///////////////////
// nearly_ge_eps //
///////////////////

#[test]
fn nearly_ge_eps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(a.nearly_ge_eps(&b, &0.0));
    assert!(b.nearly_ge_eps(&a, &0.0));
}

#[test]
fn nearly_ge_eps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(a.nearly_ge_eps(&b, &0.0));
    assert!(b.nearly_ge_eps(&a, &0.0));
}

#[test]
fn nearly_ge_eps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(a.nearly_ge_eps(&b, &0.0));
    assert!(b.nearly_ge_eps(&a, &0.0));
}

#[test]
fn nearly_ge_eps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(a.nearly_ge_eps(&b, &0.0));
    assert!(b.nearly_ge_eps(&a, &0.0));
}

#[test]
fn nearly_ge_eps_different_f32() {
    let a: f32 = 1.0000008;
    let b: f32 = 1.0;

    assert!(a.nearly_ge_eps(&b, &0.0000007));
    assert!(!b.nearly_ge_eps(&a, &0.0000007));

    assert!(a.nearly_ge_eps(&b, &0.0000008));
    assert!(!b.nearly_ge_eps(&a, &0.0000008));
    
    assert!(a.nearly_ge_eps(&b, &0.0000009));
    assert!(b.nearly_ge_eps(&a, &0.0000009));
}

#[test]
fn nearly_ge_eps_different_f64() {
    let a: f64 = 1.0000000000003;
    let b: f64 = 1.0;

    assert!(a.nearly_ge_eps(&b, &0.0000000000002));
    assert!(!b.nearly_ge_eps(&a, &0.0000000000002));

    assert!(a.nearly_ge_eps(&b, &0.0000000000003));
    assert!(b.nearly_ge_eps(&a, &0.0000000000003));
    
    assert!(a.nearly_ge_eps(&b, &0.0000000000004));
    assert!(b.nearly_ge_eps(&a, &0.0000000000004));
}

#[test]
fn nearly_ge_eps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(a.nearly_ge_eps(&b, &0.019));
    assert!(!b.nearly_ge_eps(&a, &0.019));

    assert!(a.nearly_ge_eps(&b, &0.02));
    assert!(b.nearly_ge_eps(&a, &0.02));
}

#[test]
fn nearly_ge_eps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(a.nearly_ge_eps(&b, &0.019));
    assert!(!b.nearly_ge_eps(&a, &0.019));

    assert!(a.nearly_ge_eps(&b, &0.02));
    assert!(b.nearly_ge_eps(&a, &0.02));
}

#[test]
fn nearly_ge_eps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = -f32::INFINITY;
        assert!(a.nearly_ge_eps(&b, &f32::MAX));
        assert!(!b.nearly_ge_eps(&a, &f32::MAX));
    }
    {
        let a: f32 = f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_ge_eps(&b, &f32::MAX));
        assert!(!b.nearly_ge_eps(&a, &f32::MAX));
    }

    assert!(f32::INFINITY.nearly_ge_eps(&f32::INFINITY, &0.0));
    assert!((-f32::INFINITY).nearly_ge_eps(&-f32::INFINITY, &0.0));
    assert!(f32::INFINITY.nearly_ge_eps(&-f32::INFINITY, &f32::MAX));
    assert!(!(-f32::INFINITY).nearly_ge_eps(&f32::INFINITY, &f32::MAX));
}

#[test]
fn nearly_ge_eps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = -f64::INFINITY;
        assert!(a.nearly_ge_eps(&b, &f64::MAX));
        assert!(!b.nearly_ge_eps(&a, &f64::MAX));
    }
    {
        let a: f64 = f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_ge_eps(&b, &f64::MAX));
        assert!(!b.nearly_ge_eps(&a, &f64::MAX));
    }

    assert!(f64::INFINITY.nearly_ge_eps(&f64::INFINITY, &0.0));
    assert!((-f64::INFINITY).nearly_ge_eps(&-f64::INFINITY, &0.0));
    assert!(f64::INFINITY.nearly_ge_eps(&-f64::INFINITY, &f64::MAX));
    assert!(!(-f64::INFINITY).nearly_ge_eps(&f64::INFINITY, &f64::MAX));
}

#[test]
fn nearly_ge_eps_nan_f32() {
    let a: f32 = f32::NAN;
    let b: f32 = 1.0;
    assert!(!a.nearly_ge_eps(&b, &0.0));
    assert!(!b.nearly_ge_eps(&a, &0.0));

    assert!(!f32::NAN.nearly_ge_eps(&f32::NAN, &0.0));
    assert!(!f32::NAN.nearly_ge_eps(&-f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_ge_eps(&f32::NAN, &0.0));
    assert!(!(-f32::NAN).nearly_ge_eps(&-f32::NAN, &0.0));
}

#[test]
fn nearly_ge_eps_nan_f64() {
    let a: f64 = f64::NAN;
    let b: f64 = 1.0;
    assert!(!a.nearly_ge_eps(&b, &0.0));
    assert!(!b.nearly_ge_eps(&a, &0.0));

    assert!(!f64::NAN.nearly_ge_eps(&f64::NAN, &0.0));
    assert!(!f64::NAN.nearly_ge_eps(&-f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_ge_eps(&f64::NAN, &0.0));
    assert!(!(-f64::NAN).nearly_ge_eps(&-f64::NAN, &0.0));
}
