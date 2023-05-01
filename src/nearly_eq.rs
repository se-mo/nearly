use crate::tolerance::{
    EpsAndUlpsTolerance, EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance,
    UlpsToleranceType,
};
use crate::ulps::Ulps;

/// A trait for nearly equality comparison based on an absolute epsilon value.
pub trait NearlyEqEps<Rhs = Self, LhsTol = Self, RhsTol = Rhs>
where
    Rhs: ?Sized,
    LhsTol: EpsTolerance<RhsTol> + ?Sized,
    RhsTol: ?Sized,
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
    LhsTol: UlpsTolerance<RhsTol> + ?Sized,
    RhsTol: ?Sized,
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
    LhsTol: EpsAndUlpsTolerance<RhsTol> + ?Sized,
    RhsTol: ?Sized,
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
    LhsTol: EpsAndUlpsTolerance<RhsTol> + ?Sized,
    RhsTol: ?Sized,
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

macro_rules! impl_nearly_ref {
    ($lhs: ty, $rhs: ty) => {
        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyEqEps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
        {
            fn nearly_eq_eps(&self, other: &$rhs, eps: EpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyEqEps::nearly_eq_eps(*self, *other, eps)
            }
        }

        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyEqUlps<$rhs, Lhs, Rhs> for $lhs
        where
            Lhs: NearlyEqUlps<Rhs> + UlpsTolerance<Rhs>,
        {
            fn nearly_eq_ulps(&self, other: &$rhs, ulps: UlpsToleranceType<Lhs, Rhs>) -> bool {
                NearlyEqUlps::nearly_eq_ulps(*self, *other, ulps)
            }
        }

        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyEqTol<$rhs, Lhs, Rhs> for $lhs where
            Lhs: NearlyEqTol<Rhs> + EpsAndUlpsTolerance<Rhs>
        {
        }

        impl<Lhs: ?Sized, Rhs: ?Sized> NearlyEq<$rhs, Lhs, Rhs> for $lhs where
            Lhs: NearlyEq<Rhs> + EpsAndUlpsTolerance<Rhs>
        {
        }
    };
}

impl_nearly_ref!(&Lhs, &Rhs);
impl_nearly_ref!(&Lhs, &mut Rhs);
impl_nearly_ref!(&mut Lhs, &Rhs);
impl_nearly_ref!(&mut Lhs, &mut Rhs);

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
mod std_collection {
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

#[cfg(feature = "std")]
mod pointer {
    use super::*;
    use std::boxed::Box;
    use std::rc::Rc;
    use std::sync::Arc;

    macro_rules! impl_nearly_pointer {
        ($lhs: ty, $rhs: ty) => {
            impl<Lhs, Rhs> NearlyEqEps<$rhs, Lhs, Rhs> for $lhs
            where
                Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
            {
                #[inline]
                fn nearly_eq_eps(&self, other: &$rhs, eps: EpsToleranceType<Lhs, Rhs>) -> bool {
                    NearlyEqEps::nearly_eq_eps(&**self, &**other, eps)
                }
            }

            impl<Lhs, Rhs> NearlyEqUlps<$rhs, Lhs, Rhs> for $lhs
            where
                Lhs: NearlyEqUlps<Rhs> + UlpsTolerance<Rhs>,
            {
                #[inline]
                fn nearly_eq_ulps(&self, other: &$rhs, ulps: UlpsToleranceType<Lhs, Rhs>) -> bool {
                    NearlyEqUlps::nearly_eq_ulps(&**self, &**other, ulps)
                }
            }

            impl<Lhs, Rhs> NearlyEqTol<$rhs, Lhs, Rhs> for $lhs where
                Lhs: NearlyEqTol<Rhs> + EpsAndUlpsTolerance<Rhs>
            {
            }

            impl<Lhs, Rhs> NearlyEq<$rhs, Lhs, Rhs> for $lhs where
                Lhs: NearlyEq<Rhs> + EpsAndUlpsTolerance<Rhs>
            {
            }
        };
    }

    impl_nearly_pointer!(Arc<Lhs>, Arc<Rhs>);
    impl_nearly_pointer!(Box<Lhs>, Box<Rhs>);
    impl_nearly_pointer!(Rc<Lhs>, Rc<Rhs>);

    use std::ops::Deref;
    use std::pin::Pin;

    impl<Lhs: Deref, Rhs: Deref> NearlyEqEps<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
    where
        Lhs::Target: NearlyEqEps<Rhs::Target> + EpsTolerance<Rhs::Target>,
    {
        #[inline]
        fn nearly_eq_eps(
            &self,
            other: &Pin<Rhs>,
            eps: EpsToleranceType<Lhs::Target, Rhs::Target>,
        ) -> bool {
            Lhs::Target::nearly_eq_eps(self, other, eps)
        }
    }

    impl<Lhs: Deref, Rhs: Deref> NearlyEqUlps<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs>
    where
        Lhs::Target: NearlyEqUlps<Rhs::Target> + UlpsTolerance<Rhs::Target>,
    {
        #[inline]
        fn nearly_eq_ulps(
            &self,
            other: &Pin<Rhs>,
            ulps: UlpsToleranceType<Lhs::Target, Rhs::Target>,
        ) -> bool {
            Lhs::Target::nearly_eq_ulps(self, other, ulps)
        }
    }

    impl<Lhs: Deref, Rhs: Deref> NearlyEqTol<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs> where
        Lhs::Target: NearlyEqTol<Rhs::Target> + EpsAndUlpsTolerance<Rhs::Target>
    {
    }

    impl<Lhs: Deref, Rhs: Deref> NearlyEq<Pin<Rhs>, Lhs::Target, Rhs::Target> for Pin<Lhs> where
        Lhs::Target: NearlyEq<Rhs::Target> + EpsAndUlpsTolerance<Rhs::Target>
    {
    }
}
