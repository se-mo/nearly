use crate::nearly_eq_tol::NearlyEqTol;
use crate::tolerance::Tolerance;

/// A trait for nearly equality comparison based on a default tolerance.
/// This trait is a convenience trait to use nearly equality comparison with a default tolerances.
///
/// This is the same as using the [NearlyEqTol] trait with [Tolerance::default()].
pub trait NearlyEq<Rhs = Self>: NearlyEqTol<Rhs>
where
    Rhs: ?Sized,
{
    /// Returns whether `self` is nearly equal to `other` based on the default tolerance for type
    /// `Self`.
    #[inline]
    fn nearly_eq(&self, other: &Rhs) -> bool {
        self.nearly_eq_tol(other, Tolerance::<Self, Rhs>::default())
    }

    /// Returns whether `self` is not nearly equal to `other` based on the default tolerance for
    /// type `Self`.
    #[inline]
    fn nearly_ne(&self, other: &Rhs) -> bool {
        !self.nearly_eq(other)
    }
}

impl NearlyEq for f32 {}
impl NearlyEq for f64 {}
