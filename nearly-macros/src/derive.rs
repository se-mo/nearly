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
    NearlyEqTol,
    NearlyEq,
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
        DeriveTrait::NearlyEqTol => format_ident!("nearly_eq_tol"),
        _ => panic!("invalid derive trait"),
    }
}

fn types_from_named_fields(fields: &FieldsNamed) -> Vec<&Type> {
    fields.named.iter().map(|it| &it.ty).collect()
}

fn types_from_unnamed_fields(fields: &FieldsUnnamed) -> Vec<&Type> {
    fields.unnamed.iter().map(|it| &it.ty).collect()
}

fn idents_from_named_fields(fields: &FieldsNamed) -> Vec<&Ident> {
    fields
        .named
        .iter()
        .map(|it| match &it.ident {
            Some(ident) => ident,
            None => abort!(it, "Invalid ident of named struct field"),
        })
        .collect()
}

fn indices_from_unnamed_fields(fields: &FieldsUnnamed) -> Vec<Index> {
    let num_fields = fields.unnamed.iter().count();
    (0..num_fields)
        .map(|i| Index {
            index: i as u32,
            span: proc_macro2::Span::call_site(),
        })
        .collect()
}

fn struct_tol_output(
    data_struct: &DataStruct,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    match &data_struct.fields {
        Named(fields) => struct_named_fields_tol_output(fields, derive_trait),
        Unnamed(fields) => struct_unnamed_fields_tol_output(fields, derive_trait),
        Unit => struct_unit_tol_output(),
    }
}

fn struct_fn_output(
    data_struct: &DataStruct,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    match &data_struct.fields {
        Named(fields) => struct_named_fields_fn_output(fields, derive_trait),
        Unnamed(fields) => struct_unnamed_fields_fn_output(fields, derive_trait),
        Unit => struct_unit_fn_output(),
    }
}

fn struct_named_fields_tol_output(
    fields: &FieldsNamed,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let types = types_from_named_fields(fields);
    tol_impl(&types, derive_trait)
}

fn struct_named_fields_fn_output(
    fields: &FieldsNamed,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let idents = idents_from_named_fields(fields);
    let types = types_from_named_fields(fields);
    fn_impl_struct(&types, &idents, derive_trait)
}

fn struct_unnamed_fields_tol_output(
    fields: &FieldsUnnamed,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let types = types_from_unnamed_fields(fields);
    tol_impl(&types, derive_trait)
}

fn struct_unnamed_fields_fn_output(
    fields: &FieldsUnnamed,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let indices = indices_from_unnamed_fields(fields);
    let types = types_from_unnamed_fields(fields);
    fn_impl_struct(&types, &indices, derive_trait)
}

fn struct_unit_tol_output() -> proc_macro2::TokenStream {
    quote!(
        type T = ();
        const DEFAULT: Self::T = ();
    )
}

fn struct_unit_fn_output() -> proc_macro2::TokenStream {
    quote!(true)
}

fn unnamed_variants_from_data_enum(data_enum: &DataEnum) -> Vec<&Variant> {
    data_enum
        .variants
        .iter()
        .filter(|it| matches!(it.fields, Unnamed(_)))
        .collect()
}

fn idents_from_variants<'a>(variants: &'a [&Variant]) -> Vec<&'a Ident> {
    variants.iter().map(|it| &it.ident).collect()
}

fn types_from_variants<'a>(variants: &'a [&Variant]) -> Vec<&'a Type> {
    variants
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
        .collect()
}

fn enum_tol_output(data_enum: &DataEnum, derive_trait: DeriveTrait) -> proc_macro2::TokenStream {
    let variants = unnamed_variants_from_data_enum(data_enum);
    let types = types_from_variants(&variants);
    tol_impl(&types, derive_trait)
}

fn enum_fn_output(
    data_enum: &DataEnum,
    enum_ident: &Ident,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let variants = unnamed_variants_from_data_enum(data_enum);
    let idents = idents_from_variants(&variants);
    let types = types_from_variants(&variants);
    fn_impl_enum(&types, &idents, enum_ident, derive_trait)
}

fn all_types_equal(types: &[&Type]) -> bool {
    types.windows(2).all(|x| x[0] == x[1])
}

fn tol_impl(types: &[&Type], derive_trait: DeriveTrait) -> proc_macro2::TokenStream {
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

fn fn_impl_struct<T: ToTokens>(
    types: &[&Type],
    fields: &[T],
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let func = fn_from_trait(derive_trait);

    if all_types_equal(types) {
        return match derive_trait {
            DeriveTrait::NearlyEqEps => {
                quote!(#(self.#fields.#func(&other.#fields, eps))&&*)
            }
            DeriveTrait::NearlyEqUlps => {
                quote!(#(self.#fields.#func(&other.#fields, ulps))&&*)
            }
            DeriveTrait::NearlyEqTol => {
                quote!(#(self.#fields.#func(&other.#fields, (tolerance.eps, tolerance.ulps).into()))&&*)
            }
            DeriveTrait::NearlyEq => panic!("invalid derive trait"),
        };
    }

    let indices: Vec<Index> = (0..types.len())
        .map(|i| Index {
            index: i as u32,
            span: proc_macro2::Span::call_site(),
        })
        .collect();

    match derive_trait {
        DeriveTrait::NearlyEqEps => {
            quote!(#(self.#fields.#func(&other.#fields, eps.#indices))&&*)
        }
        DeriveTrait::NearlyEqUlps => {
            quote!(#(self.#fields.#func(&other.#fields, ulps.#indices))&&*)
        }
        DeriveTrait::NearlyEqTol => {
            quote!(#(self.#fields.#func(&other.#fields, (tolerance.eps.#indices, tolerance.ulps.#indices).into()))&&*)
        }
        DeriveTrait::NearlyEq => panic!("invalid derive trait"),
    }
}

fn fn_impl_enum(
    types: &[&Type],
    idents: &[&Ident],
    enum_ident: &Ident,
    derive_trait: DeriveTrait,
) -> proc_macro2::TokenStream {
    let func = fn_from_trait(derive_trait);

    let value_match = if all_types_equal(types) {
        match derive_trait {
            DeriveTrait::NearlyEqEps => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, eps)
                })*)
            }
            DeriveTrait::NearlyEqUlps => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, ulps)
                })*)
            }
            DeriveTrait::NearlyEqTol => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, (tolerance.eps, tolerance.ulps).into())
                })*)
            }
            DeriveTrait::NearlyEq => panic!("invalid derive trait"),
        }
    } else {
        let indices: Vec<Index> = (0..types.len())
            .map(|i| Index {
                index: i as u32,
                span: proc_macro2::Span::call_site(),
            })
            .collect();

        match derive_trait {
            DeriveTrait::NearlyEqEps => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, eps.#indices)
                })*)
            }
            DeriveTrait::NearlyEqUlps => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, ulps.#indices)
                })*)
            }
            DeriveTrait::NearlyEqTol => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, (tolerance.eps.#indices, tolerance.ulps.#indices).into())
                })*)
            }
            DeriveTrait::NearlyEq => panic!("invalid derive trait"),
        }
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
    let tol_output = match data {
        Struct(data_struct) => struct_tol_output(data_struct, DeriveTrait::NearlyEqEps),
        Enum(data_enum) => enum_tol_output(data_enum, DeriveTrait::NearlyEqEps),
        Union(_data_union) => abort!(ident, "trait NearlyEqEps cannot be derived for unions"),
    };
    let fn_output = match data {
        Struct(data_struct) => struct_fn_output(data_struct, DeriveTrait::NearlyEqEps),
        Enum(data_enum) => enum_fn_output(data_enum, ident, DeriveTrait::NearlyEqEps),
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
    let tol_output = match data {
        Struct(data_struct) => struct_tol_output(data_struct, DeriveTrait::NearlyEqUlps),
        Enum(data_enum) => enum_tol_output(data_enum, DeriveTrait::NearlyEqUlps),
        Union(_data_union) => abort!(ident, "trait NearlyEqUlps cannot be derived for unions"),
    };
    let fn_output = match data {
        Struct(data_struct) => struct_fn_output(data_struct, DeriveTrait::NearlyEqUlps),
        Enum(data_enum) => enum_fn_output(data_enum, ident, DeriveTrait::NearlyEqUlps),
        Union(_data_union) => abort!(ident, "trait NearlyEqUlps cannot be derived for unions"),
    };

    quote!(
        #[automatically_derived]
        impl ::nearly::UlpsTolerance for #ident {
            #tol_output
        }

        #[automatically_derived]
        impl ::nearly::NearlyEqUlps for #ident {
            fn nearly_eq_ulps(&self, other: &Self, ulps: ::nearly::UlpsToleranceType<Self>) -> bool {
                #fn_output
            }
        }
    )
}

fn derive_nearly_eq_tol(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let eps_output = derive_nearly_eq_eps(data, ident);
    let ulps_output = derive_nearly_eq_ulps(data, ident);

    let fn_output = match data {
        Struct(data_struct) => struct_fn_output(data_struct, DeriveTrait::NearlyEqTol),
        Enum(data_enum) => enum_fn_output(data_enum, ident, DeriveTrait::NearlyEqTol),
        Union(_data_union) => abort!(ident, "trait NearlyEqTol cannot be derived for unions"),
    };

    quote!(
        #eps_output
        #ulps_output
        #[automatically_derived]
        impl ::nearly::NearlyEqTol for #ident {
            fn nearly_eq_tol(&self, other: &Self, tolerance: ::nearly::Tolerance<Self>) -> bool {
                #fn_output
            }
        }
    )
}

fn derive_nearly_eq(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let tol_output = derive_nearly_eq_tol(data, ident);

    quote!(
        #tol_output
        #[automatically_derived]
        impl ::nearly::NearlyEq for #ident {}
    )
}

pub(crate) fn nearly_eq(input: TokenStream, derive_trait: DeriveTrait) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let result: proc_macro2::TokenStream = match derive_trait {
        DeriveTrait::NearlyEqEps => derive_nearly_eq_eps(&data, &ident),
        DeriveTrait::NearlyEqUlps => derive_nearly_eq_ulps(&data, &ident),
        DeriveTrait::NearlyEqTol => derive_nearly_eq_tol(&data, &ident),
        DeriveTrait::NearlyEq => derive_nearly_eq(&data, &ident),
    };

    result.into()
}
