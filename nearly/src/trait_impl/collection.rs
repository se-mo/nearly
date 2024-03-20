use crate::nearly_eq::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use crate::nearly_ord::{NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps};
use crate::tolerance::{
    EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance, UlpsToleranceType,
};

macro_rules! impl_collection {
    ([$($vars:tt)*], $lhs: ty, $rhs: ty) => {
        ///////////////
        // nearly_eq //
        ///////////////

        impl<Lhs, Rhs, $($vars)*> NearlyEqEps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
        {
            fn nearly_eq_eps(
                &self,
                other: &$rhs,
                eps: &EpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyEqEps::nearly_eq_eps(a, b, eps))
            }
        }

        impl<Lhs, Rhs, $($vars)*> NearlyEqUlps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_eq_ulps(
                &self,
                other: &$rhs,
                ulps: &UlpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyEqUlps::nearly_eq_ulps(a, b, ulps))
            }
        }

        impl<Lhs, Rhs, $($vars)*> NearlyEqTol<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_eq_tol(
                &self,
                other: &$rhs,
                tol: &Tolerance<Lhs, Rhs>
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyEqTol::nearly_eq_tol(a, b, tol))
            }
        }

        impl<Lhs, Rhs, $($vars)*> NearlyEq<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEq<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
        }

        ////////////////
        // nearly_ord //
        ////////////////

        impl<Lhs, Rhs, $($vars)*> NearlyOrdEps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
        {
            fn nearly_lt_eps(
                &self,
                other: &$rhs,
                eps: &EpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdEps::nearly_lt_eps(a, b, eps))
            }

            fn nearly_le_eps(
                &self,
                other: &$rhs,
                eps: &EpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdEps::nearly_le_eps(a, b, eps))
            }

            fn nearly_gt_eps(
                &self,
                other: &$rhs,
                eps: &EpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdEps::nearly_gt_eps(a, b, eps))
            }

            fn nearly_ge_eps(
                &self,
                other: &$rhs,
                eps: &EpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdEps::nearly_ge_eps(a, b, eps))
            }
        }

        impl<Lhs, Rhs, $($vars)*> NearlyOrdUlps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_lt_ulps(
                &self,
                other: &$rhs,
                ulps: &UlpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdUlps::nearly_lt_ulps(a, b, ulps))
            }

            fn nearly_le_ulps(
                &self,
                other: &$rhs,
                ulps: &UlpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdUlps::nearly_le_ulps(a, b, ulps))
            }

            fn nearly_gt_ulps(
                &self,
                other: &$rhs,
                ulps: &UlpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdUlps::nearly_gt_ulps(a, b, ulps))
            }

            fn nearly_ge_ulps(
                &self,
                other: &$rhs,
                ulps: &UlpsToleranceType<Lhs, Rhs>,
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdUlps::nearly_ge_ulps(a, b, ulps))
            }
        }

        impl<Lhs, Rhs, $($vars)*> NearlyOrdTol<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_lt_tol(
                &self,
                other: &$rhs,
                tol: &Tolerance<Lhs, Rhs>
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdTol::nearly_lt_tol(a, b, tol))
            }

            fn nearly_le_tol(
                &self,
                other: &$rhs,
                tol: &Tolerance<Lhs, Rhs>
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdTol::nearly_le_tol(a, b, tol))
            }

            fn nearly_gt_tol(
                &self,
                other: &$rhs,
                tol: &Tolerance<Lhs, Rhs>
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdTol::nearly_gt_tol(a, b, tol))
            }

            fn nearly_ge_tol(
                &self,
                other: &$rhs,
                tol: &Tolerance<Lhs, Rhs>
            ) -> bool {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .all(|(a, b)| NearlyOrdTol::nearly_ge_tol(a, b, tol))
            }
        }

        impl<Lhs, Rhs, $($vars)*> NearlyOrd<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
        }
    };
}

impl_collection!([const N: usize], [Lhs; N], [Rhs; N]);
impl_collection!([const N: usize], [Lhs; N], [Rhs]);
impl_collection!([const N: usize], [Lhs; N], &[Rhs]);

impl_collection!([], [Lhs], [Rhs]);
impl_collection!([], [Lhs], &[Rhs]);
impl_collection!([const N: usize], [Lhs], [Rhs; N]);

impl_collection!([], &[Lhs], &[Rhs]);
impl_collection!([], &[Lhs], [Rhs]);
impl_collection!([const N: usize], &[Lhs], [Rhs; N]);

#[cfg(feature = "std")]
mod std_collection {
    use super::*;
    use std::collections::{LinkedList, VecDeque};

    impl_collection!([], Vec<Lhs>, Vec<Rhs>);
    impl_collection!([], Vec<Lhs>, VecDeque<Rhs>);
    impl_collection!([const N: usize], Vec<Lhs>, [Rhs; N]);
    impl_collection!([], Vec<Lhs>, [Rhs]);
    impl_collection!([], Vec<Lhs>, &[Rhs]);

    impl_collection!([const N: usize], [Lhs; N], Vec<Rhs>);
    impl_collection!([], [Lhs], Vec<Rhs>);
    impl_collection!([], &[Lhs], Vec<Rhs>);

    impl_collection!([], VecDeque<Lhs>, VecDeque<Rhs>);
    impl_collection!([], VecDeque<Lhs>, Vec<Rhs>);
    impl_collection!([const N: usize], VecDeque<Lhs>, [Rhs; N]);
    impl_collection!([], VecDeque<Lhs>, [Rhs]);
    impl_collection!([], VecDeque<Lhs>, &[Rhs]);

    impl_collection!([const N: usize], [Lhs; N], VecDeque<Rhs>);
    impl_collection!([], [Lhs], VecDeque<Rhs>);
    impl_collection!([], &[Lhs], VecDeque<Rhs>);

    impl_collection!([], LinkedList<Lhs>, LinkedList<Rhs>);
}
