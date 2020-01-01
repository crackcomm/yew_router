use crate::switch::{impl_line, SwitchItem};
use proc_macro::TokenStream;
use quote::quote;
use syn::{export::TokenStream2, Field, Fields, Generics, Ident, Type};

pub fn generate_enum_impl(
    enum_ident: Ident,
    switch_variants: Vec<SwitchItem>,
    generics: Generics,
) -> TokenStream {
    let variant_matchers = switch_variants.iter().map(|sv| {
        let SwitchItem {
            matcher,
            ident,
            fields,
        } = sv;
        let build_from_captures = build_variant_from_captures(&enum_ident, ident, fields);
        let matcher = super::build_matcher_from_tokens(&matcher);

        quote! {
            #matcher
            #build_from_captures
        }
    });

    let impl_line = impl_line(&enum_ident, &generics);

    let token_stream = quote! {
        #impl_line
        {
            fn from_path(route: &str) -> ::std::option::Option<Self> {
                #(#variant_matchers)*

                return ::std::option::Option::None
            }
        }
    };
    TokenStream::from(token_stream)
}

/// Once the 'captures' exists, attempt to populate the fields from the list of captures.
fn build_variant_from_captures(
    enum_ident: &Ident,
    variant_ident: &Ident,
    fields: &Fields,
) -> TokenStream2 {
    match fields {
        Fields::Named(named_fields) => {
            let fields: Vec<TokenStream2> = named_fields
                .named
                .iter()
                .filter_map(|field: &Field| {
                    let field_ty: &Type = &field.ty;
                    field.ident.as_ref().map(|i: &Ident| {
                        let key = i.to_string();
                        (i, key, field_ty)
                    })
                })
                .map(|(field_name, key, field_ty): (&Ident, String, &Type)| {
                    quote! {
                        #field_name: {
                            let v = match captures.remove(#key) {
                                ::std::option::Option::Some(value) => {
                                    <#field_ty as ::yew_router_min::Switch>::from_route(value)
                                }
                                ::std::option::Option::None => ::std::option::Option::None,
                            };
                            match v {
                                ::std::option::Option::Some(val) => {
                                    val
                                },
                                ::std::option::Option::None => return None // Failed
                            }
                        }
                    }
                })
                .collect();

            quote! {
                if let ::std::option::Option::Some(mut captures) = matcher.capture_route_into_map(route).ok().map(|x| x.1) {
                    return ::std::option::Option::Some(
                        #enum_ident::#variant_ident {
                            #(#fields),*
                        }
                    );
                };
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            let fields = unnamed_fields.unnamed.iter().map(|f: &Field| {
                let field_ty = &f.ty;
                quote! {
                    {
                        let v = match drain.next() {
                            ::std::option::Option::Some(value) => {
                                <#field_ty as ::yew_router_min::Switch>::from_route(value)
                            },
                            ::std::option::Option::None => ::std::option::Option::None,
                        };
                        match v {
                            ::std::option::Option::Some(val) => val,
                            ::std::option::Option::None => return None // Failed
                        }
                    }
                }
            });

            quote! {
                if let ::std::option::Option::Some(mut captures) = matcher.capture_route_into_vec(route).ok().map(|x| x.1) {
                    let mut drain = captures.drain(..);
                    return ::std::option::Option::Some(
                        #enum_ident::#variant_ident(
                            #(#fields),*
                        )
                    );
                };
            }
        }
        Fields::Unit => {
            quote! {
                if let ::std::option::Option::Some(_captures) = matcher.capture_route_into_map(route).ok().map(|x| x.1) {
                    return ::std::option::Option::Some(#enum_ident::#variant_ident);
                };
            }
        }
    }
}
