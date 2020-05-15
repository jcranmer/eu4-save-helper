use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashMap;
use syn::{parenthesized, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(effect);
}

struct Effect {
    _effect_token: kw::effect,
    paren_token: token::Paren,
    scope: Ident,
    _comma1_token: Token![,],
    name: Ident,
    _comma2_token: Token![,],
    ty: Type
}

impl Parse for Effect {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Effect {
            _effect_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            scope: content.parse()?,
            _comma1_token: content.parse()?,
            name: content.parse()?,
            _comma2_token: content.parse()?,
            ty: content.parse()?
        })
    }
}

impl Effect {
    fn make_decl(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        quote! { #name(#ty) }
    }

    fn make_match_clause(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        quote_spanned! { self.paren_token.span =>
            key if key == stringify!(#name) => {
                let mut parsee : #ty = Default::default();
                parsee.read_from(value)?;
                Ok(Self::#name(parsee))
            }
        }
    }
}

pub struct ScopedEffectList {
    by_scopes: HashMap<Ident, Vec<Effect>>
}

impl Parse for ScopedEffectList {
    fn parse(input: ParseStream) -> Result<Self> {
        type EffectList = Punctuated<Effect, Token![;]>;
        let raw_list = EffectList::parse_terminated(input)?;
        let mut by_scopes : HashMap<Ident, Vec<Effect>> = HashMap::new();
        for effect in raw_list {
            by_scopes.entry(effect.scope.clone()).or_default().push(effect);
        }
        Ok(ScopedEffectList { by_scopes })
    }
}

impl ScopedEffectList {
    pub fn generate_code(&self) -> TokenStream {
        self.by_scopes.keys().map(|scope| {
            let enum_decl = self.generate_enum(scope);
            let try_from = self.generate_try_from(scope);
            quote! { #enum_decl #try_from }
        }).collect()
    }

    fn generate_enum(&self, scope: &Ident) -> TokenStream {
        let effects = &self.by_scopes[scope];
        let enum_name = format_ident!("{}{}", scope, "Effect");
        let enum_decls = effects.iter()
            .map(Effect::make_decl);
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub enum #enum_name {
                #( #enum_decls ),*
            }
        }
    }

    fn generate_try_from(&self, scope: &Ident) -> TokenStream {
        let enum_name = format_ident!("{}{}", scope, "Effect");
        let from_ty = quote! { (String, paradox::UnparsedValue<'_>) };
        let effects = &self.by_scopes[scope];
        let match_clauses = effects.iter().map(Effect::make_match_clause);
        quote! {
            impl std::convert::TryFrom<#from_ty> for #enum_name {
                type Error = paradox::ParseError;

                fn try_from(v: #from_ty) -> Result<Self, Self::Error> {
                    use paradox::ParadoxParse;
                    let value = v.1;
                    match &v.0 {
                        #( #match_clauses ),*
                        key => {
                            value.validation_error(stringify!(#enum_name),
                                key,
                                "no definition found", true)?;
                            panic!("Should not reach this point");
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;
    #[test]
    fn test_effect() {
        let effect: ScopedEffectList = parse_quote! { effect(Country, field, u32); effect(Country, a, i32);};
        println!("{}", effect.generate_code());
        assert!(false);
    }
}
