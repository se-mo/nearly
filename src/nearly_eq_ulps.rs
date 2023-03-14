use crate::tolerance::{UlpsTolerance, UlpsToleranceType};
use crate::ulps::Ulps;

/// A trait for nearly equality comparison based on an ulps value.
pub trait NearlyEqUlps<Rhs = Self>: UlpsTolerance<Rhs>
where
    Rhs: ?Sized,
{
    /// Returns whether `self`is nearly equal to `other` based on an ulps value `ulps`.
    fn nearly_eq_ulps(&self, other: &Rhs, ulps: UlpsToleranceType<Self, Rhs>) -> bool;

    /// Returns whether `self`is not nearly equal to `other` based on an ulps value `ulps`.
    #[inline]
    fn nearly_ne_ulps(&self, other: &Rhs, ulps: UlpsToleranceType<Self, Rhs>) -> bool {
        !self.nearly_eq_ulps(other, ulps)
    }
}

macro_rules! impl_nearly_ulps {
    ($float: ty) => {
        impl NearlyEqUlps for $float {
            /// Returns true if the signed ulps distance between `self` and `other` is in range
            /// `[-ulps, ulps]`.
            ///
            /// This function will only work for inputs with the same sign.
            /// It will always return false if `other` and `self` have different signs.
            /// Therefore, do not use this function for comparison around zero.
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool {
                // handles +0 == -0
                if self == other {
                    return true;
                }

                if self.is_sign_positive() != other.is_sign_positive()
                    || self.is_nan()
                    || other.is_nan()
                {
                    return false;
                }

                let ulps_distance = self.signed_ulps_distance(*other);
                ulps_distance >= -ulps && ulps_distance <= ulps
            }
        }
    };
}

impl_nearly_ulps!(f32);
impl_nearly_ulps!(f64);
