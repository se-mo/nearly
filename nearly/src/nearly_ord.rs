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

