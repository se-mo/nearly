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
    NearlyOrdEps,
    NearlyOrdUlps,
    NearlyOrdTol,
    NearlyOrd,
}

enum Cmp {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Cmp {
    fn fn_ident(&self) -> &str {
        match self {
            Cmp::Eq => "eq",
            Cmp::Lt => "lt",
            Cmp::Le => "le",
            Cmp::Gt => "gt",
            Cmp::Ge => "ge",
        }
    }
}

enum TraitFn {
    Eps(Cmp),
    Ulps(Cmp),
    Tol(Cmp),
}

impl TraitFn {
    fn fn_ident(&self) -> Ident {
        match self {
            TraitFn::Eps(cmp) => format_ident!("nearly_{}_eps", cmp.fn_ident()),
            TraitFn::Ulps(cmp) => format_ident!("nearly_{}_ulps", cmp.fn_ident()),
            TraitFn::Tol(cmp) => format_ident!("nearly_{}_tol", cmp.fn_ident()),
        }
    }
}

fn tol_from_trait(derive_trait: DeriveTrait) -> Ident {
    match derive_trait {
        DeriveTrait::NearlyEqEps => format_ident!("EpsTolerance"),
        DeriveTrait::NearlyEqUlps => format_ident!("UlpsTolerance"),
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

fn tol_output(data: &Data, derive_trait: DeriveTrait) -> proc_macro2::TokenStream {
    match data {
        Struct(data_struct) => struct_tol_output(data_struct, derive_trait),
        Enum(data_enum) => enum_tol_output(data_enum, derive_trait),
        Union(_data_union) => abort_call_site!("trait NearlyEqEps cannot be derived for unions"),
    }
}

fn fn_output(data: &Data, ident: &Ident, trait_fn: TraitFn) -> proc_macro2::TokenStream {
    match data {
        Struct(data_struct) => struct_fn_output(data_struct, trait_fn),
        Enum(data_enum) => enum_fn_output(data_enum, ident, trait_fn),
        Union(_data_union) => abort_call_site!("trait NearlyEqEps cannot be derived for unions"),
    }
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
    trait_fn: TraitFn,
) -> proc_macro2::TokenStream {
    match &data_struct.fields {
        Named(fields) => struct_named_fields_fn_output(fields, trait_fn),
        Unnamed(fields) => struct_unnamed_fields_fn_output(fields, trait_fn),
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
    trait_fn: TraitFn,
) -> proc_macro2::TokenStream {
    let idents = idents_from_named_fields(fields);
    let types = types_from_named_fields(fields);
    fn_impl_struct(&types, &idents, trait_fn)
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
    trait_fn: TraitFn,
) -> proc_macro2::TokenStream {
    let indices = indices_from_unnamed_fields(fields);
    let types = types_from_unnamed_fields(fields);
    fn_impl_struct(&types, &indices, trait_fn)
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
    trait_fn: TraitFn,
) -> proc_macro2::TokenStream {
    let variants = unnamed_variants_from_data_enum(data_enum);
    let idents = idents_from_variants(&variants);
    let types = types_from_variants(&variants);
    fn_impl_enum(&types, &idents, enum_ident, trait_fn)
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
    trait_fn: TraitFn,
) -> proc_macro2::TokenStream {
    let func = trait_fn.fn_ident();

    if all_types_equal(types) {
        return match trait_fn {
            TraitFn::Eps(_) => {
                quote!(#(self.#fields.#func(&other.#fields, &eps))&&*)
            }
            TraitFn::Ulps(_) => {
                quote!(#(self.#fields.#func(&other.#fields, &ulps))&&*)
            }
            TraitFn::Tol(_) => {
                quote!(#(self.#fields.#func(&other.#fields, &(tol.eps, tol.ulps).into()))&&*)
            }
        };
    }

    let indices: Vec<Index> = (0..types.len())
        .map(|i| Index {
            index: i as u32,
            span: proc_macro2::Span::call_site(),
        })
        .collect();

    match trait_fn {
        TraitFn::Eps(_) => {
            quote!(#(self.#fields.#func(&other.#fields, &eps.#indices))&&*)
        }
        TraitFn::Ulps(_) => {
            quote!(#(self.#fields.#func(&other.#fields, &ulps.#indices))&&*)
        }
        TraitFn::Tol(_) => {
            quote!(#(self.#fields.#func(&other.#fields, &(tol.eps.#indices, tol.ulps.#indices).into()))&&*)
        }
    }
}

fn fn_impl_enum(
    types: &[&Type],
    idents: &[&Ident],
    enum_ident: &Ident,
    trait_fn: TraitFn,
) -> proc_macro2::TokenStream {
    let func = trait_fn.fn_ident();

    let value_match = if all_types_equal(types) {
        match trait_fn {
            TraitFn::Eps(_) => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, &eps)
                })*)
            }
            TraitFn::Ulps(_) => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, &ulps)
                })*)
            }
            TraitFn::Tol(_) => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, &(tol.eps, tol.ulps).into())
                })*)
            }
        }
    } else {
        let indices: Vec<Index> = (0..types.len())
            .map(|i| Index {
                index: i as u32,
                span: proc_macro2::Span::call_site(),
            })
            .collect();

        match trait_fn {
            TraitFn::Eps(_) => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, &eps.#indices)
                })*)
            }
            TraitFn::Ulps(_) => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, &ulps.#indices)
                })*)
            }
            TraitFn::Tol(_) => {
                quote!(#((#enum_ident::#idents(self_val), #enum_ident::#idents(other_val)) => {
                    self_val.#func(&other_val, &(tol.eps.#indices, tol.ulps.#indices).into())
                })*)
            }
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
    let tol_output = tol_output(data, DeriveTrait::NearlyEqEps);
    let fn_output = fn_output(data, ident, TraitFn::Eps(Cmp::Eq));

    quote!(
        #[automatically_derived]
        impl ::nearly::EpsTolerance for #ident {
            #tol_output
        }

        #[automatically_derived]
        impl ::nearly::NearlyEqEps for #ident {
            fn nearly_eq_eps(&self, other: &Self, eps: &::nearly::EpsToleranceType<Self>) -> bool {
                #fn_output
            }
        }
    )
}

fn derive_nearly_eq_ulps(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let tol_output = tol_output(data, DeriveTrait::NearlyEqUlps);
    let fn_output = fn_output(data, ident, TraitFn::Ulps(Cmp::Eq));

    quote!(
        #[automatically_derived]
        impl ::nearly::UlpsTolerance for #ident {
            #tol_output
        }

        #[automatically_derived]
        impl ::nearly::NearlyEqUlps for #ident {
            fn nearly_eq_ulps(&self, other: &Self, ulps: &::nearly::UlpsToleranceType<Self>) -> bool {
                #fn_output
            }
        }
    )
}

fn derive_nearly_eq_tol(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let eps_output = derive_nearly_eq_eps(data, ident);
    let ulps_output = derive_nearly_eq_ulps(data, ident);
    let fn_output = fn_output(data, ident, TraitFn::Tol(Cmp::Eq));

    quote!(
        #eps_output
        #ulps_output

        #[automatically_derived]
        impl ::nearly::NearlyEqTol for #ident {
            fn nearly_eq_tol(&self, other: &Self, tol: &::nearly::Tolerance<Self>) -> bool {
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

fn derive_nearly_ord_eps(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let lt_fn_output = fn_output(data, ident, TraitFn::Eps(Cmp::Lt));
    let le_fn_output = fn_output(data, ident, TraitFn::Eps(Cmp::Le));
    let gt_fn_output = fn_output(data, ident, TraitFn::Eps(Cmp::Gt));
    let ge_fn_output = fn_output(data, ident, TraitFn::Eps(Cmp::Ge));
    
    quote!(
        #[automatically_derived]
        impl ::nearly::NearlyOrdEps for #ident {
            fn nearly_lt_eps(&self, other: &Self, eps: &::nearly::EpsToleranceType<Self>) -> bool {
                #lt_fn_output
            }
            fn nearly_le_eps(&self, other: &Self, eps: &::nearly::EpsToleranceType<Self>) -> bool {
                #le_fn_output
            }
            fn nearly_gt_eps(&self, other: &Self, eps: &::nearly::EpsToleranceType<Self>) -> bool {
                #gt_fn_output
            }
            fn nearly_ge_eps(&self, other: &Self, eps: &::nearly::EpsToleranceType<Self>) -> bool {
                #ge_fn_output
            }
        }
    )
}

fn derive_nearly_ord_ulps(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let lt_fn_output = fn_output(data, ident, TraitFn::Ulps(Cmp::Lt));
    let le_fn_output = fn_output(data, ident, TraitFn::Ulps(Cmp::Le));
    let gt_fn_output = fn_output(data, ident, TraitFn::Ulps(Cmp::Gt));
    let ge_fn_output = fn_output(data, ident, TraitFn::Ulps(Cmp::Ge));

    quote!(
        #[automatically_derived]
        impl ::nearly::NearlyOrdUlps for #ident {
            fn nearly_lt_ulps(&self, other: &Self, ulps: &::nearly::UlpsToleranceType<Self>) -> bool {
                #lt_fn_output
            }
            fn nearly_le_ulps(&self, other: &Self, ulps: &::nearly::UlpsToleranceType<Self>) -> bool {
                #le_fn_output
            }
            fn nearly_gt_ulps(&self, other: &Self, ulps: &::nearly::UlpsToleranceType<Self>) -> bool {
                #gt_fn_output
            }
            fn nearly_ge_ulps(&self, other: &Self, ulps: &::nearly::UlpsToleranceType<Self>) -> bool {
                #ge_fn_output
            }
        }
    )
}

fn derive_nearly_ord_tol(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let eps_output = derive_nearly_ord_eps(data, ident);
    let ulps_output = derive_nearly_ord_ulps(data, ident);

    let lt_fn_output = fn_output(data, ident, TraitFn::Tol(Cmp::Lt));
    let le_fn_output = fn_output(data, ident, TraitFn::Tol(Cmp::Le));
    let gt_fn_output = fn_output(data, ident, TraitFn::Tol(Cmp::Gt));
    let ge_fn_output = fn_output(data, ident, TraitFn::Tol(Cmp::Ge));

    quote!(
        #eps_output
        #ulps_output

        #[automatically_derived]
        impl ::nearly::NearlyOrdTol for #ident {
            fn nearly_lt_tol(&self, other: &Self, tol: &::nearly::Tolerance<Self>) -> bool {
                #lt_fn_output
            }
            fn nearly_le_tol(&self, other: &Self, tol: &::nearly::Tolerance<Self>) -> bool {
                #le_fn_output
            }
            fn nearly_gt_tol(&self, other: &Self, tol: &::nearly::Tolerance<Self>) -> bool {
                #gt_fn_output
            }
            fn nearly_ge_tol(&self, other: &Self, tol: &::nearly::Tolerance<Self>) -> bool {
                #ge_fn_output
            }
        }
    )
}

fn derive_nearly_ord(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    let tol_output = derive_nearly_ord_tol(data, ident);

    quote!(
        #tol_output

        #[automatically_derived]
        impl ::nearly::NearlyOrd for #ident {}
    )
}

pub(crate) fn nearly_eq(input: TokenStream, derive_trait: DeriveTrait) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let result: proc_macro2::TokenStream = match derive_trait {
        DeriveTrait::NearlyEqEps => derive_nearly_eq_eps(&data, &ident),
        DeriveTrait::NearlyEqUlps => derive_nearly_eq_ulps(&data, &ident),
        DeriveTrait::NearlyEqTol => derive_nearly_eq_tol(&data, &ident),
        DeriveTrait::NearlyEq => derive_nearly_eq(&data, &ident),
        DeriveTrait::NearlyOrdEps => derive_nearly_ord_eps(&data, &ident),
        DeriveTrait::NearlyOrdUlps => derive_nearly_ord_ulps(&data, &ident),
        DeriveTrait::NearlyOrdTol => derive_nearly_ord_tol(&data, &ident),
        DeriveTrait::NearlyOrd => derive_nearly_ord(&data, &ident),
    };

    result.into()
}
