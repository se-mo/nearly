use crate::tolerance::{EpsTolerance, EpsToleranceType};

/// A trait for nearly equality comparison based on an absolute epsilon value.
pub trait NearlyEqEps<Rhs = Self>: EpsTolerance<Rhs>
where
    Rhs: ?Sized,
{
    /// Returns whether `self` is nearly equal to `other` based on an absolute epsilon value `eps`.
    fn nearly_eq_eps(&self, other: &Rhs, eps: EpsToleranceType<Self, Rhs>) -> bool;

    /// Returns whether `self` is nearly not equal to `other` based on an absolute epsilon value `eps`.
    #[inline]
    fn nearly_ne_eps(&self, other: &Rhs, eps: EpsToleranceType<Self, Rhs>) -> bool {
        !self.nearly_eq_eps(other, eps)
    }
}

macro_rules! impl_nearly_eps {
    ($float: ty) => {
        impl NearlyEqEps for $float {
            /// Returns true if `self - other` is in range `[-eps, eps]`.
            fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool {
                if self.is_infinite()
                    && other.is_infinite()
                    && self.is_sign_positive() == other.is_sign_positive()
                {
                    return true;
                }

                (self - other).abs() <= eps
            }
        }
    };
}

impl_nearly_eps!(f32);
impl_nearly_eps!(f64);
