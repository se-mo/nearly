use crate::nearly_eq_eps::NearlyEqEps;
use crate::nearly_eq_ulps::NearlyEqUlps;
use crate::tolerance::Tolerance;

/// A trait for nearly equality comparison based on a tolerance including an absolute epsilon value
/// and an ulps value. See [Tolerance].
///
/// This trait combines the traits [NearlyEqEps] and [NearlyEqUlps].
pub trait NearlyEqTol<Rhs = Self>: NearlyEqEps<Rhs> + NearlyEqUlps<Rhs>
where
    Rhs: ?Sized,
{
    /// Returns whether `self` is nearly equal to `other` based on a tolerance `tolerance`.
    /// Returns true if either `self` is nearly equal to `other` based on an epsilon value
    /// `tolerance.eps` or `self`is nearly equal to `other` based on an ulps value `tolerance.ulps`.
    #[inline]
    fn nearly_eq_tol(&self, other: &Rhs, tolerance: Tolerance<Self, Rhs>) -> bool {
        self.nearly_eq_eps(other, tolerance.eps) || self.nearly_eq_ulps(other, tolerance.ulps)
    }

    /// Returns whether `self` is not nearly equal to `other` based on a tolerance `tolerance`.
    /// Returns true if both `self` is not nearly equal to `other` based on an epsilon value
    /// `tolerance.eps` ans `self`is not nearly equal to `other` based on an ulps value
    /// `tolerance.ulps`.
    #[inline]
    fn nearly_ne_tol(&self, other: &Rhs, tolerance: Tolerance<Self, Rhs>) -> bool {
        !self.nearly_eq_tol(other, tolerance)
    }
}

impl NearlyEqTol for f32 {}
impl NearlyEqTol for f64 {}
