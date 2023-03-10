#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

/// A trait specifying the tolerance used for absolute epsilon based comparisons for the type
/// implementing this trait comparing to `Rhs`.
pub trait EpsTolerance<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The type of the epsilon values for the type implementing this trait.
    type T: Debug + Copy;
    /// The default epsilon value for the type implementing this trait.
    /// Used for default [Tolerance](Tolerance::default()).
    const DEFAULT: Self::T;
}

/// A type definition for convenience.
///
/// It specifies the type of the epsilon values for the type `Lhs` comparing to the type `Rhs`.
pub type EpsToleranceType<Lhs, Rhs = Lhs> = <Lhs as EpsTolerance<Rhs>>::T;

/// A trait specifying the tolerance used for ulps based comparisons for the type
/// implementing this trait comparing to `Rhs`.
pub trait UlpsTolerance<Rhs = Self>
where
    Rhs: ?Sized,
{
    type T: Debug + Copy;
    const DEFAULT: Self::T;
}

/// A type definition for convenience.
///
/// It specifies the type of the ulps values for the type `Lhs` comparing to the type `Rhs`.
pub type UlpsToleranceType<Lhs, Rhs = Lhs> = <Lhs as UlpsTolerance<Rhs>>::T;

/// A trait combining the [EpsTolerance] and [UlpsTolerance] traits. It can be used if both an
/// epsilon tolerance and an ulps tolerance is required.
///
/// Besides combining the two traits, this trait does not provide any new functionality.
pub trait ToleranceTypes<Rhs = Self>: EpsTolerance<Rhs> + UlpsTolerance<Rhs>
where
    Rhs: ?Sized,
{
}
impl<T: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>, Rhs: ?Sized> ToleranceTypes<Rhs> for T {}

impl EpsTolerance for f32 {
    type T = f32;
    const DEFAULT: f32 = 1e-6;
}

impl UlpsTolerance for f32 {
    type T = i32;
    const DEFAULT: i32 = 4;
}

impl EpsTolerance for f64 {
    type T = f64;
    const DEFAULT: f64 = 1e-14;
}

impl UlpsTolerance for f64 {
    type T = i64;
    const DEFAULT: i64 = 4;
}

/// Tolerance type used when comparing values of type [f32].
pub type ToleranceF32 = Tolerance<f32>;

/// Tolerance type used when comparing values of type [f64].
pub type ToleranceF64 = Tolerance<f64>;

/// A tolerance data type that is used for nearly comparisons based on a tolerance value.
/// This data type combines an absolute epsilon value that will be used for comparisons based on
/// absolute epsilon values and an ulps value that will be used for comparisons based on
/// ulps values.
#[derive(Clone, Copy, Debug)]
pub struct Tolerance<Lhs, Rhs = Lhs>
where
    Lhs: ?Sized + ToleranceTypes<Rhs>,
    Rhs: ?Sized,
{
    pub eps: EpsToleranceType<Lhs, Rhs>,
    pub ulps: UlpsToleranceType<Lhs, Rhs>,
}

impl<Lhs: ?Sized + ToleranceTypes<Rhs>, Rhs: ?Sized> Tolerance<Lhs, Rhs> {
    pub fn new(eps: EpsToleranceType<Lhs, Rhs>, ulps: UlpsToleranceType<Lhs, Rhs>) -> Self {
        Tolerance::<Lhs, Rhs> { eps, ulps }
    }
}

impl<Lhs: ?Sized + ToleranceTypes<Rhs>, Rhs: ?Sized> Default for Tolerance<Lhs, Rhs> {
    fn default() -> Self {
        Tolerance::<Lhs, Rhs> {
            eps: <Lhs as EpsTolerance<Rhs>>::DEFAULT,
            ulps: <Lhs as UlpsTolerance<Rhs>>::DEFAULT,
        }
    }
}

impl<Lhs: ?Sized + ToleranceTypes<Rhs>, Rhs: ?Sized>
    From<(EpsToleranceType<Lhs, Rhs>, UlpsToleranceType<Lhs, Rhs>)> for Tolerance<Lhs, Rhs>
{
    fn from(tuple: (EpsToleranceType<Lhs, Rhs>, UlpsToleranceType<Lhs, Rhs>)) -> Self {
        Tolerance::<Lhs, Rhs> {
            eps: tuple.0,
            ulps: tuple.1,
        }
    }
}

impl<Lhs: ?Sized + ToleranceTypes<Rhs>, Rhs: ?Sized> From<Tolerance<Lhs, Rhs>>
    for (EpsToleranceType<Lhs, Rhs>, UlpsToleranceType<Lhs, Rhs>)
{
    fn from(val: Tolerance<Lhs, Rhs>) -> Self {
        (val.eps, val.ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::{EpsToleranceType, Tolerance, UlpsToleranceType};

    #[test]
    fn new_f32() {
        let tolerance = Tolerance::<f32>::new(0.01, 5);
        assert_eq!(tolerance.eps, 0.01);
        assert_eq!(tolerance.ulps, 5);
    }

    #[test]
    fn new_f64() {
        let tolerance = Tolerance::<f64>::new(0.01, 5);
        assert_eq!(tolerance.eps, 0.01);
        assert_eq!(tolerance.ulps, 5);
    }

    #[test]
    fn default_f32() {
        let tolerance = Tolerance::<f32>::default();
        assert_ne!(tolerance.eps, 0.0);
        assert_ne!(tolerance.ulps, 0);
    }

    #[test]
    fn default_f64() {
        let tolerance = Tolerance::<f64>::default();
        assert_ne!(tolerance.eps, 0.0);
        assert_ne!(tolerance.ulps, 0);
    }

    #[test]
    fn from_tuple_f32() {
        let tuple: (EpsToleranceType<f32>, UlpsToleranceType<f32>) = (0.01, 5);
        let tolerance = Tolerance::<f32>::from(tuple);
        assert_eq!(tolerance.eps, 0.01);
        assert_eq!(tolerance.ulps, 5);
    }

    #[test]
    fn from_tuple_f64() {
        let tuple: (EpsToleranceType<f64>, UlpsToleranceType<f64>) = (0.01, 5);
        let tolerance = Tolerance::<f64>::from(tuple);
        assert_eq!(tolerance.eps, 0.01);
        assert_eq!(tolerance.ulps, 5);
    }

    #[test]
    fn into_tuple_f32() {
        let tolerance = Tolerance::<f32> { eps: 0.01, ulps: 5 };
        let tuple: (EpsToleranceType<f32>, UlpsToleranceType<f32>) = tolerance.into();
        assert_eq!(tuple.0, 0.01);
        assert_eq!(tuple.1, 5);
    }

    #[test]
    fn into_tuple_f64() {
        let tolerance = Tolerance::<f64> { eps: 0.01, ulps: 5 };
        let tuple: (EpsToleranceType<f64>, UlpsToleranceType<f64>) = tolerance.into();
        assert_eq!(tuple.0, 0.01);
        assert_eq!(tuple.1, 5);
    }
}
