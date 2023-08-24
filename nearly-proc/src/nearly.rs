use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, spanned::Spanned, BinOp, Expr, Result,
    Token,
};

pub(crate) enum NearlyMacroType {
    Standard,
    Assert,
    DebugAssert,
}

#[derive(Debug)]
enum NearlyOp {
    Eq,
    Ne,
}

impl NearlyOp {
    fn fn_postfix(&self) -> &str {
        match self {
            NearlyOp::Eq => "_eq",
            NearlyOp::Ne => "_ne",
        }
    }

    fn symbol(&self) -> &str {
        match self {
            NearlyOp::Eq => "==",
            NearlyOp::Ne => "!=",
        }
    }
}

#[derive(Debug)]
enum NearlyTol {
    Default,
    Eps(Expr),
    Ulps(Expr),
    Tol(Expr),
    EpsAndUlps(Expr, Expr),
}

impl NearlyTol {
    fn trait_postfix(&self) -> &str {
        match self {
            NearlyTol::Eps(_) => "Eps",
            NearlyTol::Ulps(_) => "Ulps",
            NearlyTol::EpsAndUlps(_, _) | NearlyTol::Tol(_) => "Tol",
            NearlyTol::Default => "",
        }
    }

    fn fn_postfix(&self) -> &str {
        match self {
            NearlyTol::Eps(_) => "_eps",
            NearlyTol::Ulps(_) => "_ulps",
            NearlyTol::EpsAndUlps(_, _) | NearlyTol::Tol(_) => "_tol",
            NearlyTol::Default => "",
        }
    }
}

#[derive(Debug)]
struct NearlyMacroInput {
    left: Box<Expr>,
    right: Box<Expr>,
    op: NearlyOp,
    tolerance: NearlyTol,
}

impl Parse for NearlyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let syn::ExprBinary {
            attrs: _,
            left,
            op,
            right,
        } = syn::ExprBinary::parse(input)?;

        let op = match op {
            BinOp::Eq(_) => NearlyOp::Eq,
            BinOp::Ne(_) => NearlyOp::Ne,
            _ => {
                return Err(syn::Error::new(op.span(), "invalid comparison operation"));
            }
        };

        let mut tolerance = NearlyTol::Default;
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let ident = syn::Ident::parse(input)?;
            input.parse::<Token![=]>()?;
            let expr = syn::Expr::parse(input)?;

            tolerance = update_tolerance(tolerance, ident, expr)?;
        }

        Ok(NearlyMacroInput {
            left,
            right,
            op,
            tolerance,
        })
    }
}

fn update_tolerance(tol: NearlyTol, ident: Ident, expr: Expr) -> Result<NearlyTol> {
    let updated_tol = match ident.to_string().as_str() {
        "eps" => match tol {
            NearlyTol::Default => NearlyTol::Eps(expr),
            NearlyTol::Ulps(ulps) => NearlyTol::EpsAndUlps(expr, ulps),
            NearlyTol::Eps(_) | NearlyTol::EpsAndUlps(_, _) => {
                return Err(syn::Error::new(ident.span(), "multiple eps arguments"))
            }
            NearlyTol::Tol(_) => {
                return Err(syn::Error::new(
                    ident.span(),
                    "eps argument not allowed alongside tol argument",
                ))
            }
        },
        "ulps" => match tol {
            NearlyTol::Default => NearlyTol::Ulps(expr),
            NearlyTol::Eps(eps) => NearlyTol::EpsAndUlps(eps, expr),
            NearlyTol::Ulps(_) | NearlyTol::EpsAndUlps(_, _) => {
                return Err(syn::Error::new(ident.span(), "multiple ulps arguments"))
            }
            NearlyTol::Tol(_) => {
                return Err(syn::Error::new(
                    ident.span(),
                    "ulps argument not allowed alongside tol argument",
                ))
            }
        },
        "tol" => match tol {
            NearlyTol::Default => NearlyTol::Tol(expr),
            NearlyTol::Tol(_) => {
                return Err(syn::Error::new(ident.span(), "multiple tol arguments"))
            }
            NearlyTol::Eps(_) | NearlyTol::Ulps(_) | NearlyTol::EpsAndUlps(_, _) => {
                return Err(syn::Error::new(
                    ident.span(),
                    "tol argument not allowed alongside eps or ulps argument",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "invalid tolerance identifier",
            ));
        }
    };

    Ok(updated_tol)
}

fn fn_ident(op: &NearlyOp, tolerance: &NearlyTol) -> Ident {
    format_ident!("nearly{}{}", op.fn_postfix(), tolerance.fn_postfix())
}

fn standard_macro_output(input: &NearlyMacroInput) -> proc_macro2::TokenStream {
    let left = &input.left;
    let right = &input.right;
    let fn_ident = fn_ident(&input.op, &input.tolerance);

    match &input.tolerance {
        NearlyTol::Eps(eps) => quote!(#left.#fn_ident(&#right, #eps)),
        NearlyTol::Ulps(ulps) => quote!(#left.#fn_ident(&#right, #ulps)),
        NearlyTol::Tol(tol) => quote!(#left.#fn_ident(&#right, #tol)),
        NearlyTol::EpsAndUlps(eps, ulps) => quote!(#left.#fn_ident(&#right, (#eps, #ulps).into())),
        NearlyTol::Default => quote!(#left.#fn_ident(&#right)),
    }
}

fn assert_macro_output(input: &NearlyMacroInput) -> proc_macro2::TokenStream {
    let left = &input.left;
    let right = &input.right;
    let fn_ident = fn_ident(&input.op, &input.tolerance);
    let op_str = input.op.symbol();

    match &input.tolerance {
        NearlyTol::Eps(eps) => {
            quote!(
                let left = &#left;
                let right = &#right;
                let eps = #eps;
                if !left.#fn_ident(&right, eps) {
                    panic!(
                        r#"assertion failed: `nearly (left {} right)`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`"#,
                        #op_str, left, right, eps
                    )
                }
            )
        }
        NearlyTol::Ulps(ulps) => {
            quote!(
                let left = &#left;
                let right = &#right;
                let ulps = #ulps;
                if !left.#fn_ident(&right, ulps) {
                    panic!(
                        r#"assertion failed: `nearly (left {} right)`
  left: `{:?}`,
 right: `{:?}`,
  ulps: `{:?}`"#,
                        #op_str, left, right, ulps
                    )
                }
            )
        }
        NearlyTol::Tol(tol) => {
            quote!(
                let left = &#left;
                let right = &#right;
                let tol = #tol;
                if !left.#fn_ident(&right, tol) {
                    panic!(
                        r#"assertion failed: `nearly (left {} right)`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`,
  ulps: `{:?}`"#,
                        #op_str, left, right, tol.eps, tol.ulps
                    )
                }
            )
        }
        NearlyTol::EpsAndUlps(eps, ulps) => {
            quote!(
                let left = &#left;
                let right = &#right;
                let eps = #eps;
                let ulps = #ulps;
                if !left.#fn_ident(&right, (eps, ulps).into()) {
                    panic!(
                        r#"assertion failed: `nearly (left {} right)`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`,
  ulps: `{:?}`"#,
                        #op_str, left, right, eps, ulps
                    )
                }
            )
        }
        NearlyTol::Default => {
            quote!(
                let left = &#left;
                let right = &#right;
                if !left.#fn_ident(&right) {
                    use ::nearly::{EpsTolerance, UlpsTolerance};
                    panic!(
                        r#"assertion failed: `nearly (left {} right)`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`,
  ulps: `{:?}`"#,
                        #op_str, left, right, left.default_eps(right), left.default_ulps(right)
                    )
                }
            )
        }
    }
}

pub(crate) fn nearly_macro(input: TokenStream, macro_type: NearlyMacroType) -> TokenStream {
    let nearly_input = parse_macro_input!(input as NearlyMacroInput);
    let trait_ident = format_ident!("NearlyEq{}", nearly_input.tolerance.trait_postfix());

    let output = match macro_type {
        NearlyMacroType::Standard => {
            let macro_output = standard_macro_output(&nearly_input);
            quote!({
                use ::nearly::#trait_ident;
                #macro_output
            })
        }
        NearlyMacroType::Assert => {
            let macro_output = assert_macro_output(&nearly_input);
            quote!({
                use ::nearly::#trait_ident;
                #macro_output
            })
        }
        NearlyMacroType::DebugAssert => {
            let macro_output = assert_macro_output(&nearly_input);
            quote!({
                if cfg!(debug_assertions) {
                    use ::nearly::#trait_ident;
                    #macro_output
                }
            })
        }
    };

    output.into()
}
