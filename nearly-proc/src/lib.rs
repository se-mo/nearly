//! This crate provides proc macros for the [nearly](https://docs.rs/nearly/latest/nearly/) crate.

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod derive;
mod nearly;

/// Returns whether the given comparison is nearly true using the provided tolerance.
///
/// Comparison can be equal (`==`) or unequal (`!=`).
///
/// # Examples
/// Comparison can be based on different tolerances:
/// ```
/// use nearly::nearly;
/// use nearly::ToleranceF32;
///
/// let a: f32 = 1.0;
/// let b: f32 = 1.0;
///
/// // use epsilon based comparison
/// let eq: bool = nearly!(a == b, eps = 0.01);
///
/// // use ulps based comparison
/// let eq: bool = nearly!(a == b, ulps = 5);
///
/// // use epsilon and ulps based comparison
/// let eq: bool = nearly!(a == b, eps = 0.01, ulps = 5);
/// let eq: bool = nearly!(a == b, tol = ToleranceF32::new(0.01, 5));
///
/// // use epsilon and ulps based comparison with default tolerance
/// let eq: bool = nearly!(a == b);
/// ```
#[proc_macro]
pub fn nearly(input: TokenStream) -> TokenStream {
    nearly::nearly_macro(input, nearly::NearlyMacroType::Standard)
}

/// Asserts that the given comparison is nearly true using the provided tolerance.
///
/// On panic, this macro will print the values of the comparison with their debug
/// representations as well as the values of the provided tolerance.
///
/// Comparison can be equal (`==`) or unequal (`!=`).
///
/// # Examples
/// Comparison can be based on different tolerances:
/// ```
/// use nearly::assert_nearly;
/// use nearly::ToleranceF32;
///
/// let a: f32 = 1.0;
/// let b: f32 = 1.0;
///
/// // use epsilon based comparison
/// assert_nearly!(a == b, eps = 0.01);
///
/// // use ulps based comparison
/// assert_nearly!(a == b, ulps = 5);
///
/// // use epsilon and ulps based comparison
/// assert_nearly!(a == b, eps = 0.01, ulps = 5);
/// assert_nearly!(a == b, tol = ToleranceF32::new(0.01, 5));
///
/// // use epsilon and ulps based comparison with default tolerance
/// assert_nearly!(a == b);
/// ```
#[proc_macro]
pub fn assert_nearly(input: TokenStream) -> TokenStream {
    nearly::nearly_macro(input, nearly::NearlyMacroType::Assert)
}

/// Asserts that the given comparison is nearly true using the provided tolerance.
///
/// On panic, this macro will print the values of the comparison with their debug
/// representations as well as the values of the provided tolerance.
///
/// Like [debug_assert!] this macro is only enabled in non optimized builds.
///
/// Comparison can be equal (`==`) or unequal (`!=`).
///
/// # Examples
/// Comparison can be based on different tolerances:
/// ```
/// use nearly::debug_assert_nearly;
/// use nearly::ToleranceF32;
///
/// let a: f32 = 1.0;
/// let b: f32 = 1.0;
///
/// // use epsilon based comparison
/// debug_assert_nearly!(a == b, eps = 0.01);
///
/// // use ulps based comparison
/// debug_assert_nearly!(a == b, ulps = 5);
///
/// // use epsilon and ulps based comparison
/// debug_assert_nearly!(a == b, eps = 0.01, ulps = 5);
/// debug_assert_nearly!(a == b, tol = ToleranceF32::new(0.01, 5));
///
/// // use epsilon and ulps based comparison with default tolerance
/// debug_assert_nearly!(a == b);
/// ```
#[proc_macro]
pub fn debug_assert_nearly(input: TokenStream) -> TokenStream {
    nearly::nearly_macro(input, nearly::NearlyMacroType::DebugAssert)
}

/// Derives the [NearlyEqEps] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyEqEps].
///
/// # Example
/// ## Same Type
/// If all fields have the same type, the epsilon tolerance will have the same type as the
/// epsilon tolerance of the fields type. For [f32] this would be [f32].
/// ```
/// use nearly::NearlyEqEps;
/// use nearly::assert_nearly;
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
/// If the fields have different types, the epsilon tolerance will have a tuple type. The tuple will
/// consist of the epsilon types of the fields type in the same order as they are defined.
/// For fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
/// ```
/// use nearly::NearlyEqEps;
/// use nearly::assert_nearly;
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
///
#[proc_macro_derive(NearlyEqEps)]
#[proc_macro_error]
pub fn nearly_eq_eps_derive(input: TokenStream) -> TokenStream {
    derive::nearly_eq(input, derive::DeriveTrait::NearlyEqEps)
}

/// Derives the [NearlyEqUlps] trait for a custom type.
///
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented [NearlyEqUlps].
///
/// # Example
/// ## Same Type
/// If all fields have the same type, the epsilon tolerance will have the same type as the
/// epsilon tolerance of the fields type. For [f32] this would be [i32].
/// ```
/// use nearly::NearlyEqUlps;
/// use nearly::assert_nearly;
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
/// If the fields have different types, the epsilon tolerance will have a tuple type. The tuple will
/// consist of the epsilon types of the fields type in the same order as they are defined.
/// For fields with the type [f32], [f64] and [f32] this would be ([i32], [i64], [i32]).
/// ```
/// use nearly::NearlyEqUlps;
/// use nearly::assert_nearly;
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
///
#[proc_macro_derive(NearlyEqUlps)]
#[proc_macro_error]
pub fn nearly_eq_ulps_derive(input: TokenStream) -> TokenStream {
    derive::nearly_eq(input, derive::DeriveTrait::NearlyEqUlps)
}

/// Derives the all nearly traits for a custom type.
///
/// The derived traits are: [NearlyEqEps], [NearlyEqUlps], [NearlyEqTol] and [NearlyEq].
/// This trait can be derived for structs with named or unnamed fields as well as enums.
/// To derive this trait, all types used for fields have to implemented
/// [NearlyEqEps] and [NearlyEqUlps].
///
/// # Example
/// ## Same Type
/// If all fields have the same type:
///   - the epsilon tolerance will have the same type as the epsilon tolerance of the fields type.
///     For [f32] this would be [f32].
///   - the ulps tolerance will have the same type as the ulps tolerance of the fields type.
///     For [f32] this would be [i32].
/// ```
/// use nearly::NearlyEq;
/// use nearly::assert_nearly;
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
/// If the fields have different types:
///   - the epsilon tolerance will have a tuple type. The tuple will consist of the epsilon types
///     of the fields type in the same order as they are defined.
///     For fields with the type [f32], [f64] and [f32] this would be ([f32], [f64], [f32]).
///   - the ulps tolerance will have a tuple type. The tuple will consist of the ulps types
///     of the fields type in the same order as they are defined.
///     For fields with the type [f32], [f64], [f32] this would be ([i32], [i64], [i32]).
/// ```
/// use nearly::NearlyEq;
/// use nearly::assert_nearly;
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
/// assert_nearly!(a == b, ulps = (8, 12, 8));
/// ```
///
#[proc_macro_derive(NearlyEq)]
#[proc_macro_error]
pub fn nearly_eq_derive(input: TokenStream) -> TokenStream {
    derive::nearly_eq(input, derive::DeriveTrait::NearlyEq)
}
