//! Compare IEEE floating point primitives by nearly comparisons.
//!
//! The issue in directly compare floating point primitives is, that they might be identical from a
//! logical point of view but because they have limited precision, they are not identical
//! bit by bit.
//!
//! Consider the following example, where a and b should be identical, but they are not:
//! ```
//! let a: f32 = 1.0 + 1.04 + 1.1;
//! let b: f32 = 3.14;
//!
//! assert!(a != b);
//! ```
//!
//! This crate provides functionality to solve this problem and offers traits and macros to
//! compare the floating point primitive types [f32] and [f64].
//!
//! # Comparison methods
//!
//! There are several comparison methods available in this crate you can choose from, depending on
//! your need and current situation.
//!
//! Methods available to choose for comparison are:
//!
//! | **Method**                               | **Corresponding Trait** | **Function postfix** | **Macro parameter** |
//! |------------------------------------------|-------------------------|----------------------|---------------------|
//! | absolute epsilon value                   | [NearlyEqEps]           | _eps                 | eps =               |
//! | ulps (unit of least precision)           | [NearlyEqUlps]          | _ulps                | ulps =              |
//! | Tolerance (epsilon + ulps)               | [NearlyEqTol]           | _tol                 | tol =               |
//! | default (default epsilon + default ulps) | [NearlyEq]              | n/a                  | n/a                 |
//!
//! # Usage
//!
//! There are multiple ways to use this crate for floating point comparisons. First via direct
//! functions implemented for [f32] and [f64] and second via macros. For both options there are
//! multiple comparison methods to choose from listed above.
//!
//! ## Function based
//!
//! Implemented for the types [f32] and [f64] are trait functions you can use to do the comparison.
//!
//! If you don't have any specific requirements regarding the tolerance used for comparison,
//! it's a good choice to stick with the default. In that case, the comparison will first
//! check the equality based on the absolute distance and if required based on the ulps distance
//! between the two inputs as well. The tolerance values used for these two comparisons
//! (epsilon and ulps) are the default values chosen by this crate for [f32] or [f64].
//!
//! To use the default comparison, you can call:
//!
//! ```
//! use nearly::NearlyEq;
//!
//! let a: f32 = 1.0 + 1.04 + 1.1;
//! let b: f32 = 3.14;
//!
//! assert!(a.nearly_eq(&b));
//! ```
//!
//! If you want to be more specific (which often is required, depending on the individual situation)
//! you are welcome to choose from one of the concrete functions:
//!
//! ```
//! use nearly::{NearlyEqEps, NearlyEqUlps, NearlyEqTol, ToleranceF32};
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//!
//! // compare a and b using the absolute distance with a tolerance of 0.001
//! assert!(a.nearly_eq_eps(&b, 0.001));
//!
//! // compare a and b using the ulps distance with a tolerance of 15 ulps
//! assert!(a.nearly_eq_ulps(&b, 15));
//!
//! // compare a and b using first the absolute distance with a tolerance of 0.001
//! // and second the ulps distance with a tolerance of 15 ulps
//! assert!(a.nearly_eq_tol(&b, ToleranceF32::new(0.001, 15)));
//! ```
//!
//! Using the default function `nearly_eq`is the same as using `nearly_eq_tol`with
//! `ToleranceF32::default()`.
//!
//! ```
//! # use nearly::{NearlyEq, NearlyEqTol, ToleranceF32};
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//! assert_eq!(
//!     a.nearly_eq(&b),
//!     a.nearly_eq_tol(&b, ToleranceF32::default()));
//! ```
//!
//! ## Macro based
//!
//! An alternative way to invoke a nearly comparison of two floating point primitives is by using
//! the macros this crate provides. These macros have exactly the same functionality as calling the
//! comparison function based. Instead of the function name, the used comparison method is
//! determined by the provided parameter.
//!
//! ```ignore
//! use nearly::{nearly_eq, ToleranceF32};
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//!
//! assert!( nearly_eq!(a, b) );
//! assert!( nearly_eq!(a, b, eps = 0.001) );
//! assert!( nearly_eq!(a, b, ulps = 15) );
//! assert!( nearly_eq!(a, b, eps = 0.001, ulps = 15) );
//! assert!( nearly_eq!(a, b, tol = ToleranceF32::new(0.001, 15)) );
//! ```
//!
//! This crate also provides assert macros for nearly comparison. These can be used as the
//! regular comparison macros, but they will panic if the comparison evaluates to false.
//!
//! ```ignore
//! use nearly::{assert_nearly_eq, ToleranceF32};
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//!
//! assert_nearly_eq!(a, b);
//! assert_nearly_eq!(a, b, eps = 0.001);
//! assert_nearly_eq!(a, b, ulps = 15);
//! assert_nearly_eq!(a, b, eps = 0.001, ulps = 15);
//! assert_nearly_eq!(a, b, tol = ToleranceF32::new(0.001, 15));
//! ```
//!
//! If you want to only assert a nearly comparison in debug runs, there are debug assert macros for
//! that. These are identical to the normal assert macros, except that they are only enabled in
//! non optimized builds.
//!
//! ```ignore
//! use nearly::{debug_assert_nearly_eq, ToleranceF32};
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//!
//! debug_assert_nearly_eq!(a, b);
//! debug_assert_nearly_eq!(a, b, eps = 0.001);
//! debug_assert_nearly_eq!(a, b, ulps = 15);
//! debug_assert_nearly_eq!(a, b, eps = 0.001, ulps = 15);
//! debug_assert_nearly_eq!(a, b, tol = ToleranceF32::new(0.001, 15));
//! ```
//!
//! # Implement your own types
//!
//! If you want to add the nearly comparison functionality to your own types, you can do so by
//! implementing the corresponding traits. The following examples explain all necessary steps.
//!
//! ## Simple struct
//!
//! Let's say we have a struct with two fields of type f32.
//!
//! ```
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//! ```
//!
//! First we have to implement the [EpsTolerance] nad [UlpsTolerance]. The [EpsTolerance] specifies
//! the type that is used for the epsilon tolerance during absolute difference comparisons as well
//! as the default value for that tolerance. The [UlpsTolerance] specifies the type that is used for
//! the ulps tolerance during ulps difference comparisons as well as the default value for that
//! tolerance.
//!
//! There are two options to define the types and default values. The first is to define them with
//! individual values, appropriate to your situation.
//!
//! ```
//! # struct Point { x: f32, y: f32 }
//! use nearly::{EpsTolerance, UlpsTolerance};
//!
//! impl EpsTolerance for Point {
//!     type T = f32;
//!     const DEFAULT: f32 = 0.0001;
//! }
//!
//! impl UlpsTolerance for Point {
//!     type T = i32;
//!     const DEFAULT: i32 = 10;
//! }
//! ```
//!
//! Another option is to use the the types and default values of the floating point type that
//! we used as the field types of our struct.
//!
//! ```
//! # struct Point { x: f32, y: f32 }
//! use nearly::{EpsTolerance, UlpsTolerance};
//!
//! impl EpsTolerance for Point {
//!     type T = <f32 as EpsTolerance>::T;
//!     const DEFAULT: Self::T = <f32 as EpsTolerance>::DEFAULT;
//! }
//!
//! impl UlpsTolerance for Point {
//!     type T = <f32 as UlpsTolerance>::T;
//!     const DEFAULT: Self::T = <f32 as UlpsTolerance>::DEFAULT;
//! }
//! ```
//!
//! After we have defined the tolerances, we have to implement the comparison traits.
//! These implementations specify how our struct is checked for nearly equality. In this
//! example we simply call the nearly equality for each field. Since our fields ar of type f32, we
//! can utilize the nearly comparison implementation this crate provides.
//!
//! ```
//! # use nearly::{EpsTolerance, UlpsTolerance};
//! # struct Point { x: f32, y: f32 }
//! # impl EpsTolerance for Point { type T = f32; const DEFAULT: f32 = 0.0001; }
//! # impl UlpsTolerance for Point { type T = i32; const DEFAULT: i32 = 10; }
//! use nearly::{
//!     EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, UlpsToleranceType
//! };
//!
//! impl NearlyEqEps for Point {
//!     fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool {
//!         self.x.nearly_eq_eps(&other.x, eps) && self.y.nearly_eq_eps(&other.y, eps)
//!     }
//! }
//!
//! impl NearlyEqUlps for Point {
//!     fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool {
//!         self.x.nearly_eq_ulps(&other.y, ulps) && self.y.nearly_eq(&other.y)
//!     }
//! }
//! ```
//!
//! # Struct with generic typed field
//!
//! If your struct contains fields of a generic type, you can add nearly comparison as well. The
//! following example shows how to do so on a struct with two fields having the same generic type.
//!
//! ```ignore
//! use nearly::{
//!     EpsAndUlpsTolerance, EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol,
//!     NearlyEqUlps, UlpsTolerance, UlpsToleranceType
//! };
//!
//! struct Point<T> {
//!     x: T,
//!     y: T,
//! }
//!
//! impl<T> EpsTolerance for Point<T>
//! where
//!     T: EpsTolerance,
//! {
//!     type T = <T as EpsTolerance>::T;
//!     const DEFAULT: Self::T = <T as EpsTolerance>::DEFAULT;
//! }
//!
//! impl<T> UlpsTolerance for Point<T>
//! where
//!     T: UlpsTolerance,
//! {
//!     type T = <T as UlpsTolerance>::T;
//!     const DEFAULT: Self::T = <T as UlpsTolerance>::DEFAULT;
//! }
//!
//! impl<T> NearlyEqEps for Point<T>
//! where
//!     T: NearlyEqEps + EpsTolerance,
//! {
//!     fn nearly_eq_eps(&self, other: &Self, eps: EpsToleranceType<Self>) -> bool {
//!         self.x.nearly_eq_eps(&other.x, eps) && self.y.nearly_eq_eps(&other.y, eps)
//!     }
//! }
//!
//! impl<T> NearlyEqUlps for Point<T>
//! where
//!     T: NearlyEqUlps + UlpsTolerance,
//! {
//!     fn nearly_eq_ulps(&self, other: &Self, ulps: UlpsToleranceType<Self>) -> bool {
//!         self.x.nearly_eq_ulps(&other.x, ulps) && self.y.nearly_eq_ulps(&other.y, ulps)
//!     }
//! }
//! ```
//!
//! ## Comparing two different structs
//!
//! So far, we implemented nearly comparison for a type to compare it with the same type.
//! This crate also provides the ability to implement a nearly comparison between two different
//! types. This is similar to if you would implement the [PartialEq] trait.
//!
//! This example also shows how to have a tuple tolerance type consist of two floating point types.
//!
//! ```
//! use nearly::{
//!     EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps,
//!     UlpsTolerance, UlpsToleranceType
//! };
//!
//! struct A {
//!     a_32: f32,
//!     a_64: f64,
//! }
//!
//! struct B {
//!     b_32: f32,
//!     b_64: f64,
//! }
//!
//!
//! impl EpsTolerance<B> for A {
//!     type T = (f32, f64);
//!     const DEFAULT: (f32, f64)  = (0.0001, 0.0000001);
//! }
//!
//! impl UlpsTolerance<B> for A {
//!     type T = (i32, i64);
//!     const DEFAULT: (i32, i64) = (4, 4);
//! }
//!
//! impl NearlyEqEps<B> for A {
//!     fn nearly_eq_eps(&self, other: &B, eps: EpsToleranceType<Self, B>) -> bool {
//!         self.a_32.nearly_eq_eps(&other.b_32, eps.0) &&
//!         self.a_64.nearly_eq_eps(&other.b_64, eps.1)
//!     }
//! }
//!
//! impl NearlyEqUlps<B> for A {
//!     fn nearly_eq_ulps(&self, other: &B, ulps: UlpsToleranceType<Self, B>) -> bool {
//!         self.a_32.nearly_eq_ulps(&other.b_32, ulps.0) &&
//!         self.a_64.nearly_eq_ulps(&other.b_64, ulps.1)
//!     }
//! }
//!
//!
//! // This implementation allows us to compare A with B
//! let a = A {a_32: 0.1, a_64: 0.01};
//! let b = B {b_32: 0.1, b_64: 0.01};
//!
//! assert!(a.nearly_eq(&b));
//! ```
//!
//! Note that this implementation only enables comparing an instance of type A with B, not the
//! other way around. If you want to compare B with A, you simply need to implement that
//! combination, too. You also can implement the nearly traits for A and B to enable comparisons
//! between the types itself, as shown in the first examples.
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use nearly_proc::assert_nearly;
pub use nearly_proc::debug_assert_nearly;
pub use nearly_proc::nearly;
pub use nearly_proc::NearlyEq;
pub use nearly_proc::NearlyEqEps;
pub use nearly_proc::NearlyEqUlps;

mod nearly_eq;
pub use nearly_eq::NearlyEq;
pub use nearly_eq::NearlyEqEps;
pub use nearly_eq::NearlyEqTol;
pub use nearly_eq::NearlyEqUlps;

mod tolerance;
pub use tolerance::EpsTolerance;
pub use tolerance::EpsToleranceType;
pub use tolerance::Tolerance;
pub use tolerance::ToleranceF32;
pub use tolerance::ToleranceF64;
pub use tolerance::UlpsTolerance;
pub use tolerance::UlpsToleranceType;

mod ulps;
pub use ulps::Ulps;
