#![cfg(feature = "std")]

use crate::nearly_eq::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use crate::nearly_ord::{NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps};
use crate::tolerance::{
    EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance, UlpsToleranceType,
};

use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;

macro_rules! impl_pointer {
    ($lhs: ty, $rhs: ty) => {
        ///////////////
        // nearly_eq //
        ///////////////

        impl<Lhs, Rhs> NearlyEqEps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_eq_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyEqEps::nearly_eq_eps(&**self, &**other, eps)
            }
        }

        impl<Lhs, Rhs> NearlyEqUlps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_eq_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyEqUlps::nearly_eq_ulps(&**self, &**other, ulps)
            }
        }

        impl<Lhs, Rhs> NearlyEqTol<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_eq_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyEqTol::nearly_eq_tol(&**self, &**other, tol)
            }
        }

        impl<Lhs, Rhs> NearlyEq<$rhs, Lhs, Rhs> for $lhs where
            Lhs: NearlyEq<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>
        {
        }

        ////////////////
        // nearly_ord //
        ////////////////

        impl<Lhs, Rhs> NearlyOrdEps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_lt_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_lt_eps(&**self, &**other, eps)
            }

            #[inline]
            fn nearly_le_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_le_eps(&**self, &**other, eps)
            }

            #[inline]
            fn nearly_gt_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_gt_eps(&**self, &**other, eps)
            }

            #[inline]
            fn nearly_ge_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_ge_eps(&**self, &**other, eps)
            }
        }

        impl<Lhs, Rhs> NearlyOrdUlps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_lt_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_lt_ulps(&**self, &**other, ulps)
            }

            #[inline]
            fn nearly_le_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_le_ulps(&**self, &**other, ulps)
            }

            #[inline]
            fn nearly_gt_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_gt_ulps(&**self, &**other, ulps)
            }

            #[inline]
            fn nearly_ge_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_ge_ulps(&**self, &**other, ulps)
            }
        }

        impl<Lhs, Rhs> NearlyOrdTol<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_lt_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_lt_tol(&**self, &**other, tol)
            }

            #[inline]
            fn nearly_le_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_le_tol(&**self, &**other, tol)
            }

            #[inline]
            fn nearly_gt_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_gt_tol(&**self, &**other, tol)
            }

            #[inline]
            fn nearly_ge_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_ge_tol(&**self, &**other, tol)
            }
        }

        impl<Lhs, Rhs> NearlyOrd<$rhs, Lhs, Rhs> for $lhs where
            Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>
        {
        }
    };
}

impl_pointer!(Arc<Lhs>, Arc<Rhs>);
impl_pointer!(Box<Lhs>, Box<Rhs>);
impl_pointer!(Rc<Lhs>, Rc<Rhs>);
