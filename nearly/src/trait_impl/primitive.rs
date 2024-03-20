use crate::nearly_eq::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use crate::nearly_ord::{NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps};
use crate::tolerance::{EpsToleranceType, UlpsToleranceType};
use crate::ulps::Ulps;

macro_rules! impl_float {
    ($float: ty) => {
        ///////////////
        // nearly_eq //
        ///////////////

        impl NearlyEqEps for $float {
            /// Returns true if `self - other` is in range `[-eps, eps]`.
            fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
                if self == other {
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

                abs <= *eps
            }
        }

        impl NearlyEqUlps for $float {
            /// Returns true if the signed ulps distance between `self` and `other` is in range
            /// `[-ulps, ulps]`.
            ///
            /// This function will only work for inputs with the same sign.
            /// It will always return false if `self` and `other` have different signs.
            /// Therefore, do not use this function for comparison around zero.
            fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<$float>) -> bool {
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
                ulps_distance >= -*ulps && ulps_distance <= *ulps
            }
        }

        impl NearlyEqTol for $float {}
        impl NearlyEq for $float {}

        ////////////////
        // nearly_ord //
        ////////////////

        impl NearlyOrdEps for $float {
            /// Returns true if `self < other` and `self` is not nearly equal to `other` based on
            /// the absolute epsilon value `eps`.
            #[inline]
            fn nearly_lt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
                self < other && self.nearly_ne_eps(other, eps)
            }

            /// Returns true if `self > other` and `self` is not nearly equal to `other` based on
            /// the absolute epsilon value `eps`.
            #[inline]
            fn nearly_gt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
                self > other && self.nearly_ne_eps(other, eps)
            }
        }

        impl NearlyOrdUlps for $float {
            /// Returns true if `self < other` and `self` is not nearly equal to `other` based on
            /// the ulps distance `ulps`.
            #[inline]
            fn nearly_lt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
                self < other && self.nearly_ne_ulps(other, ulps)
            }

            /// Returns true if `self > other` and `self` is not nearly equal to `other` based on
            /// the ulps distance `ulps`.
            #[inline]
            fn nearly_gt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
                self > other && self.nearly_ne_ulps(other, ulps)
            }
        }

        impl NearlyOrdTol for $float {}
        impl NearlyOrd for $float {}
    };
}

impl_float!(f32);
impl_float!(f64);
