use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashMap;
use syn::{parenthesized, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(modifier);
}

struct Modifier {
    _modifier_token: kw::modifier,
    paren_token: token::Paren,
    scope: Ident,
    _comma1_token: Token![,],
    name: Ident,
    _comma2_token: Token![,],
    ty: Type
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Modifier {
            _modifier_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            scope: content.parse()?,
            _comma1_token: content.parse()?,
            name: content.parse()?,
            _comma2_token: content.parse()?,
            ty: content.parse()?
        })
    }
}

impl Modifier {
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

pub struct ScopedModifierList {
    by_scopes: HashMap<Ident, Vec<Modifier>>
}

impl Parse for ScopedModifierList {
    fn parse(input: ParseStream) -> Result<Self> {
        type ModifierList = Punctuated<Modifier, Token![;]>;
        let raw_list = ModifierList::parse_terminated(input)?;
        let mut by_scopes : HashMap<Ident, Vec<Modifier>> = HashMap::new();
        for modifier in raw_list {
            by_scopes.entry(modifier.scope.clone()).or_default().push(modifier);
        }
        Ok(ScopedModifierList { by_scopes })
    }
}

impl ScopedModifierList {
    pub fn generate_code(&self) -> TokenStream {
        self.by_scopes.keys().map(|scope| {
            let enum_decl = self.generate_enum(scope);
            let try_from = self.generate_try_from(scope);
            quote! { #enum_decl #try_from }
        }).collect()
    }

    fn generate_enum(&self, scope: &Ident) -> TokenStream {
        let modifiers = &self.by_scopes[scope];
        let enum_name = format_ident!("{}{}", scope, "Modifier");
        let enum_decls = modifiers.iter()
            .map(Modifier::make_decl);
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub enum #enum_name {
                #( #enum_decls ),*
            }
        }
    }

    fn generate_try_from(&self, scope: &Ident) -> TokenStream {
        let enum_name = format_ident!("{}{}", scope, "Modifier");
        let from_ty = quote! { (String, paradox::UnparsedValue<'_>) };
        let modifiers = &self.by_scopes[scope];
        let match_clauses = modifiers.iter().map(Modifier::make_match_clause);
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
    fn test_modifier() {
        let modifier: ScopedModifierList = parse_quote! { modifier(Country, field, u32); modifier(Country, a, i32);};
        println!("{}", modifier.generate_code());
        assert!(false);
    }
}
