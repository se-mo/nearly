use crate::tolerance::{EpsTolerance, EpsToleranceType};

/// A trait for nearly equality comparison based on an absolute epsilon value.
pub trait NearlyEqEps<Rhs = Self, LhsTol = Self, RhsTol = Rhs>
where
    Rhs: ?Sized,
    LhsTol: EpsTolerance<RhsTol>,
{
    /// Returns whether `self` is nearly equal to `other` based on an absolute epsilon value `eps`.
    fn nearly_eq_eps(&self, other: &Rhs, eps: EpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self` is nearly not equal to `other` based on an absolute epsilon value `eps`.
    #[inline]
    fn nearly_ne_eps(&self, other: &Rhs, eps: EpsToleranceType<LhsTol, RhsTol>) -> bool {
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

                let diff = self - other;
                #[cfg(not(feature = "std"))]
                // use custom abs in no_std by setting sign bit to 0
                let abs = <$float>::from_bits(
                    diff.to_bits() & !(1 << ((core::mem::size_of::<$float>() * 8) - 1)),
                );
                #[cfg(feature = "std")]
                let abs = diff.abs();

                abs <= eps
            }
        }
    };
}

impl_nearly_eps!(f32);
impl_nearly_eps!(f64);

impl<Lhs, Rhs, const N: usize> NearlyEqEps<[Rhs; N], Lhs, Rhs> for [Lhs; N]
where
    Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
{
    fn nearly_eq_eps(&self, other: &[Rhs; N], eps: EpsToleranceType<Lhs, Rhs>) -> bool {
        for i in 0..N {
            if self[i].nearly_ne_eps(&other[i], eps) {
                return false;
            }
        }
        true
    }
}
