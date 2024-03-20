#![cfg(feature = "std")]

use crate::nearly_eq::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use crate::nearly_ord::{NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps};
use crate::tolerance::{
    EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance, UlpsToleranceType,
};

use std::ops::Deref;
use std::pin::Pin;

///////////////
// nearly_eq //
///////////////

impl<Lhs: Deref, Rhs: Deref> NearlyEqEps<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
where
    Lhs::Target: NearlyEqEps<Rhs::Target> + EpsTolerance<Rhs::Target>,
{
    #[inline]
    fn nearly_eq_eps(
        &self,
        other: &Pin<Rhs>,
        eps: &EpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_eq_eps(self, other, eps)
    }
}

impl<Lhs: Deref, Rhs: Deref> NearlyEqUlps<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
where
    Lhs::Target: NearlyEqUlps<Rhs::Target> + UlpsTolerance<Rhs::Target>,
{
    #[inline]
    fn nearly_eq_ulps(
        &self,
        other: &Pin<Rhs>,
        ulps: &UlpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_eq_ulps(self, other, ulps)
    }
}

impl<Lhs: Deref, Rhs: Deref> NearlyEqTol<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
where
    Lhs::Target: NearlyEqTol<Rhs::Target> + EpsTolerance<Rhs::Target> + UlpsTolerance<Rhs::Target>,
{
    #[inline]
    fn nearly_eq_tol(&self, other: &Pin<Rhs>, tol: &Tolerance<Lhs::Target, Rhs::Target>) -> bool {
        Lhs::Target::nearly_eq_tol(self, other, tol)
    }
}

impl<Lhs: Deref, Rhs: Deref> NearlyEq<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs> where
    Lhs::Target: NearlyEq<Rhs::Target> + EpsTolerance<Rhs::Target> + UlpsTolerance<Rhs::Target>
{
}

////////////////
// nearly_ord //
////////////////

impl<Lhs: Deref, Rhs: Deref> NearlyOrdEps<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
where
    Lhs::Target: NearlyOrdEps<Rhs::Target> + EpsTolerance<Rhs::Target>,
{
    #[inline]
    fn nearly_lt_eps(
        &self,
        other: &Pin<Rhs>,
        eps: &EpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_lt_eps(self, other, eps)
    }

    #[inline]
    fn nearly_le_eps(
        &self,
        other: &Pin<Rhs>,
        eps: &EpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_le_eps(self, other, eps)
    }

    #[inline]
    fn nearly_gt_eps(
        &self,
        other: &Pin<Rhs>,
        eps: &EpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_gt_eps(self, other, eps)
    }

    #[inline]
    fn nearly_ge_eps(
        &self,
        other: &Pin<Rhs>,
        eps: &EpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_ge_eps(self, other, eps)
    }
}

impl<Lhs: Deref, Rhs: Deref> NearlyOrdUlps<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
where
    Lhs::Target: NearlyOrdUlps<Rhs::Target> + UlpsTolerance<Rhs::Target>,
{
    #[inline]
    fn nearly_lt_ulps(
        &self,
        other: &Pin<Rhs>,
        ulps: &UlpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_lt_ulps(self, other, ulps)
    }

    #[inline]
    fn nearly_le_ulps(
        &self,
        other: &Pin<Rhs>,
        ulps: &UlpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_le_ulps(self, other, ulps)
    }

    #[inline]
    fn nearly_gt_ulps(
        &self,
        other: &Pin<Rhs>,
        ulps: &UlpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_gt_ulps(self, other, ulps)
    }

    #[inline]
    fn nearly_ge_ulps(
        &self,
        other: &Pin<Rhs>,
        ulps: &UlpsToleranceType<Lhs::Target, Rhs::Target>,
    ) -> bool {
        Lhs::Target::nearly_ge_ulps(self, other, ulps)
    }
}

impl<Lhs: Deref, Rhs: Deref> NearlyOrdTol<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
where
    Lhs::Target: NearlyOrdTol<Rhs::Target> + EpsTolerance<Rhs::Target> + UlpsTolerance<Rhs::Target>,
{
    #[inline]
    fn nearly_lt_tol(&self, other: &Pin<Rhs>, tol: &Tolerance<Lhs::Target, Rhs::Target>) -> bool {
        Lhs::Target::nearly_lt_tol(self, other, tol)
    }

    #[inline]
    fn nearly_le_tol(&self, other: &Pin<Rhs>, tol: &Tolerance<Lhs::Target, Rhs::Target>) -> bool {
        Lhs::Target::nearly_le_tol(self, other, tol)
    }

    #[inline]
    fn nearly_gt_tol(&self, other: &Pin<Rhs>, tol: &Tolerance<Lhs::Target, Rhs::Target>) -> bool {
        Lhs::Target::nearly_gt_tol(self, other, tol)
    }

    #[inline]
    fn nearly_ge_tol(&self, other: &Pin<Rhs>, tol: &Tolerance<Lhs::Target, Rhs::Target>) -> bool {
        Lhs::Target::nearly_ge_tol(self, other, tol)
    }
}

impl<Lhs: Deref, Rhs: Deref> NearlyOrd<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs> where
    Lhs::Target: NearlyOrd<Rhs::Target> + EpsTolerance<Rhs::Target> + UlpsTolerance<Rhs::Target>
{
}
