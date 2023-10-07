use proc_macro::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input,
    Data::{self, Enum, Struct, Union},
    DataEnum, DataStruct, DeriveInput,
    Fields::{Named, Unit, Unnamed},
    FieldsNamed, FieldsUnnamed, Ident, Index, Type, Variant,
};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum DeriveTrait {
    NearlyEqEps,
    NearlyEqUlps,
    NearlyEq,
}

struct NearlyOutput {
    tol_output: proc_macro2::TokenStream,
    fn_output: proc_macro2::TokenStream,
}

fn tol_from_trait(derive_trait: DeriveTrait) -> Ident {
    match derive_trait {
        DeriveTrait::NearlyEqEps => format_ident!("EpsTolerance"),
        DeriveTrait::NearlyEqUlps => format_ident!("UlpsTolerance"),
        _ => panic!("invalid derive trait"),
    }
}

fn fn_from_trait(derive_trait: DeriveTrait) -> Ident {
    match derive_trait {
        DeriveTrait::NearlyEqEps => format_ident!("nearly_eq_eps"),
        DeriveTrait::NearlyEqUlps => format_ident!("nearly_eq_ulps"),
        _ => panic!("invalid derive trait"),
    }
}

fn struct_output(data_struct: &DataStruct, derive_trait: DeriveTrait) -> NearlyOutput {
    match &data_struct.fields {
        Named(fields) => struct_named_fields_output(fields, derive_trait),
        Unnamed(fields) => struct_unnamed_fields_output(fields, derive_trait),
        Unit => struct_unit_output(),
    }
}

fn struct_named_fields_output(fields: &FieldsNamed, derive_trait: DeriveTrait) -> NearlyOutput {
    let idents: Vec<&Ident> = fields
        .named
        .iter()
        .map(|it| match &it.ident {
            Some(ident) => ident,
            None => abort!(it, "Invalid ident of named struct field"),
        })
        .collect();
    let types: Vec<&Type> = fields.named.iter().map(|it| &it.ty).collect();

    let tol_output = tol_output(&types, derive_trait);
    let fn_output = fn_output_struct(&types, &idents, derive_trait);

    NearlyOutput {
        tol_output,
        fn_output,
    }
}

fn struct_unnamed_fields_output(fields: &FieldsUnnamed, derive_trait: DeriveTrait) -> NearlyOutput {
    let num_fields = fields.unnamed.iter().count();
    let indices: Vec<Index> = (0..num_fields)
        .map(|i| Index {
            index: i as u32,
            span: proc_macro2::Span::call_site(),
        })
        .collect();
    let types: Vec<&Type> = fields.unnamed.iter().map(|it| &it.ty).collect();

    let tol_output = tol_output(&types, derive_trait);
    let fn_output = fn_output_struct(&types, &indices, derive_trait);

    NearlyOutput {
        tol_output,
        fn_output,
    }
}

fn struct_unit_output() -> NearlyOutput {
    let tol_output = quote!(
        type T = ();
        const DEFAULT: Self::T = ();
    );
    let fn_output = quote!(true);

    NearlyOutput {
        tol_output,
        fn_output,
    }
}

fn enum_output(
    data_enum: &DataEnum,
    enum_ident: &Ident,
    derive_trait: DeriveTrait,
) -> NearlyOutput {
    let filtered_variants: Vec<&Variant> = data_enum
        .variants
        .iter()
        .filter(|it| matches!(it.fields, Unnamed(_)))
        .collect();
    let idents: Vec<&Ident> = filtered_variants.iter().map(|it| &it.ident).collect();
    let types: Vec<&Type> = filtered_variants
        .iter()
        .map(|it| match &it.fields {
            Unnamed(f) => {
                if let Some(first_field) = f.unnamed.first() {
                    return &first_field.ty;
                }
                abort!(f.unnamed, "Invalid ident of unnamed enum field")
            }
            _ => abort!(it, "invalid enum field"),
        })
        .collect();

    let tol_output = tol_output(&types, derive_trait);
    let fn_output = fn_output_enum(&types, &idents, enum_ident, derive_trait);

    NearlyOutput {
        tol_output,
        fn_output,
    }
}

fn all_types_equal(types: &[&Type]) -> bool {
    types.windows(2).all(|x| x[0] == x[1])
}

fn tol_output(types: &[&Type], derive_trait: DeriveTrait) -> proc_macro2::TokenStream {
    let tol = tol_from_trait(derive_trait);

    if all_types_equal(types) {
        let ty = types.first();
        if ty.is_none() {
            abort_call_site!("At least one element must have non unit type");
        }
        return quote!(
            type T = <#ty as ::nearly::#tol>::T;
            const DEFAULT: Self::T = <#ty as ::nearly::#tol>::DEFAULT;
        );
    }

    quote!(
        type T = (#(<#types as ::nearly::#tol>::T),*);
        const DEFAULT: Self::T = (#(<#types as ::nearly::#tol>::DEFAULT),*);
    )
}

fn fn_output_struct<T: ToTokens>(
    types: &[&Type],
    fields: &[T],
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let func = fn_from_trait(derive_trait);

    if all_types_equal(types) {
        return quote!(#(self.#fields.#func(&other.#fields, eps))&&*);
    }

    let indices: Vec<Index> = (0..types.len())
        .map(|i| Index {
            index: i as u32,
            span: proc_macro2::Span::call_site(),
        })
        .collect();
    quote!(#(self.#fields.#func(&other.#fields, eps.#indices))&&*)
}

fn fn_output_enum(
    types: &[&Type],
    idents: &[&Ident],
    enum_ident: &Ident,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let func = fn_from_trait(derive_trait);

    let value_match = if all_types_equal(types) {
        quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) =>
            {self_val.#func(&other_val, eps)})*)
    } else {
        let indices: Vec<Index> = (0..types.len())
            .map(|i| Index {
                index: i as u32,
                span: proc_macro2::Span::call_site(),
            })
            .collect();
        quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) =>
            {self_val.#func(&other_val, eps.#indices)})*)
    };

    quote!(
        let self_tag = core::mem::discriminant(self);
        let other_tag = core::mem::discriminant(other);

        self_tag == other_tag &&
        match (self, other) {
            #value_match
            _ => true,
        }
    )
}

fn derive_nearly_eq_eps(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let NearlyOutput {
        tol_output,
        fn_output,
    } = match data {
        Struct(data_struct) => struct_output(data_struct, DeriveTrait::NearlyEqEps),
        Enum(data_enum) => enum_output(data_enum, ident, DeriveTrait::NearlyEqEps),
        Union(_data_union) => abort!(ident, "trait NearlyEqEps cannot be derived for unions"),
    };

    quote!(
        #[automatically_derived]
        impl ::nearly::EpsTolerance for #ident {
            #tol_output
        }

        #[automatically_derived]
        impl ::nearly::NearlyEqEps for #ident {
            fn nearly_eq_eps(&self, other: &Self, eps: ::nearly::EpsToleranceType<Self>) -> bool {
                #fn_output
            }
        }
    )
}

fn derive_nearly_eq_ulps(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let NearlyOutput {
        tol_output,
        fn_output,
    } = match data {
        Struct(data_struct) => struct_output(data_struct, DeriveTrait::NearlyEqUlps),
        Enum(data_enum) => enum_output(data_enum, ident, DeriveTrait::NearlyEqUlps),
        Union(_data_union) => abort!(ident, "trait NearlyEqUlps cannot be derived for unions"),
    };

    quote!(
        #[automatically_derived]
        impl ::nearly::UlpsTolerance for #ident {
            #tol_output
        }

        #[automatically_derived]
        impl ::nearly::NearlyEqUlps for #ident {
            fn nearly_eq_ulps(&self, other: &Self, eps: ::nearly::UlpsToleranceType<Self>) -> bool {
                #fn_output
            }
        }
    )
}

fn derive_nearly_eq(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let eps_output = derive_nearly_eq_eps(data, ident);
    let ulps_output = derive_nearly_eq_ulps(data, ident);

    quote!(
        #eps_output
        #ulps_output
        #[automatically_derived]
        impl ::nearly::NearlyEqTol for #ident {}
        #[automatically_derived]
        impl ::nearly::NearlyEq for #ident {}
    )
}

pub(crate) fn nearly_eq(input: TokenStream, derive_trait: DeriveTrait) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let result: proc_macro2::TokenStream = match derive_trait {
        DeriveTrait::NearlyEqEps => derive_nearly_eq_eps(&data, &ident),
        DeriveTrait::NearlyEqUlps => derive_nearly_eq_ulps(&data, &ident),
        DeriveTrait::NearlyEq => derive_nearly_eq(&data, &ident),
    };

    result.into()
}
