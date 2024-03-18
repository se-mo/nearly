use crate::nearly_eq::{NearlyEqEps, NearlyEqUlps};
use crate::tolerance::{
    EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance, UlpsToleranceType,
};

/// A trait for nearly ordering comparison based on an absolute epsilon value.
pub trait NearlyOrdEps<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyEqEps<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + EpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self` is strict less than `other` but not nearly equal to `other`
    /// based on an absolute epsilon value `eps`.
    ///
    /// See [nearly_ne_eps](NearlyEqEps::nearly_ne_eps()).
    fn nearly_lt_eps(&self, other: &Rhs, eps: &EpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self` is strict less than `other` or nearly equal to `other`
    /// based on an absolute epsilon value `eps`.
    ///
    /// See [nearly_eq_eps](NearlyEqEps::nearly_eq_eps()).
    fn nearly_le_eps(&self, other: &Rhs, eps: &EpsToleranceType<LhsTol, RhsTol>) -> bool {
        self.nearly_lt_eps(other, eps) || self.nearly_eq_eps(other, eps)
    }

    /// Returns whether `self` is strict greater than `other` but not nearly equal to `other`
    /// based on an absolute epsilon value `eps`.
    ///
    /// See [nearly_ne_eps](NearlyEqEps::nearly_ne_eps()).
    fn nearly_gt_eps(&self, other: &Rhs, eps: &EpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self` is strict greater than `other` or nearly equal to `other`
    /// based on an absolute epsilon value `eps`.
    ///
    /// See [nearly_eq_eps](NearlyEqEps::nearly_eq_eps()).
    fn nearly_ge_eps(&self, other: &Rhs, eps: &EpsToleranceType<LhsTol, RhsTol>) -> bool {
        self.nearly_gt_eps(other, eps) || self.nearly_eq_eps(other, eps)
    }
}

/// A trait for nearly ordering comparison based on an ulps value.
pub trait NearlyOrdUlps<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyEqUlps<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + UlpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self` is strict less than `other` but not nearly equal to `other`
    /// based on an ulps value `ulps`.
    ///
    /// See [nearly_ne_ulps](NearlyEqUlps::nearly_ne_ulps()).
    fn nearly_lt_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self` is strict less than `other` or nearly equal to `other`
    /// based on an ulps value `ulps`.
    ///
    /// See [nearly_eq_ulps](NearlyEqUlps::nearly_eq_ulps()).
    #[inline]
    fn nearly_le_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<LhsTol, RhsTol>) -> bool {
        self.nearly_lt_ulps(other, ulps) || self.nearly_eq_ulps(other, ulps)
    }

    /// Returns whether `self` is strict greater than `other` but not nearly equal to `other`
    /// based on an ulps value `ulps`.
    ///
    /// See [nearly_ne_ulps](NearlyEqUlps::nearly_ne_ulps()).
    fn nearly_gt_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self` is strict greater than `other` or nearly equal to `other`
    /// based on an ulps value `ulps`.
    ///
    /// See [nearly_eq_ulps](NearlyEqUlps::nearly_eq_ulps()).
    #[inline]
    fn nearly_ge_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<LhsTol, RhsTol>) -> bool {
        self.nearly_gt_ulps(other, ulps) || self.nearly_eq_ulps(other, ulps)
    }
}

/// A trait for nearly ordering comparison based on a tolerance including an absolute epsilon value
/// and an ulps value.
///
/// See [Tolerance].
/// This trait combines the traits [NearlyOrdEps] and [NearlyOrdUlps].
pub trait NearlyOrdTol<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyOrdEps<Rhs, LhsTol, RhsTol> + NearlyOrdUlps<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + EpsTolerance<RhsTol> + UlpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self` is strict less than `other` but not nearly equal to `other`
    /// based on a tolerance `tol`.
    ///
    /// Returns true if `self` is strict less than `other` but not nearly equal to `other`
    /// based on an absolute epsilon value `tol.eps` and not nearly equal to `other` based
    /// on an ulps value `tol.ulps`.
    ///
    /// See [nearly_ne_eps](NearlyEqEps::nearly_ne_eps()) and
    /// [nearly_ne_ulps](NearlyEqUlps::nearly_ne_ulps()).
    #[inline]
    fn nearly_lt_tol(&self, other: &Rhs, tol: &Tolerance<LhsTol, RhsTol>) -> bool {
        self.nearly_lt_eps(other, &tol.eps) || self.nearly_lt_ulps(other, &tol.ulps)
    }

    /// Returns whether `self` is strict less than `other` or nearly equal to `other`
    /// based on a tolerance `tol`.
    ///
    /// Returns true if `self` is strict less than `other` or nearly equal to `other`
    /// based on an absolute epsilon value `tol.eps` or nearly equal to `other` based
    /// on an ulps value `tol.ulps`.
    ///
    /// See [nearly_eq_eps](NearlyEqEps::nearly_eq_eps()) and
    /// [nearly_eq_ulps](NearlyEqUlps::nearly_eq_ulps()).
    #[inline]
    fn nearly_le_tol(&self, other: &Rhs, tol: &Tolerance<LhsTol, RhsTol>) -> bool {
        self.nearly_le_eps(other, &tol.eps) || self.nearly_le_ulps(other, &tol.ulps)
    }

    /// Returns whether `self` is strict greater than `other` but not nearly equal to `other`
    /// based on a tolerance `tol`.
    ///
    /// Returns true if `self` is strict greater than `other` but not nearly equal to `other`
    /// based on an absolute epsilon value `tol.eps` and not nearly equal to `other` based
    /// on an ulps value `tol.ulps`.
    ///
    /// See [nearly_ne_eps](NearlyEqEps::nearly_ne_eps()) and
    /// [nearly_ne_ulps](NearlyEqUlps::nearly_ne_ulps()).
    #[inline]
    fn nearly_gt_tol(&self, other: &Rhs, tol: &Tolerance<LhsTol, RhsTol>) -> bool {
        self.nearly_gt_eps(other, &tol.eps) || self.nearly_gt_ulps(other, &tol.ulps)
    }

    /// Returns whether `self` is strict greater than `other` or nearly equal to `other`
    /// based on a tolerance `tol`.
    ///
    /// Returns true if `self` is strict greater than `other` or nearly equal to `other`
    /// based on an absolute epsilon value `tol.eps` or nearly equal to `other` based
    /// on an ulps value `tol.ulps`.
    ///
    /// See [nearly_eq_eps](NearlyEqEps::nearly_eq_eps()) and
    /// [nearly_eq_ulps](NearlyEqUlps::nearly_eq_ulps()).
    #[inline]
    fn nearly_ge_tol(&self, other: &Rhs, tol: &Tolerance<LhsTol, RhsTol>) -> bool {
        self.nearly_ge_eps(other, &tol.eps) || self.nearly_ge_ulps(other, &tol.ulps)
    }
}

/// A trait for nearly ordering comparison based on a default tolerance.
///
/// This trait is a convenience trait to use nearly ordering comparison with a default tolerance.
/// This is the same as using the [NearlyOrdTol] trait with [Tolerance::default()].
pub trait NearlyOrd<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyOrdTol<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + EpsTolerance<RhsTol> + UlpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self` is strict less than `other` but not nearly equal to `other`
    /// based on the default tolerance for comparisons of `self` with `other`.
    #[inline]
    fn nearly_lt(&self, other: &Rhs) -> bool {
        self.nearly_lt_tol(other, &Tolerance::<LhsTol, RhsTol>::default())
    }

    /// Returns whether `self` is strict less than `other` or nearly equal to `other`
    /// based on the default tolerance for comparisons of `self` with `other`.
    #[inline]
    fn nearly_le(&self, other: &Rhs) -> bool {
        self.nearly_le_tol(other, &Tolerance::<LhsTol, RhsTol>::default())
    }

    /// Returns whether `self` is strict greater than `other` but not nearly equal to `other`
    /// based on the default tolerance for comparisons of `self` with `other`.
    #[inline]
    fn nearly_gt(&self, other: &Rhs) -> bool {
        self.nearly_gt_tol(other, &Tolerance::<LhsTol, RhsTol>::default())
    }

    /// Returns whether `self` is strict greater than `other` or nearly equal to `other`
    /// based on the default tolerance for comparisons of `self` with `other`.
    #[inline]
    fn nearly_ge(&self, other: &Rhs) -> bool {
        self.nearly_ge_tol(other, &Tolerance::<LhsTol, RhsTol>::default())
    }
}

////////////////
// primitives //
////////////////

macro_rules! impl_float {
    ($float: ty) => {
        impl NearlyOrdEps for $float {
            /// Returns true if `self < other` and `self` is not nearly equal to `other` based on
            /// the absolute epsilon value `eps`.
            #[inline]
            fn nearly_lt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
                self < other && self.nearly_ne_eps(other, eps)
            }

            /// Returns true if `self < other` or `self` is nearly equal to `other` based on
            /// the absolute epsilon value `eps`.
            #[inline]
            fn nearly_le_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
                self < other || self.nearly_eq_eps(other, eps)
            }

            /// Returns true if `self > other` and `self` is not nearly equal to `other` based on
            /// the absolute epsilon value `eps`.
            #[inline]
            fn nearly_gt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
                self > other && self.nearly_ne_eps(other, eps)
            }

            /// Returns true if `self > other` or `self` is nearly equal to `other` based on
            /// the absolute epsilon value `eps`.
            #[inline]
            fn nearly_ge_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
                self > other || self.nearly_eq_eps(other, eps)
            }
        }

        impl NearlyOrdUlps for $float {
            /// Returns true if `self < other` and `self` is not nearly equal to `other` based on
            /// the ulps distance `ulps`.
            #[inline]
            fn nearly_lt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
                self < other && self.nearly_ne_ulps(other, ulps)
            }

            /// Returns true if `self < other` or `self` is nearly equal to `other` based on
            /// the ulps distance `ulps`.
            #[inline]
            fn nearly_le_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
                self < other || self.nearly_eq_ulps(other, ulps)
            }

            /// Returns true if `self > other` and `self` is not nearly equal to `other` based on
            /// the ulps distance `ulps`.
            #[inline]
            fn nearly_gt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
                self > other && self.nearly_ne_ulps(other, ulps)
            }

            /// Returns true if `self > other` or `self` is nearly equal to `other` based on
            /// the ulps distance `ulps`.
            #[inline]
            fn nearly_ge_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
                self > other || self.nearly_eq_ulps(other, ulps)
            }
        }

        impl NearlyOrdTol for $float {}
        impl NearlyOrd for $float {}
    };
}

impl_float!(f32);
impl_float!(f64);

////////////////
// references //
////////////////

macro_rules! impl_ref {
    ($lhs: ty, $rhs: ty) => {
        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyOrdEps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_lt_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_lt_eps(*self, *other, eps)
            }

            #[inline]
            fn nearly_le_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_le_eps(*self, *other, eps)
            }

            #[inline]
            fn nearly_gt_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_gt_eps(*self, *other, eps)
            }

            #[inline]
            fn nearly_ge_eps(&self, other: &$rhs, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdEps::nearly_ge_eps(*self, *other, eps)
            }
        }

        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyOrdUlps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_lt_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_lt_ulps(*self, *other, ulps)
            }

            #[inline]
            fn nearly_le_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_le_ulps(*self, *other, ulps)
            }

            #[inline]
            fn nearly_gt_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_gt_ulps(*self, *other, ulps)
            }

            #[inline]
            fn nearly_ge_ulps(&self, other: &$rhs, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyOrdUlps::nearly_ge_ulps(*self, *other, ulps)
            }
        }

        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyOrdTol<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        {
            #[inline]
            fn nearly_lt_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_lt_tol(*self, *other, tol)
            }

            #[inline]
            fn nearly_le_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_le_tol(*self, *other, tol)
            }

            #[inline]
            fn nearly_gt_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_gt_tol(*self, *other, tol)
            }

            #[inline]
            fn nearly_ge_tol(&self, other: &$rhs, tol: &Tolerance<Lhs, Rhs>) -> bool {
                NearlyOrdTol::nearly_ge_tol(*self, *other, tol)
            }
        }

        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyOrd<$rhs, Lhs, Rhs> for $lhs where
            Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>
        {
        }
    };
}

impl_ref!(&Lhs, &Rhs);
impl_ref!(&Lhs, &mut Rhs);
impl_ref!(&mut Lhs, &Rhs);
impl_ref!(&mut Lhs, &mut Rhs);

/////////////////
// collections //
/////////////////

macro_rules! impl_collection {
    ([$($vars:tt)*], $lhs: ty, $rhs: ty) => {
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

//////////
// maps //
//////////

#[cfg(feature = "std")]
mod map {
    use super::*;
    use std::collections::{BTreeMap, HashMap};
    use std::hash::{BuildHasher, Hash};

    impl<K, Lhs, Rhs, S> NearlyOrdEps<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
    where
        K: Eq + Hash,
        Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
        S: BuildHasher,
    {
        fn nearly_lt_eps(
            &self,
            other: &HashMap<K, Rhs, S>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdEps::nearly_lt_eps(v_lhs, v_rhs, eps)
                    })
                })
        }

        fn nearly_le_eps(
            &self,
            other: &HashMap<K, Rhs, S>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdEps::nearly_le_eps(v_lhs, v_rhs, eps)
                    })
                })
        }

        fn nearly_gt_eps(
            &self,
            other: &HashMap<K, Rhs, S>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdEps::nearly_gt_eps(v_lhs, v_rhs, eps)
                    })
                })
        }

        fn nearly_ge_eps(
            &self,
            other: &HashMap<K, Rhs, S>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdEps::nearly_ge_eps(v_lhs, v_rhs, eps)
                    })
                })
        }
    }

    impl<K, Lhs, Rhs, S> NearlyOrdUlps<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
    where
        K: Eq + Hash,
        Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
        S: BuildHasher,
    {
        fn nearly_lt_ulps(
            &self,
            other: &HashMap<K, Rhs, S>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdUlps::nearly_lt_ulps(v_lhs, v_rhs, ulps)
                    })
                })
        }

        fn nearly_le_ulps(
            &self,
            other: &HashMap<K, Rhs, S>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdUlps::nearly_le_ulps(v_lhs, v_rhs, ulps)
                    })
                })
        }

        fn nearly_gt_ulps(
            &self,
            other: &HashMap<K, Rhs, S>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdUlps::nearly_gt_ulps(v_lhs, v_rhs, ulps)
                    })
                })
        }

        fn nearly_ge_ulps(
            &self,
            other: &HashMap<K, Rhs, S>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdUlps::nearly_ge_ulps(v_lhs, v_rhs, ulps)
                    })
                })
        }
    }

    impl<K, Lhs, Rhs, S> NearlyOrdTol<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
    where
        K: Eq + Hash,
        Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        S: BuildHasher,
    {
        fn nearly_lt_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdTol::nearly_lt_tol(v_lhs, v_rhs, tol)
                    })
                })
        }

        fn nearly_le_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdTol::nearly_le_tol(v_lhs, v_rhs, tol)
                    })
                })
        }

        fn nearly_gt_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdTol::nearly_gt_tol(v_lhs, v_rhs, tol)
                    })
                })
        }

        fn nearly_ge_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self.iter().all(|(key, v_lhs)| {
                    other.get(key).map_or(false, |v_rhs| {
                        NearlyOrdTol::nearly_ge_tol(v_lhs, v_rhs, tol)
                    })
                })
        }
    }

    impl<K, Lhs, Rhs, S> NearlyOrd<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
    where
        K: Eq + Hash,
        Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
        S: BuildHasher,
    {
    }

    impl<K, Lhs, Rhs> NearlyOrdEps<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
    where
        K: PartialEq,
        Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
    {
        fn nearly_lt_eps(
            &self,
            other: &BTreeMap<K, Rhs>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_lt_eps(a.1, b.1, eps))
        }

        fn nearly_le_eps(
            &self,
            other: &BTreeMap<K, Rhs>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_le_eps(a.1, b.1, eps))
        }

        fn nearly_gt_eps(
            &self,
            other: &BTreeMap<K, Rhs>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_gt_eps(a.1, b.1, eps))
        }

        fn nearly_ge_eps(
            &self,
            other: &BTreeMap<K, Rhs>,
            eps: &EpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_ge_eps(a.1, b.1, eps))
        }
    }

    impl<K, Lhs, Rhs> NearlyOrdUlps<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
    where
        K: PartialEq,
        Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
    {
        fn nearly_lt_ulps(
            &self,
            other: &BTreeMap<K, Rhs>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_lt_ulps(a.1, b.1, ulps))
        }

        fn nearly_le_ulps(
            &self,
            other: &BTreeMap<K, Rhs>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_le_ulps(a.1, b.1, ulps))
        }

        fn nearly_gt_ulps(
            &self,
            other: &BTreeMap<K, Rhs>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_gt_ulps(a.1, b.1, ulps))
        }

        fn nearly_ge_ulps(
            &self,
            other: &BTreeMap<K, Rhs>,
            ulps: &UlpsToleranceType<Lhs, Rhs>,
        ) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_ge_ulps(a.1, b.1, ulps))
        }
    }

    impl<K, Lhs, Rhs> NearlyOrdTol<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
    where
        K: PartialEq,
        Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    {
        fn nearly_lt_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_lt_tol(a.1, b.1, tol))
        }

        fn nearly_le_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_le_tol(a.1, b.1, tol))
        }

        fn nearly_gt_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_gt_tol(a.1, b.1, tol))
        }

        fn nearly_ge_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
            self.len() == other.len()
                && self
                    .iter()
                    .zip(other)
                    .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_ge_tol(a.1, b.1, tol))
        }
    }

    impl<K, Lhs, Rhs> NearlyOrd<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
    where
        K: PartialEq,
        Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    {
    }
}

/////////////
// pointer //
/////////////

#[cfg(feature = "std")]
mod pointer {
    use super::*;
    use std::boxed::Box;
    use std::rc::Rc;
    use std::sync::Arc;

    macro_rules! impl_nearly_pointer {
        ($lhs: ty, $rhs: ty) => {
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

    impl_nearly_pointer!(Arc<Lhs>, Arc<Rhs>);
    impl_nearly_pointer!(Box<Lhs>, Box<Rhs>);
    impl_nearly_pointer!(Rc<Lhs>, Rc<Rhs>);

    use std::ops::Deref;
    use std::pin::Pin;

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
        Lhs::Target:
            NearlyOrdTol<Rhs::Target> + EpsTolerance<Rhs::Target> + UlpsTolerance<Rhs::Target>,
    {
        #[inline]
        fn nearly_lt_tol(
            &self,
            other: &Pin<Rhs>,
            tol: &Tolerance<Lhs::Target, Rhs::Target>,
        ) -> bool {
            Lhs::Target::nearly_lt_tol(self, other, tol)
        }

        #[inline]
        fn nearly_le_tol(
            &self,
            other: &Pin<Rhs>,
            tol: &Tolerance<Lhs::Target, Rhs::Target>,
        ) -> bool {
            Lhs::Target::nearly_le_tol(self, other, tol)
        }

        #[inline]
        fn nearly_gt_tol(
            &self,
            other: &Pin<Rhs>,
            tol: &Tolerance<Lhs::Target, Rhs::Target>,
        ) -> bool {
            Lhs::Target::nearly_gt_tol(self, other, tol)
        }

        #[inline]
        fn nearly_ge_tol(
            &self,
            other: &Pin<Rhs>,
            tol: &Tolerance<Lhs::Target, Rhs::Target>,
        ) -> bool {
            Lhs::Target::nearly_ge_tol(self, other, tol)
        }
    }

    impl<Lhs: Deref, Rhs: Deref> NearlyOrd<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs> where
        Lhs::Target:
            NearlyOrd<Rhs::Target> + EpsTolerance<Rhs::Target> + UlpsTolerance<Rhs::Target>
    {
    }
}

///////////
// tuple //
///////////

macro_rules! impl_nearly_tuple {
    ($lhs:ident, $rhs:ident, $idx: tt) => {
        impl_nearly_tuple!(@impl $lhs, $rhs, $idx);
    };
    ($lhs:ident $( $lhs_tail:ident )+, $rhs:ident $( $rhs_tail:ident )+, $idx:tt $( $idx_tail:tt )+) => {
        impl_nearly_tuple!($( $lhs_tail )+, $( $rhs_tail )+, $( $idx_tail )+);
        impl_nearly_tuple!(@impl $lhs $( $lhs_tail )+, $rhs $( $rhs_tail )+, $idx $( $idx_tail )+);
    };
    (@impl $( $lhs: ident )+, $( $rhs: ident )+, $( $idx: tt )+) => {
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

impl_nearly_tuple!(
    Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs Lhs,
    Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs Rhs,
    11 10 9 8 7 6 5 4 3 2 1 0);
