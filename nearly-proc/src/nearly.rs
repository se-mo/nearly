use proc_macro::TokenStream;
use proc_macro2::{Ident, Punct, Spacing};
use quote::{format_ident, quote, ToTokens};
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

#[derive(Debug)]
struct NearlyTol {
    ident: Ident,
    expr: Expr,
}

impl ToTokens for NearlyTol {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        Punct::new(',', Spacing::Alone).to_tokens(tokens);
        self.ident.to_tokens(tokens);
        Punct::new('=', Spacing::Alone).to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

#[derive(Debug)]
struct NearlyMacroInput {
    left: Box<Expr>,
    right: Box<Expr>,
    op: NearlyOp,
    tol: Vec<NearlyTol>,
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

        let mut tol = Vec::<NearlyTol>::new();
        for _ in 0..2 {
            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
            let ident = syn::Ident::parse(input)?;
            match ident.to_string().as_str() {
                "eps" | "ulps" | "tol" => (),
                _ => {
                    return Err(syn::Error::new(ident.span(), "invalid identifier"));
                }
            };

            input.parse::<Token![=]>()?;
            let expr = syn::Expr::parse(input)?;
            tol.push(NearlyTol { ident, expr });
        }

        Ok(NearlyMacroInput {
            left,
            right,
            op,
            tol,
        })
    }
}

pub(crate) fn nearly_macro(input: TokenStream, macro_type: NearlyMacroType) -> TokenStream {
    let NearlyMacroInput {
        left,
        right,
        op,
        tol,
    } = parse_macro_input!(input as NearlyMacroInput);

    let macro_prefix = match macro_type {
        NearlyMacroType::Standard => "",
        NearlyMacroType::Assert => "assert_",
        NearlyMacroType::DebugAssert => "debug_assert_",
    };
    let macro_postfix = match op {
        NearlyOp::Eq => "_eq",
        NearlyOp::Ne => "_ne",
    };

    let macro_ident = format_ident!("{}nearly{}", macro_prefix, macro_postfix);

    let mut tol_stream = proc_macro2::TokenStream::new();
    tol.iter().for_each(|t| t.to_tokens(&mut tol_stream));

    quote!({
        use nearly::#macro_ident;
        #macro_ident!(#left, #right #tol_stream)
    })
    .into()
}
