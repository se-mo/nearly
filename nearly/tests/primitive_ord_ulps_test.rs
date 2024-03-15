use nearly::NearlyOrdUlps;

////////////////////
// nearly_lt_ulps //
////////////////////

#[test]
fn nearly_lt_ulps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(!a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &0));
}

#[test]
fn nearly_lt_ulps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(!a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &0));
}

#[test]
fn nearly_lt_ulps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(!a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &0));
}

#[test]
fn nearly_lt_ulps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(!a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &0));
}

#[test]
fn nearly_lt_ulps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;

    assert!(a.nearly_lt_ulps(&b, &6));
    assert!(!b.nearly_lt_ulps(&a, &6));

    assert!(!a.nearly_lt_ulps(&b, &7));
    assert!(!b.nearly_lt_ulps(&a, &7));
    
    assert!(!a.nearly_lt_ulps(&b, &8));
    assert!(!b.nearly_lt_ulps(&a, &8));
}

#[test]
fn nearly_lt_ulps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000016;

    assert!(a.nearly_lt_ulps(&b, &6));
    assert!(!b.nearly_lt_ulps(&a, &6));

    assert!(!a.nearly_lt_ulps(&b, &7));
    assert!(!b.nearly_lt_ulps(&a, &7));
    
    assert!(!a.nearly_lt_ulps(&b, &8));
    assert!(!b.nearly_lt_ulps(&a, &8));
}

#[test]
fn nearly_lt_ulps_different_sign_f32() {
    let a: f32 = -0.01;
    let b: f32 = 0.01;

    assert!(a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &i32::MAX));

    assert!(a.nearly_lt_ulps(&b, &i32::MAX));
    assert!(!b.nearly_lt_ulps(&a, &i32::MAX));
}

#[test]
fn nearly_lt_ulps_different_sign_f64() {
    let a: f64 = -0.01;
    let b: f64 = 0.01;

    assert!(a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &i64::MAX));

    assert!(a.nearly_lt_ulps(&b, &i64::MAX));
    assert!(!b.nearly_lt_ulps(&a, &i64::MAX));
}

#[test]
fn nearly_lt_ulps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = f32::INFINITY;
        assert!(a.nearly_lt_ulps(&b, &(i32::MAX >> 1)));
        assert!(!b.nearly_lt_ulps(&a, &(i32::MAX >> 1)));
    }
    {
        let a: f32 = -f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_lt_ulps(&b, &i32::MAX));
        assert!(!b.nearly_lt_ulps(&a, &i32::MAX));
    }

    assert!(!f32::INFINITY.nearly_lt_ulps(&f32::INFINITY, &0));
    assert!(!(-f32::INFINITY).nearly_lt_ulps(&-f32::INFINITY, &0));
    assert!(!f32::INFINITY.nearly_lt_ulps(&-f32::INFINITY, &i32::MAX));
    assert!((-f32::INFINITY).nearly_lt_ulps(&f32::INFINITY, &i32::MAX));
}

#[test]
fn nearly_lt_ulps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = f64::INFINITY;
        assert!(a.nearly_lt_ulps(&b, &(i64::MAX >> 1)));
        assert!(!b.nearly_lt_ulps(&a, &(i64::MAX >> 1)));
    }
    {
        let a: f64 = -f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_lt_ulps(&b, &i64::MAX));
        assert!(!b.nearly_lt_ulps(&a, &i64::MAX));
    }

    assert!(!f64::INFINITY.nearly_lt_ulps(&f64::INFINITY, &0));
    assert!(!(-f64::INFINITY).nearly_lt_ulps(&-f64::INFINITY, &0));
    assert!(!f64::INFINITY.nearly_lt_ulps(&-f64::INFINITY, &i64::MAX));
    assert!((-f64::INFINITY).nearly_lt_ulps(&f64::INFINITY, &i64::MAX));
}

#[test]
fn nearly_lt_ulps_nan_f32() {
    let a: f32 = f32::NAN;
    let b: f32 = 1.0;
    assert!(!a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &0));

    assert!(!f32::NAN.nearly_lt_ulps(&f32::NAN, &0));
    assert!(!f32::NAN.nearly_lt_ulps(&-f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_lt_ulps(&f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_lt_ulps(&-f32::NAN, &0));
}

#[test]
fn nearly_lt_ulps_nan_f64() {
    let a: f64 = f64::NAN;
    let b: f64 = 1.0;
    assert!(!a.nearly_lt_ulps(&b, &0));
    assert!(!b.nearly_lt_ulps(&a, &0));

    assert!(!f64::NAN.nearly_lt_ulps(&f64::NAN, &0));
    assert!(!f64::NAN.nearly_lt_ulps(&-f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_lt_ulps(&f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_lt_ulps(&-f64::NAN, &0));
}

////////////////////
// nearly_le_ulps //
////////////////////

#[test]
fn nearly_le_ulps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(a.nearly_le_ulps(&b, &0));
    assert!(b.nearly_le_ulps(&a, &0));
}

#[test]
fn nearly_le_ulps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(a.nearly_le_ulps(&b, &0));
    assert!(b.nearly_le_ulps(&a, &0));
}

#[test]
fn nearly_le_ulps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(a.nearly_le_ulps(&b, &0));
    assert!(b.nearly_le_ulps(&a, &0));
}

#[test]
fn nearly_le_ulps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(a.nearly_le_ulps(&b, &0));
    assert!(b.nearly_le_ulps(&a, &0));
}

#[test]
fn nearly_le_ulps_different_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0000008;

    assert!(a.nearly_le_ulps(&b, &6));
    assert!(!b.nearly_le_ulps(&a, &6));

    assert!(a.nearly_le_ulps(&b, &7));
    assert!(b.nearly_le_ulps(&a, &7));
    
    assert!(a.nearly_le_ulps(&b, &8));
    assert!(b.nearly_le_ulps(&a, &8));
}

#[test]
fn nearly_le_ulps_different_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0000000000000016;

    assert!(a.nearly_le_ulps(&b, &6));
    assert!(!b.nearly_le_ulps(&a, &6));

    assert!(a.nearly_le_ulps(&b, &7));
    assert!(b.nearly_le_ulps(&a, &7));
    
    assert!(a.nearly_le_ulps(&b, &8));
    assert!(b.nearly_le_ulps(&a, &8));
}

#[test]
fn nearly_le_ulps_different_sign_f32() {
    let a: f32 = -0.01;
    let b: f32 = 0.01;

    assert!(a.nearly_le_ulps(&b, &0));
    assert!(!b.nearly_le_ulps(&a, &i32::MAX));

    assert!(a.nearly_le_ulps(&b, &i32::MAX));
    assert!(!b.nearly_le_ulps(&a, &i32::MAX));
}

#[test]
fn nearly_le_ulps_different_sign_f64() {
    let a: f64 = -0.01;
    let b: f64 = 0.01;

    assert!(a.nearly_le_ulps(&b, &0));
    assert!(!b.nearly_le_ulps(&a, &i64::MAX));

    assert!(a.nearly_le_ulps(&b, &i64::MAX));
    assert!(!b.nearly_le_ulps(&a, &i64::MAX));
}

#[test]
fn nearly_le_ulps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = f32::INFINITY;
        assert!(a.nearly_le_ulps(&b, &(i32::MAX >> 1)));
        assert!(!b.nearly_le_ulps(&a, &(i32::MAX >> 1)));
    }
    {
        let a: f32 = -f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_le_ulps(&b, &i32::MAX));
        assert!(!b.nearly_le_ulps(&a, &i32::MAX));
    }

    assert!(f32::INFINITY.nearly_le_ulps(&f32::INFINITY, &0));
    assert!((-f32::INFINITY).nearly_le_ulps(&-f32::INFINITY, &0));
    assert!(!f32::INFINITY.nearly_le_ulps(&-f32::INFINITY, &i32::MAX));
    assert!((-f32::INFINITY).nearly_le_ulps(&f32::INFINITY, &i32::MAX));
}

#[test]
fn nearly_le_ulps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = f64::INFINITY;
        assert!(a.nearly_le_ulps(&b, &(i64::MAX >> 1)));
        assert!(!b.nearly_le_ulps(&a, &(i64::MAX >> 1)));
    }
    {
        let a: f64 = -f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_le_ulps(&b, &i64::MAX));
        assert!(!b.nearly_le_ulps(&a, &i64::MAX));
    }

    assert!(f64::INFINITY.nearly_le_ulps(&f64::INFINITY, &0));
    assert!((-f64::INFINITY).nearly_le_ulps(&-f64::INFINITY, &0));
    assert!(!f64::INFINITY.nearly_le_ulps(&-f64::INFINITY, &i64::MAX));
    assert!((-f64::INFINITY).nearly_le_ulps(&f64::INFINITY, &i64::MAX));
}

#[test]
fn nearly_le_ulps_nan_f32() {
    let a: f32 = f32::NAN;
    let b: f32 = 1.0;
    assert!(!a.nearly_le_ulps(&b, &0));
    assert!(!b.nearly_le_ulps(&a, &0));

    assert!(!f32::NAN.nearly_le_ulps(&f32::NAN, &0));
    assert!(!f32::NAN.nearly_le_ulps(&-f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_le_ulps(&f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_le_ulps(&-f32::NAN, &0));
}

#[test]
fn nearly_le_ulps_nan_f64() {
    let a: f64 = f64::NAN;
    let b: f64 = 1.0;
    assert!(!a.nearly_le_ulps(&b, &0));
    assert!(!b.nearly_le_ulps(&a, &0));

    assert!(!f64::NAN.nearly_le_ulps(&f64::NAN, &0));
    assert!(!f64::NAN.nearly_le_ulps(&-f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_le_ulps(&f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_le_ulps(&-f64::NAN, &0));
}

////////////////////
// nearly_gt_ulps //
////////////////////

#[test]
fn nearly_gt_ulps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(!a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &0));
}

#[test]
fn nearly_gt_ulps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(!a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &0));
}

#[test]
fn nearly_gt_ulps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(!a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &0));
}

#[test]
fn nearly_gt_ulps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(!a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &0));
}

#[test]
fn nearly_gt_ulps_different_f32() {
    let a: f32 = 1.0000008;
    let b: f32 = 1.0;

    assert!(a.nearly_gt_ulps(&b, &6));
    assert!(!b.nearly_gt_ulps(&a, &6));

    assert!(!a.nearly_gt_ulps(&b, &7));
    assert!(!b.nearly_gt_ulps(&a, &7));
    
    assert!(!a.nearly_gt_ulps(&b, &8));
    assert!(!b.nearly_gt_ulps(&a, &8));
}

#[test]
fn nearly_gt_ulps_different_f64() {
    let a: f64 = 1.0000000000000016;
    let b: f64 = 1.0;

    assert!(a.nearly_gt_ulps(&b, &6));
    assert!(!b.nearly_gt_ulps(&a, &6));

    assert!(!a.nearly_gt_ulps(&b, &7));
    assert!(!b.nearly_gt_ulps(&a, &7));
    
    assert!(!a.nearly_gt_ulps(&b, &8));
    assert!(!b.nearly_gt_ulps(&a, &8));
}

#[test]
fn nearly_gt_ulps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &i32::MAX));

    assert!(a.nearly_gt_ulps(&b, &i32::MAX));
    assert!(!b.nearly_gt_ulps(&a, &i32::MAX));
}

#[test]
fn nearly_gt_ulps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &i64::MAX));

    assert!(a.nearly_gt_ulps(&b, &i64::MAX));
    assert!(!b.nearly_gt_ulps(&a, &i64::MAX));
}

#[test]
fn nearly_gt_ulps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = -f32::INFINITY;
        assert!(a.nearly_gt_ulps(&b, &i32::MAX));
        assert!(!b.nearly_gt_ulps(&a, &i32::MAX));
    }
    {
        let a: f32 = f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_gt_ulps(&b, &(i32::MAX >> 1)));
        assert!(!b.nearly_gt_ulps(&a, &(i32::MAX >> 1)));
    }

    assert!(!f32::INFINITY.nearly_gt_ulps(&f32::INFINITY, &0));
    assert!(!(-f32::INFINITY).nearly_gt_ulps(&-f32::INFINITY, &0));
    assert!(f32::INFINITY.nearly_gt_ulps(&-f32::INFINITY, &i32::MAX));
    assert!(!(-f32::INFINITY).nearly_gt_ulps(&f32::INFINITY, &i32::MAX));
}

#[test]
fn nearly_gt_ulps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = -f64::INFINITY;
        assert!(a.nearly_gt_ulps(&b, &i64::MAX));
        assert!(!b.nearly_gt_ulps(&a, &i64::MAX));
    }
    {
        let a: f64 = f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_gt_ulps(&b, &(i64::MAX >> 1)));
        assert!(!b.nearly_gt_ulps(&a, &(i64::MAX >> 1)));
    }

    assert!(!f64::INFINITY.nearly_gt_ulps(&f64::INFINITY, &0));
    assert!(!(-f64::INFINITY).nearly_gt_ulps(&-f64::INFINITY, &0));
    assert!(f64::INFINITY.nearly_gt_ulps(&-f64::INFINITY, &i64::MAX));
    assert!(!(-f64::INFINITY).nearly_gt_ulps(&f64::INFINITY, &i64::MAX));
}

#[test]
fn nearly_gt_ulps_nan_f32() {
    let a: f32 = f32::NAN;
    let b: f32 = 1.0;
    assert!(!a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &0));

    assert!(!f32::NAN.nearly_gt_ulps(&f32::NAN, &0));
    assert!(!f32::NAN.nearly_gt_ulps(&-f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_gt_ulps(&f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_gt_ulps(&-f32::NAN, &0));
}

#[test]
fn nearly_gt_ulps_nan_f64() {
    let a: f64 = f64::NAN;
    let b: f64 = 1.0;
    assert!(!a.nearly_gt_ulps(&b, &0));
    assert!(!b.nearly_gt_ulps(&a, &0));

    assert!(!f64::NAN.nearly_gt_ulps(&f64::NAN, &0));
    assert!(!f64::NAN.nearly_gt_ulps(&-f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_gt_ulps(&f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_gt_ulps(&-f64::NAN, &0));
}

////////////////////
// nearly_ge_ulps //
////////////////////

#[test]
fn nearly_ge_ulps_zero_f32() {
    let a: f32 = 0.0;
    let b: f32 = -0.0;
    assert!(a.nearly_ge_ulps(&b, &0));
    assert!(b.nearly_ge_ulps(&a, &0));
}

#[test]
fn nearly_ge_ulps_zero_f64() {
    let a: f64 = 0.0;
    let b: f64 = -0.0;
    assert!(a.nearly_ge_ulps(&b, &0));
    assert!(b.nearly_ge_ulps(&a, &0));
}

#[test]
fn nearly_ge_ulps_same_f32() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    assert!(a.nearly_ge_ulps(&b, &0));
    assert!(b.nearly_ge_ulps(&a, &0));
}

#[test]
fn nearly_ge_ulps_same_f64() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    assert!(a.nearly_ge_ulps(&b, &0));
    assert!(b.nearly_ge_ulps(&a, &0));
}

#[test]
fn nearly_ge_ulps_different_f32() {
    let a: f32 = 1.0000008;
    let b: f32 = 1.0;

    assert!(a.nearly_ge_ulps(&b, &6));
    assert!(!b.nearly_ge_ulps(&a, &6));

    assert!(a.nearly_ge_ulps(&b, &7));
    assert!(b.nearly_ge_ulps(&a, &7));
    
    assert!(a.nearly_ge_ulps(&b, &8));
    assert!(b.nearly_ge_ulps(&a, &8));
}

#[test]
fn nearly_ge_ulps_different_f64() {
    let a: f64 = 1.0000000000000016;
    let b: f64 = 1.0;

    assert!(a.nearly_ge_ulps(&b, &6));
    assert!(!b.nearly_ge_ulps(&a, &6));

    assert!(a.nearly_ge_ulps(&b, &7));
    assert!(b.nearly_ge_ulps(&a, &7));
    
    assert!(a.nearly_ge_ulps(&b, &8));
    assert!(b.nearly_ge_ulps(&a, &8));
}

#[test]
fn nearly_ge_ulps_different_sign_f32() {
    let a: f32 = 0.01;
    let b: f32 = -0.01;

    assert!(a.nearly_ge_ulps(&b, &0));
    assert!(!b.nearly_ge_ulps(&a, &i32::MAX));

    assert!(a.nearly_ge_ulps(&b, &i32::MAX));
    assert!(!b.nearly_ge_ulps(&a, &i32::MAX));
}

#[test]
fn nearly_ge_ulps_different_sign_f64() {
    let a: f64 = 0.01;
    let b: f64 = -0.01;

    assert!(a.nearly_ge_ulps(&b, &0));
    assert!(!b.nearly_ge_ulps(&a, &i64::MAX));

    assert!(a.nearly_ge_ulps(&b, &i64::MAX));
    assert!(!b.nearly_ge_ulps(&a, &i64::MAX));
}

#[test]
fn nearly_ge_ulps_inf_f32() {
    {
        let a: f32 = 1.0;
        let b: f32 = -f32::INFINITY;
        assert!(a.nearly_ge_ulps(&b, &i32::MAX));
        assert!(!b.nearly_ge_ulps(&a, &i32::MAX));
    }
    {
        let a: f32 = f32::INFINITY;
        let b: f32 = 1.0;
        assert!(a.nearly_ge_ulps(&b, &(i32::MAX >> 1)));
        assert!(!b.nearly_ge_ulps(&a, &(i32::MAX >> 1)));
    }

    assert!(f32::INFINITY.nearly_ge_ulps(&f32::INFINITY, &0));
    assert!((-f32::INFINITY).nearly_ge_ulps(&-f32::INFINITY, &0));
    assert!(f32::INFINITY.nearly_ge_ulps(&-f32::INFINITY, &i32::MAX));
    assert!(!(-f32::INFINITY).nearly_ge_ulps(&f32::INFINITY, &i32::MAX));
}

#[test]
fn nearly_ge_ulps_inf_f64() {
    {
        let a: f64 = 1.0;
        let b: f64 = -f64::INFINITY;
        assert!(a.nearly_ge_ulps(&b, &i64::MAX));
        assert!(!b.nearly_ge_ulps(&a, &i64::MAX));
    }
    {
        let a: f64 = f64::INFINITY;
        let b: f64 = 1.0;
        assert!(a.nearly_ge_ulps(&b, &(i64::MAX >> 1)));
        assert!(!b.nearly_ge_ulps(&a, &(i64::MAX >> 1)));
    }

    assert!(f64::INFINITY.nearly_ge_ulps(&f64::INFINITY, &0));
    assert!((-f64::INFINITY).nearly_ge_ulps(&-f64::INFINITY, &0));
    assert!(f64::INFINITY.nearly_ge_ulps(&-f64::INFINITY, &i64::MAX));
    assert!(!(-f64::INFINITY).nearly_ge_ulps(&f64::INFINITY, &i64::MAX));
}

#[test]
fn nearly_ge_ulps_nan_f32() {
    let a: f32 = f32::NAN;
    let b: f32 = 1.0;
    assert!(!a.nearly_ge_ulps(&b, &0));
    assert!(!b.nearly_ge_ulps(&a, &0));

    assert!(!f32::NAN.nearly_ge_ulps(&f32::NAN, &0));
    assert!(!f32::NAN.nearly_ge_ulps(&-f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_ge_ulps(&f32::NAN, &0));
    assert!(!(-f32::NAN).nearly_ge_ulps(&-f32::NAN, &0));
}

#[test]
fn nearly_ge_ulps_nan_f64() {
    let a: f64 = f64::NAN;
    let b: f64 = 1.0;
    assert!(!a.nearly_ge_ulps(&b, &0));
    assert!(!b.nearly_ge_ulps(&a, &0));

    assert!(!f64::NAN.nearly_ge_ulps(&f64::NAN, &0));
    assert!(!f64::NAN.nearly_ge_ulps(&-f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_ge_ulps(&f64::NAN, &0));
    assert!(!(-f64::NAN).nearly_ge_ulps(&-f64::NAN, &0));
}
