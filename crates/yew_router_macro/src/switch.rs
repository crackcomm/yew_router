use crate::switch::{
    enum_impl::generate_enum_impl, shadow::ShadowMatcherToken, struct_impl::generate_struct_impl,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    export::TokenStream2, parse_macro_input, Data, DeriveInput, Fields, GenericParam, Generics,
    Ident, Variant,
};

mod attribute;
mod enum_impl;
mod shadow;
mod struct_impl;

use self::attribute::AttrToken;
use syn::punctuated::Punctuated;
use yew_router_route_parser::FieldNamingScheme;

/// Holds data that is required to derive Switch for a struct or a single enum variant.
pub struct SwitchItem {
    pub matcher: Vec<ShadowMatcherToken>,
    pub ident: Ident,
    pub fields: Fields,
}

pub fn switch_impl(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident: Ident = input.ident;
    let generics = input.generics;

    match input.data {
        Data::Struct(ds) => {
            let field_naming_scheme = match ds.fields {
                Fields::Unnamed(_) => FieldNamingScheme::Unnamed,
                Fields::Unit => FieldNamingScheme::Unit,
                Fields::Named(_) => FieldNamingScheme::Named,
            };
            let matcher = AttrToken::convert_attributes_to_tokens(input.attrs)
                .into_iter()
                .enumerate()
                .map(|(index, at)| at.into_shadow_matcher_tokens(index, field_naming_scheme))
                .flatten()
                .collect::<Vec<_>>();

            let switch_item = SwitchItem {
                matcher,
                ident,
                fields: ds.fields,
            };
            generate_struct_impl(switch_item, generics)
        }
        Data::Enum(de) => {
            let switch_variants = de
                .variants
                .into_iter()
                .map(|variant: Variant| {
                    let field_type = match variant.fields {
                        Fields::Unnamed(_) => yew_router_route_parser::FieldNamingScheme::Unnamed,
                        Fields::Unit => FieldNamingScheme::Unit,
                        Fields::Named(_) => yew_router_route_parser::FieldNamingScheme::Named,
                    };
                    let matcher = AttrToken::convert_attributes_to_tokens(variant.attrs)
                        .into_iter()
                        .enumerate()
                        .map(|(index, at)| at.into_shadow_matcher_tokens(index, field_type))
                        .flatten()
                        .collect::<Vec<_>>();
                    SwitchItem {
                        matcher,
                        ident: variant.ident,
                        fields: variant.fields,
                    }
                })
                .collect::<Vec<SwitchItem>>();
            generate_enum_impl(ident, switch_variants, generics)
        }
        Data::Union(_du) => panic!("Deriving FromCaptures not supported for Unions."),
    }
}

trait Flatten<T> {
    /// Because flatten is a nightly feature. I'm making a new variant of the function here for
    /// stable use. The naming is changed to avoid this getting clobbered when object_flattening
    /// 60258 is stabilized.
    fn flatten_stable(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten_stable(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}

fn build_matcher_from_tokens(tokens: &[ShadowMatcherToken]) -> TokenStream2 {
    quote! {
        let settings = ::yew_router::matcher::MatcherSettings {
            case_insensitive: true,
        };
        let matcher = ::yew_router::matcher::RouteMatcher {
            tokens: ::std::vec![#(#tokens),*],
            settings
        };
    }
}

/// Creates the "impl <X,Y,Z> ::yew_router::Switch for TypeName<X,Y,Z> where etc.." line.
pub fn impl_line(ident: &Ident, generics: &Generics) -> TokenStream2 {
    if generics.params.is_empty() {
        quote! {
            impl ::yew_router::Switch for #ident
        }
    } else {
        let params = &generics.params;
        let param_idents = params
            .iter()
            .map(|p: &GenericParam| {
                match p {
                    GenericParam::Type(ty) => ty.ident.clone(),
//                    GenericParam::Lifetime(lt) => lt.lifetime, // TODO different type here, must be handled by collecting into a new enum and defining how to convert _that_ to tokens.
                    _ => unimplemented!("Not all type parameter variants (lifetimes and consts) are supported in Switch")
                }
            })
            .collect::<Punctuated<_,syn::token::Comma>>();

        let where_clause = &generics.where_clause;
        quote! {
            impl <#params> ::yew_router::Switch for #ident <#param_idents> #where_clause
        }
    }
}
