use crate::tolerance::{
    EpsAndUlpsTolerance, EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance,
    UlpsToleranceType,
};
use crate::ulps::Ulps;

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

/// A trait for nearly equality comparison based on an ulps value.
pub trait NearlyEqUlps<Rhs = Self, LhsTol = Self, RhsTol = Rhs>
where
    Rhs: ?Sized,
    LhsTol: UlpsTolerance<RhsTol>,
{
    /// Returns whether `self`is nearly equal to `other` based on an ulps value `ulps`.
    fn nearly_eq_ulps(&self, other: &Rhs, ulps: UlpsToleranceType<LhsTol, RhsTol>) -> bool;

    /// Returns whether `self`is not nearly equal to `other` based on an ulps value `ulps`.
    #[inline]
    fn nearly_ne_ulps(&self, other: &Rhs, ulps: UlpsToleranceType<LhsTol, RhsTol>) -> bool {
        !self.nearly_eq_ulps(other, ulps)
    }
}

/// A trait for nearly equality comparison based on a tolerance including an absolute epsilon value
/// and an ulps value. See [Tolerance].
///
/// This trait combines the traits [NearlyEqEps] and [NearlyEqUlps].
pub trait NearlyEqTol<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyEqEps<Rhs, LhsTol, RhsTol> + NearlyEqUlps<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: EpsAndUlpsTolerance<RhsTol>,
{
    /// Returns whether `self` is nearly equal to `other` based on a tolerance `tolerance`.
    /// Returns true if either `self` is nearly equal to `other` based on an epsilon value
    /// `tolerance.eps` or `self`is nearly equal to `other` based on an ulps value `tolerance.ulps`.
    #[inline]
    fn nearly_eq_tol(&self, other: &Rhs, tolerance: Tolerance<LhsTol, RhsTol>) -> bool {
        self.nearly_eq_eps(other, tolerance.eps) || self.nearly_eq_ulps(other, tolerance.ulps)
    }

    /// Returns whether `self` is not nearly equal to `other` based on a tolerance `tolerance`.
    /// Returns true if both `self` is not nearly equal to `other` based on an epsilon value
    /// `tolerance.eps` ans `self`is not nearly equal to `other` based on an ulps value
    /// `tolerance.ulps`.
    #[inline]
    fn nearly_ne_tol(&self, other: &Rhs, tolerance: Tolerance<LhsTol, RhsTol>) -> bool {
        !self.nearly_eq_tol(other, tolerance)
    }
}

/// A trait for nearly equality comparison based on a default tolerance.
/// This trait is a convenience trait to use nearly equality comparison with a default tolerances.
///
/// This is the same as using the [NearlyEqTol] trait with [Tolerance::default()].
pub trait NearlyEq<Rhs = Self, LhsTol = Self, RhsTol = Rhs>:
    NearlyEqTol<Rhs, LhsTol, RhsTol>
where
    Rhs: ?Sized,
    LhsTol: EpsAndUlpsTolerance<RhsTol>,
{
    /// Returns whether `self` is nearly equal to `other` based on the default tolerance for type
    /// `Self`.
    #[inline]
    fn nearly_eq(&self, other: &Rhs) -> bool {
        self.nearly_eq_tol(other, Tolerance::<LhsTol, RhsTol>::default())
    }

    /// Returns whether `self` is not nearly equal to `other` based on the default tolerance for
    /// type `Self`.
    #[inline]
    fn nearly_ne(&self, other: &Rhs) -> bool {
        !self.nearly_eq(other)
    }
}

macro_rules! impl_nearly_float {
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

        impl NearlyEqUlps for $float {
            /// Returns true if the signed ulps distance between `self` and `other` is in range
            /// `[-ulps, ulps]`.
            ///
            /// This function will only work for inputs with the same sign.
            /// It will always return false if `other` and `self` have different signs.
            /// Therefore, do not use this function for comparison around zero.
            fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<$float>) -> bool {
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

        impl NearlyEqTol for $float {}

        impl NearlyEq for $float {}
    };
}

impl_nearly_float!(f32);
impl_nearly_float!(f64);

macro_rules! impl_nearly_collection {
    ([$($vars:tt)*], $lhs: ty, $rhs: ty) => {
        impl<Lhs, Rhs, $($vars)*> NearlyEqEps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
        {
            fn nearly_eq_eps(
                &self,
                other: &$rhs,
                eps: EpsToleranceType<Lhs, Rhs>,
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
                ulps: UlpsToleranceType<Lhs, Rhs>,
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
            Lhs: NearlyEqTol<Rhs> + EpsAndUlpsTolerance<Rhs>,
        {
        }

        impl<Lhs, Rhs, $($vars)*> NearlyEq<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEq<Rhs> + EpsAndUlpsTolerance<Rhs>,
        {
        }
    };
}

impl_nearly_collection!([const N: usize], [Lhs; N], [Rhs; N]);
impl_nearly_collection!([const N: usize], [Lhs; N], [Rhs]);
impl_nearly_collection!([const N: usize], [Lhs; N], &[Rhs]);

impl_nearly_collection!([], [Lhs], [Rhs]);
impl_nearly_collection!([], [Lhs], &[Rhs]);
impl_nearly_collection!([const N: usize], [Lhs], [Rhs; N]);

impl_nearly_collection!([], &[Lhs], &[Rhs]);
impl_nearly_collection!([], &[Lhs], [Rhs]);
impl_nearly_collection!([const N: usize], &[Lhs], [Rhs; N]);

#[cfg(feature = "std")]
mod std_types {
    use super::*;
    use std::collections::{LinkedList, VecDeque};

    impl_nearly_collection!([], Vec<Lhs>, Vec<Rhs>);
    impl_nearly_collection!([], Vec<Lhs>, VecDeque<Rhs>);
    impl_nearly_collection!([const N: usize], Vec<Lhs>, [Rhs; N]);
    impl_nearly_collection!([], Vec<Lhs>, [Rhs]);
    impl_nearly_collection!([], Vec<Lhs>, &[Rhs]);

    impl_nearly_collection!([const N: usize], [Lhs; N], Vec<Rhs>);
    impl_nearly_collection!([], [Lhs], Vec<Rhs>);
    impl_nearly_collection!([], &[Lhs], Vec<Rhs>);

    impl_nearly_collection!([], VecDeque<Lhs>, VecDeque<Rhs>);
    impl_nearly_collection!([], VecDeque<Lhs>, Vec<Rhs>);
    impl_nearly_collection!([const N: usize], VecDeque<Lhs>, [Rhs; N]);
    impl_nearly_collection!([], VecDeque<Lhs>, [Rhs]);
    impl_nearly_collection!([], VecDeque<Lhs>, &[Rhs]);

    impl_nearly_collection!([const N: usize], [Lhs; N], VecDeque<Rhs>);
    impl_nearly_collection!([], [Lhs], VecDeque<Rhs>);
    impl_nearly_collection!([], &[Lhs], VecDeque<Rhs>);

    impl_nearly_collection!([], LinkedList<Lhs>, LinkedList<Rhs>);
}
