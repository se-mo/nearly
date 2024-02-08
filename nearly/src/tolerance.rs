#[cfg(not(feature = "std"))]
use core::fmt::{Debug, Display, Formatter, Result};
#[cfg(feature = "std")]
use std::fmt::{Debug, Display, Formatter, Result};

/// A trait specifying the tolerance used for absolute epsilon based comparisons.
///
/// This tolerance if for the type implementing this trait comparing to `Rhs`.
pub trait EpsTolerance<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The type of the epsilon tolerance values.
    type T: Debug + Copy;

    /// The default epsilon tolerance value.
    /// Used for default [Tolerance](Tolerance::default()).
    const DEFAULT: Self::T;

    /// Returns the default eps tolerance value that is used when comparing self with other.
    fn default_eps(&self, _other: &Rhs) -> Self::T {
        Self::DEFAULT
    }
}

/// A type definition for convenience.
///
/// It specifies the type of the epsilon values for the type `Lhs` comparing to the type `Rhs`.
pub type EpsToleranceType<Lhs, Rhs = Lhs> = <Lhs as EpsTolerance<Rhs>>::T;

/// A trait specifying the tolerance used for ulps based comparisons.
///
/// This tolerance if for the type implementing this trait comparing to `Rhs`.
pub trait UlpsTolerance<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// The type of the ulps tolerance values.
    type T: Debug + Copy;

    /// The default ulps tolerance value.
    /// Used for default [Tolerance](Tolerance::default()).
    const DEFAULT: Self::T;

    /// Returns the default ulps tolerance value that is used when comparing self with other.
    fn default_ulps(&self, _other: &Rhs) -> Self::T {
        Self::DEFAULT
    }
}

/// A type definition for convenience.
///
/// It specifies the type of the ulps values for the type `Lhs` comparing to the type `Rhs`.
pub type UlpsToleranceType<Lhs, Rhs = Lhs> = <Lhs as UlpsTolerance<Rhs>>::T;

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

impl EpsTolerance for () {
    type T = ();
    const DEFAULT: () = ();
}

impl UlpsTolerance for () {
    type T = ();
    const DEFAULT: () = ();
}

/// A tolerance data type that is used for nearly comparisons based on a tolerance value.
///
/// This data type combines an absolute epsilon value that will be used for comparisons based on
/// absolute epsilon values and an ulps value that will be used for comparisons based on
/// ulps values.
pub struct Tolerance<Lhs, Rhs = Lhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
    pub eps: EpsToleranceType<Lhs, Rhs>,
    pub ulps: UlpsToleranceType<Lhs, Rhs>,
}

impl<Lhs, Rhs> Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
    pub fn new(eps: EpsToleranceType<Lhs, Rhs>, ulps: UlpsToleranceType<Lhs, Rhs>) -> Self {
        Tolerance::<Lhs, Rhs> { eps, ulps }
    }
}

impl<Lhs, Rhs> Default for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
    fn default() -> Self {
        Tolerance::<Lhs, Rhs> {
            eps: <Lhs as EpsTolerance<Rhs>>::DEFAULT,
            ulps: <Lhs as UlpsTolerance<Rhs>>::DEFAULT,
        }
    }
}

impl<Lhs, Rhs> From<(EpsToleranceType<Lhs, Rhs>, UlpsToleranceType<Lhs, Rhs>)>
    for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
    fn from(tuple: (EpsToleranceType<Lhs, Rhs>, UlpsToleranceType<Lhs, Rhs>)) -> Self {
        Tolerance::<Lhs, Rhs> {
            eps: tuple.0,
            ulps: tuple.1,
        }
    }
}

impl<Lhs, Rhs> From<Tolerance<Lhs, Rhs>>
    for (EpsToleranceType<Lhs, Rhs>, UlpsToleranceType<Lhs, Rhs>)
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
    fn from(val: Tolerance<Lhs, Rhs>) -> Self {
        (val.eps, val.ulps)
    }
}

impl<Lhs, Rhs> PartialEq for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
    EpsToleranceType<Lhs, Rhs>: PartialEq,
    UlpsToleranceType<Lhs, Rhs>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.eps == other.eps && self.ulps == other.ulps
    }
}

impl<Lhs, Rhs> Eq for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
    EpsToleranceType<Lhs, Rhs>: Eq,
    UlpsToleranceType<Lhs, Rhs>: Eq,
{
}

impl<Lhs, Rhs> Display for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
    EpsToleranceType<Lhs, Rhs>: Display,
    UlpsToleranceType<Lhs, Rhs>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(eps: {}, ulps: {})", self.eps, self.ulps)
    }
}

// We have to implement Copy and Clone explicitly, since derive
// of these traits doesn't work because of the generics used.
// See related issue: https://github.com/rust-lang/rust/issues/108894

impl<Lhs, Rhs> Clone for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Lhs, Rhs> Copy for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
}

// We implement the Debug trait explicitly. Deriving the Debug trait would add
// a trait bound to Lhs and Rhs to also implement Debug. This is not required.
// We only need the Debug trait bound for the corresponding eps and ulps values.
impl<Lhs, Rhs> Debug for Tolerance<Lhs, Rhs>
where
    Lhs: ?Sized + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    Rhs: ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Tolerance")
            .field("eps", &self.eps)
            .field("ulps", &self.ulps)
            .finish()
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

    #[test]
    fn clone_f32() {
        let tolerance = Tolerance::<f32>::new(0.01, 5);
        let tolerance_2 = tolerance;
        assert_eq!(tolerance.eps, tolerance_2.eps);
        assert_eq!(tolerance.ulps, tolerance_2.ulps);
    }

    #[test]
    fn clone_f64() {
        let tolerance = Tolerance::<f64>::new(0.01, 5);
        let tolerance_2 = tolerance;
        assert_eq!(tolerance.eps, tolerance_2.eps);
        assert_eq!(tolerance.ulps, tolerance_2.ulps);
    }

    #[test]
    fn equal_f32() {
        let tolerance = Tolerance::<f32>::new(0.01, 5);
        assert_eq!(tolerance, Tolerance::<f32>::new(0.01, 5));
        assert_ne!(tolerance, Tolerance::<f32>::new(0.02, 5));
        assert_ne!(tolerance, Tolerance::<f32>::new(0.01, 10));
    }

    #[test]
    fn equal_f64() {
        let tolerance = Tolerance::<f64>::new(0.01, 5);
        assert_eq!(tolerance, Tolerance::<f64>::new(0.01, 5));
        assert_ne!(tolerance, Tolerance::<f64>::new(0.02, 5));
        assert_ne!(tolerance, Tolerance::<f64>::new(0.01, 10));
    }
}
