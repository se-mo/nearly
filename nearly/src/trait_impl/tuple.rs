use crate::nearly_eq::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use crate::nearly_ord::{NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps};
use crate::tolerance::{
    EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance, UlpsToleranceType,
};

macro_rules! impl_tuple {
    ($lhs:ident, $rhs:ident, $idx: tt) => {
        impl_tuple!(@impl $lhs, $rhs, $idx);
    };
    ($lhs:ident $( $lhs_tail:ident )+, $rhs:ident $( $rhs_tail:ident )+, $idx:tt $( $idx_tail:tt )+) => {
        impl_tuple!($( $lhs_tail )+, $( $rhs_tail )+, $( $idx_tail )+);
        impl_tuple!(@impl $lhs $( $lhs_tail )+, $rhs $( $rhs_tail )+, $idx $( $idx_tail )+);
    };
    (@impl $( $lhs: ident )+, $( $rhs: ident )+, $( $idx: tt )+) => {
        ///////////////
        // nearly_eq //
        ///////////////

        impl<Lhs, Rhs> NearlyEqEps<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
        {
            fn nearly_eq_eps(&self, other: &($($rhs,)+), eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_eq_eps(&other.$idx, eps) )&&+
            }
        }

        impl<Lhs, Rhs> NearlyEqUlps<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyEqUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_eq_ulps(&self, other: &($($rhs,)+), ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_eq_ulps(&other.$idx, ulps) )&&+
            }
        }

        impl<Lhs, Rhs> NearlyEqTol<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyEqTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_eq_tol(&self, other: &($($rhs,)+), tol: &Tolerance<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_eq_tol(&other.$idx, tol) )&&+
            }
        }

        impl<Lhs, Rhs> NearlyEq<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyEq<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
        }

        ////////////////
        // nearly_ord //
        ////////////////

        impl<Lhs, Rhs> NearlyOrdEps<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
        {
            fn nearly_lt_eps(&self, other: &($($rhs,)+), eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_lt_eps(&other.$idx, eps) )&&+
            }

            fn nearly_le_eps(&self, other: &($($rhs,)+), eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_le_eps(&other.$idx, eps) )&&+
            }

            fn nearly_gt_eps(&self, other: &($($rhs,)+), eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_gt_eps(&other.$idx, eps) )&&+
            }

            fn nearly_ge_eps(&self, other: &($($rhs,)+), eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_ge_eps(&other.$idx, eps) )&&+
            }
        }

        impl<Lhs, Rhs> NearlyOrdUlps<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_lt_ulps(&self, other: &($($rhs,)+), ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_lt_ulps(&other.$idx, ulps) )&&+
            }

            fn nearly_le_ulps(&self, other: &($($rhs,)+), ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_le_ulps(&other.$idx, ulps) )&&+
            }

            fn nearly_gt_ulps(&self, other: &($($rhs,)+), ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_gt_ulps(&other.$idx, ulps) )&&+
            }

            fn nearly_ge_ulps(&self, other: &($($rhs,)+), ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_ge_ulps(&other.$idx, ulps) )&&+
            }
        }

        impl<Lhs, Rhs> NearlyOrdTol<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_lt_tol(&self, other: &($($rhs,)+), tol: &Tolerance<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_lt_tol(&other.$idx, tol) )&&+
            }

            fn nearly_le_tol(&self, other: &($($rhs,)+), tol: &Tolerance<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_le_tol(&other.$idx, tol) )&&+
            }

            fn nearly_gt_tol(&self, other: &($($rhs,)+), tol: &Tolerance<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_gt_tol(&other.$idx, tol) )&&+
            }

            fn nearly_ge_tol(&self, other: &($($rhs,)+), tol: &Tolerance<Lhs, Rhs>) -> bool {
                $( self.$idx.nearly_ge_tol(&other.$idx, tol) )&&+
            }
        }

        impl<Lhs, Rhs> NearlyOrd<($($rhs,)+), Lhs, Rhs> for ($($lhs,)+)
        where
            Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
        }
    }
}

impl_tuple!(
    Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs,
    Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs,
    11 10 9 8 7 6 5 4 3 2 1 0);
