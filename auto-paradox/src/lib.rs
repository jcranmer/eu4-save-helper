extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Field, Type};

#[derive(Default)]
struct ScopeFieldHandling {
    name_check: TokenStream2,
    body: TokenStream2,
    scope: bool
}

fn has_tag(field: &Field, tag: &'static str) -> bool {
    field.attrs.iter()
        .any(|attr| attr.path.is_ident(tag))
}

fn handle_field(field: &Field) -> ScopeFieldHandling {
    // Ignore fields marked #[ignore]
    if has_tag(field, "ignore") {
        return Default::default();
    }

    let name = &field.ident;
    let ty = match &field.ty {
        &Type::Path(ref p) => p.path.clone().into_token_stream().to_string(),
        _ => unimplemented!()
    };

    if has_tag(field, "collect_scopes") {
        let name_check = TokenStream2::new();
        let scope = true;
        let body = quote_spanned!{field.span()=>
            &mut self.#name.push(Default::default());
            let mut field = self.#name.last_mut().unwrap();
            field.name = id.into();
            field
        };
        return ScopeFieldHandling { name_check, body, scope };
    }

    let name_check = quote_spanned!{field.span()=> id.is(stringify!(#name))};
    let scope = ty.starts_with("HashMap") || ty.starts_with("Vec");
    let body = if scope {
        quote_spanned!{field.span() => &mut self.#name}
    } else {
        quote_spanned!{field.span() => self.#name = value.into()}
    };
    ScopeFieldHandling { name_check, body, scope }
}

#[proc_macro_derive(ParadoxScope, attributes(collect_scopes, ignore))]
pub fn derive_paradox_scope(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let body_map : Vec<_> = match input.data {
        Data::Struct(ref data) =>
            data.fields.iter().map(handle_field).collect(),
        _ => unimplemented!()
    };
    let mut default_body = quote!{
        println!(concat!(stringify!(#name), "/{:?}"), id);
    };
    let mut default_scoped_body = quote!{
        println!(concat!(stringify!(#name), "/{:?} (scope)"), id);
        crate::lexer::NullScope::instance()
    };
    let mut scoped_statements = Vec::new();
    let mut leaf_statements = Vec::new();
    for field in body_map {
        if field.body.is_empty() {
            continue;
        }

        let (default, list) = if field.scope {
            (&mut default_scoped_body, &mut scoped_statements)
        } else {
            (&mut default_body, &mut leaf_statements)
        };

        if field.name_check.is_empty() {
            *default = field.body;
        } else {
            let name_check = field.name_check;
            let body = field.body;
            list.push(quote!{
                if #name_check {
                    #body
                }
            });
        }
    }

    let expanded = quote! {
        impl crate::lexer::ParadoxScope for #name {
            fn start_scope(&mut self, id: crate::lexer::Token) ->
                    &mut crate::lexer::ParadoxScope {
                #( #scoped_statements else )* {
                    #default_scoped_body
                }
            }

            fn set_property(&mut self, id: Option<crate::lexer::Token>,
                            value: crate::lexer::Token) {
                if let Some(id) = id {
                    #( #leaf_statements else )* {
                        #default_body
                    }
                } else {
                    #default_body
                }
            }
        }
    };

    TokenStream::from(expanded)
}
