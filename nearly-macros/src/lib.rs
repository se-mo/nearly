//! This crate provides procedural macros for the
//! [nearly](https://docs.rs/nearly/latest/nearly/) crate.

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod derive;
mod nearly;

#[proc_macro]
pub fn nearly(input: TokenStream) -> TokenStream {
    nearly::nearly_macro(input, nearly::NearlyMacroType::Standard)
}

#[proc_macro]
pub fn assert_nearly(input: TokenStream) -> TokenStream {
    nearly::nearly_macro(input, nearly::NearlyMacroType::Assert)
}

#[proc_macro]
pub fn debug_assert_nearly(input: TokenStream) -> TokenStream {
    nearly::nearly_macro(input, nearly::NearlyMacroType::DebugAssert)
}

#[proc_macro_derive(NearlyEqEps)]
#[proc_macro_error]
pub fn nearly_eq_eps_derive(input: TokenStream) -> TokenStream {
    derive::nearly_eq(input, derive::DeriveTrait::NearlyEqEps)
}

#[proc_macro_derive(NearlyEqUlps)]
#[proc_macro_error]
pub fn nearly_eq_ulps_derive(input: TokenStream) -> TokenStream {
    derive::nearly_eq(input, derive::DeriveTrait::NearlyEqUlps)
}

#[proc_macro_derive(NearlyEqTol)]
#[proc_macro_error]
pub fn nearly_eq_tol_derive(input: TokenStream) -> TokenStream {
    derive::nearly_eq(input, derive::DeriveTrait::NearlyEqTol)
}

#[proc_macro_derive(NearlyEq)]
#[proc_macro_error]
pub fn nearly_eq_derive(input: TokenStream) -> TokenStream {
    derive::nearly_eq(input, derive::DeriveTrait::NearlyEq)
}
