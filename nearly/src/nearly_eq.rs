use crate::tolerance::{
    EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance, UlpsToleranceType,
};

/// A trait for nearly equality comparison based on an absolute epsilon value.
pub trait NearlyEqEps<Rhs = Self, LhsTol = Self, RhsTol = Rhs>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + EpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self` is nearly equal to `other` based on an absolute epsilon value `eps`.
    fn nearly_eq_eps(&self, other: &Rhs, eps: &EpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self` is nearly not equal to `other` based on an absolute epsilon value `eps`.
    #[inline]
    fn nearly_ne_eps(&self, other: &Rhs, eps: &EpsToleranceType<LhsTol, RhsTol>) -> bool {
        !self.nearly_eq_eps(other, eps)
    }
}

/// A trait for nearly equality comparison based on an ulps value.
pub trait NearlyEqUlps<Rhs = Self, LhsTol = Self, RhsTol = Rhs>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + UlpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self`is nearly equal to `other` based on an ulps value `ulps`.
    fn nearly_eq_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self`is not nearly equal to `other` based on an ulps value `ulps`.
    #[inline]
    fn nearly_ne_ulps(&self, other: &Rhs, ulps: &UlpsToleranceType<LhsTol, RhsTol>) -> bool {
        !self.nearly_eq_ulps(other, ulps)
    }
}

/// A trait for nearly equality comparison based on a tolerance including an absolute epsilon value
/// and an ulps value.
///
/// See [Tolerance].
/// This trait combines the traits [NearlyEqEps] and [NearlyEqUlps].
pub trait NearlyEqTol<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyEqEps<Rhs, LhsTol, RhsTol> + NearlyEqUlps<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + EpsTolerance<RhsTol> + UlpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self` is nearly equal to `other` based on a tolerance `tol`.
    ///
    /// Returns true if either `self` is nearly equal to `other` based on an absolute epsilon value
    /// `tol.eps` or `self` is nearly equal to `other` based on an ulps value `tol.ulps`.
    #[inline]
    fn nearly_eq_tol(&self, other: &Rhs, tol: &Tolerance<LhsTol, RhsTol>) -> bool {
        self.nearly_eq_eps(other, &tol.eps) || self.nearly_eq_ulps(other, &tol.ulps)
    }

    /// Returns whether `self` is not nearly equal to `other` based on a tolerance `tol`.
    ///
    /// Returns true if both `self` is not nearly equal to `other` based on an absolute epsilon value
    /// `tol.eps` ans `self`is not nearly equal to `other` based on an ulps value
    /// `tol.ulps`.
    #[inline]
    fn nearly_ne_tol(&self, other: &Rhs, tol: &Tolerance<LhsTol, RhsTol>) -> bool {
        !self.nearly_eq_tol(other, tol)
    }
}

/// A trait for nearly equality comparison based on a default tolerance.
///
/// This trait is a convenience trait to use nearly equality comparison with a default tolerances.
/// This is the same as using the [NearlyEqTol] trait with [Tolerance::default()].
pub trait NearlyEq<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyEqTol<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: ?Sized + EpsTolerance<RhsTol> + UlpsTolerance<RhsTol>,
    RhsTol: ?Sized,
{
    /// Returns whether `self` is nearly equal to `other` based on the default tolerance for
    /// comparisons of `Self` with `other`.
    #[inline]
    fn nearly_eq(&self, other: &Rhs) -> bool {
        self.nearly_eq_tol(other, &Tolerance::<LhsTol, RhsTol>::default())
    }

    /// Returns whether `self` is not nearly equal to `other` based on the default tolerance for
    /// comparisons of `Self` with `other`.
    #[inline]
    fn nearly_ne(&self, other: &Rhs) -> bool {
        !self.nearly_eq(other)
    }
}
