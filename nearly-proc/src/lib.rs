//! This crate provides proc macros for the [nearly](https://crates.io/crates/nearly) crate.

use proc_macro::TokenStream;

mod nearly;

/// Returns whether the given comparison is nearly true using the provided tolerance.
///
/// Comparison can be equal (`==`) or unequal (`!=`).
///
/// # Examples
/// Comparison can be based on different tolerances:
/// ```ignore
/// use nearly::nearly;
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
    nearly::nearly_macro(input)
}
