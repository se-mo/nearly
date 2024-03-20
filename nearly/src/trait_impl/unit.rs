use crate::nearly_eq::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use crate::nearly_ord::{NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps};
use crate::tolerance::{EpsToleranceType, UlpsToleranceType};

///////////////
// nearly_eq //
///////////////

impl NearlyEqEps for () {
    #[inline]
    fn nearly_eq_eps(&self, _other: &Self, _eps: &EpsToleranceType<Self>) -> bool {
        true
    }
}

impl NearlyEqUlps for () {
    #[inline]
    fn nearly_eq_ulps(&self, _other: &Self, _ulps: &UlpsToleranceType<Self>) -> bool {
        true
    }
}

impl NearlyEqTol for () {}
impl NearlyEq for () {}

////////////////
// nearly_ord //
////////////////

impl NearlyOrdEps for () {
    #[inline]
    fn nearly_lt_eps(&self, _other: &Self, _eps: &EpsToleranceType<Self>) -> bool {
        false
    }

    #[inline]
    fn nearly_gt_eps(&self, _other: &Self, _eps: &EpsToleranceType<Self>) -> bool {
        false
    }
}

impl NearlyOrdUlps for () {
    #[inline]
    fn nearly_lt_ulps(&self, _other: &Self, _ulps: &UlpsToleranceType<Self>) -> bool {
        false
    }

    #[inline]
    fn nearly_gt_ulps(&self, _other: &Self, _ulps: &UlpsToleranceType<Self>) -> bool {
        false
    }
}

impl NearlyOrdTol for () {}
impl NearlyOrd for () {}
