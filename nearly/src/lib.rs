//! Compare IEEE floating point types by nearly comparisons.
//!
//! When comparing floating point types, because of their limited precision, they might not be
//! exactly identical. Consider the following example, where a and b appear to be identical, but
//! they are not:
//!
//! ```should_panic
//! let a: f32 = 1.0 + 1.04 + 1.1;
//! let b: f32 = 3.14;
//!
//! assert!(a == b); // <-- PANICS
//! ```
//!
//! This crate provides macros to perform a comparison with some tolerance.
//!
//! ```
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//! use nearly::nearly;
//! assert!( nearly!(a == b) ); // <-- OK
//! ```
//!
//! # Usage
//!
//! The easiest way to use nearly comparisons is by invoking the [nearly!] macro. The macro
//! returns a boolean whether the comparison is true or false by using the provided tolerance.
//!
//! The comparison can be:
//!   - `a == b` for testing whether a is nearly equal to b
//!   - `a != b` for testing whether a is not nearly equal to b
//!   - `a < b` for testing whether a is strict less than b but not nearly equal to b
//!   - `a <= b` for testing whether a is strict less than b or nearly equal to b
//!   - `a > b` for testing whether a is strict greater than b but not nearly equal to b
//!   - `a >= b` for testing whether a is strict greater than b or nearly equal to b
//!
//! The tolerance used can be:
//!   - `eps` for an absolute epsilon tolerance
//!   - `ulps` for an ulps based tolerance
//!   - `tol` for an absolute epsilon and ulps based tolerance
//!   - `default` for an absolute epsilon and ulps based tolerance using default values
//!
//! Here are some example calls:
//!
//! ```
//! use nearly::{nearly, Tolerance};
//!
//! let a: f32 = 1.0 + 1.04 + 1.1;
//! let b: f32 = 3.14;
//!
//! // use absolute epsilon tolerance
//! nearly!(a == b, eps = 0.001);
//!
//! // use ulps based tolerance
//! nearly!(a == b, ulps = 5);
//!
//! // use absolute epsilon and ulps based tolerance
//! nearly!(a == b, eps = 0.001, ulps = 5);
//! nearly!(a == b, tol = Tolerance::new(0.001, 5));
//!
//! // use default absolute epsilon and default ulps based tolerance
//! nearly!(a == b);
//! ```
//!
//! There is also an [assert_nearly!] and [debug_assert_nearly!] macro you can use that panic
//! if the nearly comparison evaluates to false. The signature is the same as for the [nearly!]
//! macro.
//!
//! ```
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//! use nearly::{assert_nearly, debug_assert_nearly, Tolerance};
//!
//! assert_nearly!(a == b, eps = 0.001);
//! assert_nearly!(a == b, ulps = 5);
//! assert_nearly!(a == b, eps = 0.001, ulps = 5);
//! assert_nearly!(a == b, tol = Tolerance::new(0.001, 5));
//! assert_nearly!(a == b);
//!
//! debug_assert_nearly!(a == b, eps = 0.001);
//! debug_assert_nearly!(a == b, ulps = 5);
//! debug_assert_nearly!(a == b, eps = 0.001, ulps = 5);
//! debug_assert_nearly!(a == b, tol = Tolerance::new(0.001, 5));
//! debug_assert_nearly!(a == b);
//! ```
//!
//! If required, you can invoke the corresponding trait functions directly instead of using the
//! macro. The macro use is recommended, though.
//!
//! ```
//! # let a: f32 = 1.0 + 1.04 + 1.1;
//! # let b: f32 = 3.14;
//! use nearly::{NearlyEqEps, NearlyEqUlps, NearlyEqTol, NearlyEq, Tolerance};
//!
//! assert!(a.nearly_eq_eps(&b, &0.001));
//! assert!(a.nearly_eq_ulps(&b, &5));
//! assert!(a.nearly_eq_tol(&b, &Tolerance::new(0.001, 5)));
//! assert!(a.nearly_eq(&b));
//! ```
//!
//! The nearly functionality is also implemented for a variety of other types holding floats like
//! containers, maps, pointers or tuples. Here is an example of comparing two arrays of floats.
//!
//! ```
//! use nearly::nearly;
//!
//! let a: [f32; 4] = [1.1, 2.2, 2.2, 4.4];
//! let b: [f32; 4] = [1.1, 2.2, 3.3, 4.4];
//!
//! nearly!(a <= b, eps = 0.001, ulps = 5);
//! ```
//!
//! # Own types
//!
//! ## Derive the nearly traits
//!
//! The easiest way to add nearly comparison to your own types is by deriving the nearly traits.
//! Just derive [NearlyEq](nearly_macros::NearlyEq) and [NearlyOrd](nearly_macros::NearlyOrd)
//! to get full support on your type.
//!
//! ```
//! use nearly::{assert_nearly, NearlyEq, NearlyOrd};
//!
//! #[derive(Debug, NearlyEq, NearlyOrd)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! let a = Point { x: 1.23, y: 4.56 };
//! let b = Point { x: 1.23, y: 4.567 };
//!
//! assert_nearly!(a == b, eps = 0.01);
//! assert_nearly!(a <= b, eps = 0.01);
//! ```
//!
//! To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
//! the Debug trait.
//!
//! You can derive the following traits:
//!   - [NearlyEqEps](nearly_macros::NearlyEqEps): enables nearly equality support with
//!     absolute epsilon tolerance
//!   - [NearlyEqUlps](nearly_macros::NearlyEqUlps): enables nearly equality support with
//!     ulps based tolerance
//!   - [NearlyEqTol][nearly_macros::NearlyEqTol]: enables nearly equality support with
//!     absolute epsilon and ulps based tolerances
//!   - [NearlyEq](nearly_macros::NearlyEq): enables nearly equality support with
//!     absolute epsilon and ulps based tolerances with default values
//!   - [NearlyOrdEps](nearly_macros::NearlyOrdEps): enables nearly ordering support with
//!     absolute epsilon tolerance
//!   - [NearlyOrdUlps](nearly_macros::NearlyOrdUlps): enables nearly ordering support with
//!     ulps based tolerance
//!   - [NearlyOrdTol][nearly_macros::NearlyOrdTol]: enables nearly ordering support with
//!     absolute epsilon and ulps based tolerances
//!   - [NearlyOrd](nearly_macros::NearlyOrd): enables nearly ordering support with
//!     absolute epsilon and ulps based tolerances with default values
//!  
//! ## Implement the nearly traits
//!
//! If required, you can also implement the nearly traits by your own.
//!
//! ### Simple struct
//!
//! Let's say we have a struct with two fields of type [f32].
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
//! After we have defined the tolerances, we have to implement the comparison traits.
//! These implementations specify how our struct is checked for nearly equality and ordering.
//! In this example we simply call the corresponding nearly comparison for each field. Since our
//! fields are of type [f32], we can utilize the nearly comparison implementation this crate provides.
//!
//! Fist let's implement the nearly equality traits:
//!
//! ```
//! # use nearly::{EpsTolerance, UlpsTolerance};
//! # struct Point { x: f32, y: f32 }
//! # impl EpsTolerance for Point { type T = f32; const DEFAULT: f32 = 0.0001; }
//! # impl UlpsTolerance for Point { type T = i32; const DEFAULT: i32 = 10; }
//! use nearly::{
//!     EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps, Tolerance,
//!     UlpsToleranceType
//! };
//!
//! impl NearlyEqEps for Point {
//!     fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
//!         self.x.nearly_eq_eps(&other.x, eps) && self.y.nearly_eq_eps(&other.y, eps)
//!     }
//! }
//!
//! impl NearlyEqUlps for Point {
//!     fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
//!         self.x.nearly_eq_ulps(&other.x, ulps) && self.y.nearly_eq_ulps(&other.y, ulps)
//!     }
//! }
//!
//! impl NearlyEqTol for Point {
//!     fn nearly_eq_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool {
//!         self.x.nearly_eq_tol(&other.x, &(tol.eps, tol.ulps).into()) &&
//!         self.y.nearly_eq_tol(&other.y, &(tol.eps, tol.ulps).into())
//!     }
//! }
//!
//! // use provided trait implementation
//! impl NearlyEq for Point {}
//! ```
//!
//! Now let's also implement the nearly ordering traits:
//!
//! ```
//! # use nearly::{NearlyEq};
//! # #[derive(NearlyEq)] struct Point { x: f32, y: f32 }
//! use nearly::{
//!     EpsToleranceType, NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps, Tolerance,
//!     UlpsToleranceType
//! };
//!
//! impl NearlyOrdEps for Point {
//!     fn nearly_lt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
//!         self.x.nearly_lt_eps(&other.x, eps) && self.y.nearly_lt_eps(&other.y, eps)
//!     }
//!     fn nearly_le_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
//!         self.x.nearly_le_eps(&other.x, eps) && self.y.nearly_le_eps(&other.y, eps)
//!     }
//!     fn nearly_gt_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
//!         self.x.nearly_gt_eps(&other.x, eps) && self.y.nearly_gt_eps(&other.y, eps)
//!     }
//!     fn nearly_ge_eps(&self, other: &Self, eps: &EpsToleranceType<Self>) -> bool {
//!         self.x.nearly_ge_eps(&other.x, eps) && self.y.nearly_ge_eps(&other.y, eps)
//!     }
//! }
//!
//! impl NearlyOrdUlps for Point {
//!     fn nearly_lt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
//!         self.x.nearly_lt_ulps(&other.x, ulps) && self.y.nearly_lt_ulps(&other.y, ulps)
//!     }
//!     fn nearly_le_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
//!         self.x.nearly_le_ulps(&other.x, ulps) && self.y.nearly_le_ulps(&other.y, ulps)
//!     }
//!     fn nearly_gt_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
//!         self.x.nearly_gt_ulps(&other.x, ulps) && self.y.nearly_gt_ulps(&other.y, ulps)
//!     }
//!     fn nearly_ge_ulps(&self, other: &Self, ulps: &UlpsToleranceType<Self>) -> bool {
//!         self.x.nearly_ge_ulps(&other.x, ulps) && self.y.nearly_ge_ulps(&other.y, ulps)
//!     }
//! }
//!
//! impl NearlyOrdTol for Point {
//!     fn nearly_lt_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool {
//!         self.x.nearly_lt_tol(&other.x, &(tol.eps, tol.ulps).into()) &&
//!         self.y.nearly_lt_tol(&other.y, &(tol.eps, tol.ulps).into())
//!     }
//!     fn nearly_le_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool {
//!         self.x.nearly_le_tol(&other.x, &(tol.eps, tol.ulps).into()) &&
//!         self.y.nearly_le_tol(&other.y, &(tol.eps, tol.ulps).into())
//!     }
//!     fn nearly_gt_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool {
//!         self.x.nearly_gt_tol(&other.x, &(tol.eps, tol.ulps).into()) &&
//!         self.y.nearly_gt_tol(&other.y, &(tol.eps, tol.ulps).into())
//!     }
//!     fn nearly_ge_tol(&self, other: &Self, tol: &Tolerance<Self>) -> bool {
//!         self.x.nearly_ge_tol(&other.x, &(tol.eps, tol.ulps).into()) &&
//!         self.y.nearly_ge_tol(&other.y, &(tol.eps, tol.ulps).into())
//!     }
//! }
//!
//! // use provided trait implementation
//! impl NearlyOrd for Point {}
//! ```
//!
//! Please not that when implementing the nearly ordering traits, you also must implement the
//! nearly equality traits.
//!
//! ### Comparing two different structs
//!
//! So far, we implemented nearly comparison for a type to compare it with the same type.
//! This crate also allows to implement a nearly comparison between two different types.
//! This is similar to implementing the [PartialEq] trait.
//!
//! This following example shows such an implementation for the nearly equality traits.
//! You can also implement the nearly ordering traits in the same way.
//! The example also shows the use of generic typed fields.
//!
//! ```
//! use nearly::{
//!     assert_nearly, EpsTolerance, EpsToleranceType, NearlyEq, NearlyEqEps, NearlyEqTol,
//!     NearlyEqUlps, Tolerance, UlpsTolerance, UlpsToleranceType
//! };
//!
//! #[derive(Debug)]
//! struct A<T> {
//!     x: T,
//!     y: T,
//! }
//!
//! #[derive(Debug)]
//! struct B<T> {
//!     u: T,
//!     v: T,
//! }
//!
//! impl<T> EpsTolerance<B<T>> for A<T>
//! where
//!     T: EpsTolerance,
//! {
//!     type T = <T as EpsTolerance>::T;
//!     const DEFAULT: Self::T = <T as EpsTolerance>::DEFAULT;
//! }
//!
//! impl<T> UlpsTolerance<B<T>> for A<T>
//! where
//!     T: UlpsTolerance,
//! {
//!     type T = <T as UlpsTolerance>::T;
//!     const DEFAULT: Self::T = <T as UlpsTolerance>::DEFAULT;
//! }
//!
//! impl<T> NearlyEqEps<B<T>> for A<T>
//! where
//!     T: NearlyEqEps + EpsTolerance,
//! {
//!     fn nearly_eq_eps(&self, other: &B<T>, eps: &EpsToleranceType<Self, B<T>>) -> bool {
//!         self.x.nearly_eq_eps(&other.u, eps) && self.y.nearly_eq_eps(&other.v, eps)
//!     }
//! }
//!
//! impl<T> NearlyEqUlps<B<T>> for A<T>
//! where
//!     T: NearlyEqUlps + UlpsTolerance,
//! {
//!     fn nearly_eq_ulps(&self, other: &B<T>, ulps: &UlpsToleranceType<Self, B<T>>) -> bool {
//!         self.x.nearly_eq_ulps(&other.u, ulps) && self.y.nearly_eq_ulps(&other.v, ulps)
//!     }
//! }
//!
//! impl<T> NearlyEqTol<B<T>> for A<T>
//! where
//!     T: NearlyEqTol + EpsTolerance + UlpsTolerance
//! {
//!     fn nearly_eq_tol(&self, other: &B<T>, tol: &Tolerance<Self, B<T>>) -> bool {
//!         self.x.nearly_eq_tol(&other.u, &(tol.eps, tol.ulps).into()) &&
//!         self.y.nearly_eq_tol(&other.v, &(tol.eps, tol.ulps).into())
//!     }
//! }
//!
//! // use provided trait implementation
//! impl<T> NearlyEq<B<T>> for A<T> where T: NearlyEq + EpsTolerance + UlpsTolerance {}
//!
//!
//! // This implementation allows us to compare A with B
//! let a = A {x: 0.1, y: 0.01};
//! let b = B {u: 0.1, v: 0.01};
//!
//! assert_nearly!(a == b);
//! ```
//!
//! Note that this implementation only enables comparing an instance of type A with B, not the
//! other way around. If you want to compare B with A, you simply need to implement that
//! combination, too. You also can implement the nearly traits for A and B to enable comparisons
//! between the types itself, as shown in the first examples.

#![cfg_attr(not(feature = "std"), no_std)]

/// Asserts that the given comparison is nearly true using the provided tolerance.
///
/// On panic, this macro will print the values of the comparison with their debug
/// representations as well as the values of the provided tolerance.
///
/// The comparison can be:
///   - `a == b` for testing whether a is nearly equal to b
///   - `a != b` for testing whether a is not nearly equal to b
///
/// The tolerance used can be:
///   - `eps` for an absolute epsilon tolerance
///   - `ulps` for an ulps based tolerance
///   - `tol` for an absolute epsilon and ulps based tolerance
///   - `default` for an absolute epsilon and ulps based tolerance using default values
///
/// # Examples
///
/// ```
/// use nearly::{assert_nearly, Tolerance};
///
/// let a: f32 = 1.0;
/// let b: f32 = 1.0;
///
/// // use absolute epsilon tolerance
/// assert_nearly!(a == b, eps = 0.01);
///
/// // use ulps based tolerance
/// assert_nearly!(a == b, ulps = 5);
///
/// // use absolute epsilon and ulps based tolerance
/// assert_nearly!(a == b, eps = 0.01, ulps = 5);
/// assert_nearly!(a == b, tol = Tolerance::new(0.01, 5));
///
/// // use default absolute epsilon and default ulps based tolerance
/// assert_nearly!(a == b);
/// ```
pub use nearly_macros::assert_nearly;

/// Asserts that the given comparison is nearly true using the provided tolerance.
///
/// On panic, this macro will print the values of the comparison with their debug
/// representations as well as the values of the provided tolerance.
///
/// Like [debug_assert!] this macro is only enabled in non optimized builds.
///
/// The comparison can be:
///   - `a == b` for testing whether a is nearly equal to b
///   - `a != b` for testing whether a is not nearly equal to b
///
/// The tolerance used can be:
///   - `eps` for an absolute epsilon tolerance
///   - `ulps` for an ulps based tolerance
///   - `tol` for an absolute epsilon and ulps based tolerance
///   - `default` for an absolute epsilon and ulps based tolerance using default values
///
/// # Examples
///
/// ```
/// use nearly::{debug_assert_nearly, Tolerance};
///
/// let a: f32 = 1.0;
/// let b: f32 = 1.0;
///
/// // use absolute epsilon tolerance
/// debug_assert_nearly!(a == b, eps = 0.01);
///
/// // use ulps based tolerance
/// debug_assert_nearly!(a == b, ulps = 5);
///
/// // use absolute epsilon and ulps based tolerance
/// debug_assert_nearly!(a == b, eps = 0.01, ulps = 5);
/// debug_assert_nearly!(a == b, tol = Tolerance::new(0.01, 5));
///
/// // use default absolute epsilon and default ulps based tolerance
/// debug_assert_nearly!(a == b);
/// ```
pub use nearly_macros::debug_assert_nearly;

/// Returns whether the given comparison is nearly true using the provided tolerance.
///
/// The comparison can be:
///   - `a == b` for testing whether a is nearly equal to b
///   - `a != b` for testing whether a is not nearly equal to b
///
/// The tolerance used can be:
///   - `eps` for an absolute epsilon tolerance
///   - `ulps` for an ulps based tolerance
///   - `tol` for an absolute epsilon and ulps based tolerance
///   - `default` for an absolute epsilon and ulps based tolerance using default values
///
/// # Examples
///
/// ```
/// use nearly::{nearly, Tolerance};
///
/// let a: f32 = 1.0;
/// let b: f32 = 1.0;
///
/// // use absolute epsilon tolerance
/// let eq: bool = nearly!(a == b, eps = 0.01);
///
/// // use ulps based tolerance
/// let eq: bool = nearly!(a == b, ulps = 5);
///
/// // use absolute epsilon and ulps based tolerance
/// let eq: bool = nearly!(a == b, eps = 0.01, ulps = 5);
/// let eq: bool = nearly!(a == b, tol = Tolerance::new(0.01, 5));
///
/// // use default absolute epsilon and default ulps based tolerance
/// let eq: bool = nearly!(a == b);
/// ```
pub use nearly_macros::nearly;

/// Derives the [NearlyEqEps] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyEqEps].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type, the epsilon tolerance will have the same type as the
/// epsilon tolerance of the fields type. E.g., for [f32] this would be [f32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEqEps};
///
/// #[derive(NearlyEqEps, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, eps = 0.0001);
/// ```
///
/// ## Different Types
///
/// If the fields have different types, the epsilon tolerance will have a tuple type. The tuple
/// will consist of the epsilon types of the fields type in the same order as they are defined.
/// E.g., for fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEqEps};
///
/// #[derive(NearlyEqEps, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, eps = (0.0001, 0.000001, 0.0001));
/// ```
pub use nearly_macros::NearlyEqEps;

/// Derives the [NearlyEqUlps] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyEqUlps].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type, the epsilon tolerance will have the same type as the
/// epsilon tolerance of the fields type. E.g., for [f32] this would be [i32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEqUlps};
///
/// #[derive(NearlyEqUlps, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, ulps = 8);
/// ```
///
/// ## Different Types
///
/// If the fields have different types, the epsilon tolerance will have a tuple type. The tuple
/// will consist of the epsilon types of the fields type in the same order as they are defined.
/// E.g., for fields with the type [f32], [f64] and [f32] this would be ([i32], [i64], [i32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEqUlps};
///
/// #[derive(NearlyEqUlps, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, ulps = (8, 12, 8));
/// ```
pub use nearly_macros::NearlyEqUlps;

/// Derives the [NearlyEqTol] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyEqTol].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type:
///   - the epsilon tolerance will have the same type as the epsilon tolerance of the fields type.
///     E.g., for [f32] this would be [f32].
///   - the ulps tolerance will have the same type as the ulps tolerance of the fields type.
///     E.g., for [f32] this would be [i32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEqTol, Tolerance};
///
/// #[derive(NearlyEqTol, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, tol = Tolerance::new(0.0001, 8));
/// ```
///
/// ## Different Types
///
/// If the fields have different types:
///   - the epsilon tolerance will have a tuple type. The tuple will consist of the epsilon types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
///   - the ulps tolerance will have a tuple type. The tuple will consist of the ulps types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64], [f32] this would be ([i32], [i64], [i32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEqTol, Tolerance};
///
/// #[derive(NearlyEqTol, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, tol = Tolerance::new((0.0001, 0.000001, 0.0001), (8, 12, 8)));
/// ```
pub use nearly_macros::NearlyEqTol;

/// Derives all nearly equality traits for a custom type.
///
/// The derived traits are: [NearlyEqEps], [NearlyEqUlps], [NearlyEqTol] and [NearlyEq].
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented
/// [NearlyEqEps], [NearlyEqUlps], [NearlyEqTol] and [NearlyEq].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type:
///   - the epsilon tolerance will have the same type as the epsilon tolerance of the fields type.
///     E.g., for [f32] this would be [f32].
///   - the ulps tolerance will have the same type as the ulps tolerance of the fields type.
///     E.g., for [f32] this would be [i32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEq};
///
/// #[derive(NearlyEq, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, eps = 0.0001);
/// assert_nearly!(a == b, ulps = 8);
/// assert_nearly!(a == b, eps = 0.0001, ulps = 8);
/// assert_nearly!(a == b);
/// ```
///
/// ## Different Types
///
/// If the fields have different types:
///   - the epsilon tolerance will have a tuple type. The tuple will consist of the epsilon types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
///   - the ulps tolerance will have a tuple type. The tuple will consist of the ulps types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64], [f32] this would be ([i32], [i64], [i32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEq};
///
/// #[derive(NearlyEq, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a == b, eps = (0.0001, 0.000001, 0.0001));
/// assert_nearly!(a == b, ulps = (8, 12, 8));
/// assert_nearly!(a == b, eps = (0.0001, 0.000001, 0.0001), ulps = (8, 12, 8));
/// assert_nearly!(a == b);
/// ```
pub use nearly_macros::NearlyEq;

/// Derives the [NearlyOrdEps] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyOrdEps].
///
/// To derive [NearlyOrdEps] on a type, this type also has to implement or derive [NearlyEqEps].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type, the epsilon tolerance will have the same type as the
/// epsilon tolerance of the fields type. E.g., for [f32] this would be [f32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEqEps, NearlyOrdEps};
///
/// #[derive(NearlyEqEps, NearlyOrdEps, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, eps = 0.0001);
/// ```
///
/// ## Different Types
///
/// If the fields have different types, the epsilon tolerance will have a tuple type. The tuple
/// will consist of the epsilon types of the fields type in the same order as they are defined.
/// E.g., for fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEqEps, NearlyOrdEps};
///
/// #[derive(NearlyEqEps, NearlyOrdEps, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, eps = (0.0001, 0.000001, 0.0001));
/// ```
pub use nearly_macros::NearlyOrdEps;

/// Derives the [NearlyOrdUlps] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyEqUlps].
///
/// To derive [NearlyOrdUlps] on a type, this type also has to implement or derive [NearlyEqUlps].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type, the epsilon tolerance will have the same type as the
/// epsilon tolerance of the fields type. E.g., for [f32] this would be [i32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEqUlps, NearlyOrdUlps};
///
/// #[derive(NearlyEqUlps, NearlyOrdUlps, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, ulps = 8);
/// ```
///
/// ## Different Types
///
/// If the fields have different types, the epsilon tolerance will have a tuple type. The tuple
/// will consist of the epsilon types of the fields type in the same order as they are defined.
/// E.g., for fields with the type [f32], [f64] and [f32] this would be ([i32], [i64], [i32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEqUlps, NearlyOrdUlps};
///
/// #[derive(NearlyEqUlps, NearlyOrdUlps, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, ulps = (8, 12, 8));
/// ```
pub use nearly_macros::NearlyOrdUlps;

/// Derives the [NearlyOrdTol] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyEqTol].
///
/// To derive [NearlyOrdTol] on a type, this type also has to implement or derive [NearlyEqTol].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type:
///   - the epsilon tolerance will have the same type as the epsilon tolerance of the fields type.
///     E.g., for [f32] this would be [f32].
///   - the ulps tolerance will have the same type as the ulps tolerance of the fields type.
///     E.g., for [f32] this would be [i32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEqTol, NearlyOrdTol, Tolerance};
///
/// #[derive(NearlyEqTol, NearlyOrdTol, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, tol = Tolerance::new(0.0001, 8));
/// ```
///
/// ## Different Types
///
/// If the fields have different types:
///   - the epsilon tolerance will have a tuple type. The tuple will consist of the epsilon types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
///   - the ulps tolerance will have a tuple type. The tuple will consist of the ulps types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64], [f32] this would be ([i32], [i64], [i32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEqTol, NearlyOrdTol, Tolerance};
///
/// #[derive(NearlyEqTol, NearlyOrdTol, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, tol = Tolerance::new((0.0001, 0.000001, 0.0001), (8, 12, 8)));
/// ```
pub use nearly_macros::NearlyOrdTol;

/// Derives all nearly ordering traits for a custom type.
///
/// The derived traits are: [NearlyOrdEps], [NearlyOrdUlps], [NearlyOrdTol] and [NearlyOrd].
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented
/// [NearlyOrdEps], [NearlyOrdUlps], [NearlyOrdTol] and [NearlyOrd].
///
/// To derive all nearly ordering traits on a type, this type also has to implement or derive all
/// nearly equality traits [NearlyEqEps], [NearlyEqUlps], [NearlyEqTol] and [NearlyEq].
///
/// To use the [assert_nearly!] and [debug_assert_nearly!] macros, your type must also implement
/// the Debug trait.
///
/// # Example
///
/// ## Same Type
///
/// If all fields have the same type:
///   - the epsilon tolerance will have the same type as the epsilon tolerance of the fields type.
///     E.g., for [f32] this would be [f32].
///   - the ulps tolerance will have the same type as the ulps tolerance of the fields type.
///     E.g., for [f32] this would be [i32].
///
/// ```
/// use nearly::{assert_nearly, NearlyEq, NearlyOrd};
///
/// #[derive(NearlyEq, NearlyOrd, Debug)]
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, eps = 0.0001);
/// assert_nearly!(a <= b, ulps = 8);
/// assert_nearly!(a <= b, eps = 0.0001, ulps = 8);
/// assert_nearly!(a <= b);
/// ```
///
/// ## Different Types
///
/// If the fields have different types:
///   - the epsilon tolerance will have a tuple type. The tuple will consist of the epsilon types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
///   - the ulps tolerance will have a tuple type. The tuple will consist of the ulps types
///     of the fields type in the same order as they are defined.
///     E.g., for fields with the type [f32], [f64], [f32] this would be ([i32], [i64], [i32]).
///
/// ```
/// use nearly::{assert_nearly, NearlyEq, NearlyOrd};
///
/// #[derive(NearlyEq, NearlyOrd, Debug)]
/// struct Point {
///     x: f32,
///     y: f64,
///     z: f32,
/// }
///
/// let a = Point{x: -3.4, y: 2.1, z: 1.0};
/// let b = Point{x: -3.4, y: 2.1, z: 1.0000008};
///
/// assert_nearly!(a <= b, eps = (0.0001, 0.000001, 0.0001));
/// assert_nearly!(a <= b, ulps = (8, 12, 8));
/// assert_nearly!(a <= b, eps = (0.0001, 0.000001, 0.0001), ulps = (8, 12, 8));
/// assert_nearly!(a <= b);
/// ```
pub use nearly_macros::NearlyOrd;

mod nearly_eq;
pub use nearly_eq::NearlyEq;
pub use nearly_eq::NearlyEqEps;
pub use nearly_eq::NearlyEqTol;
pub use nearly_eq::NearlyEqUlps;

mod nearly_ord;
pub use nearly_ord::NearlyOrd;
pub use nearly_ord::NearlyOrdEps;
pub use nearly_ord::NearlyOrdTol;
pub use nearly_ord::NearlyOrdUlps;

mod tolerance;
pub use tolerance::EpsTolerance;
pub use tolerance::EpsToleranceType;
pub use tolerance::Tolerance;
pub use tolerance::UlpsTolerance;
pub use tolerance::UlpsToleranceType;

mod trait_impl;
mod ulps;
