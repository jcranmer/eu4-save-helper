use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashMap;
use super::stringify;
use syn::{parenthesized, token, Error, Ident, ItemFn, PatType, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

mod kw {
    syn::custom_keyword!(modifier);
}

pub(crate) trait TableEntry : Parse {
    const ENUM_NAME: &'static str;
    fn make_decl(&self) -> TokenStream;
    fn make_match_clause(&self) -> TokenStream;

    fn parse_list(input: ParseStream) -> Result<Vec<Self>>;
    fn scope(&self) -> &Ident;
}

pub(crate) struct Modifier {
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

impl TableEntry for Modifier {
    const ENUM_NAME : &'static str = "Modifier";
    fn scope(&self) -> &Ident {
        &self.scope
    }

    fn parse_list(input: ParseStream) -> Result<Vec<Self>> {
        type ModifierList = Punctuated<Modifier, Token![;]>;
        let raw_list = ModifierList::parse_terminated(input)?;
        Ok(raw_list.into_iter().collect())
    }

    fn make_decl(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        quote! { #name(#ty) }
    }

    fn make_match_clause(&self) -> TokenStream {
        let name = &self.name;
        let stringy_name = stringify(name);
        let ty = &self.ty;
        quote_spanned! { self.paren_token.span =>
            key if key == #stringy_name => {
                let mut parsee : #ty = Default::default();
                parsee.read_from(parser, value)?;
                Ok(Self::#name(parsee))
            }
        }
    }
}

fn get_name(arg: &PatType) -> Option<&Ident> {
    match arg.pat.as_ref() {
        syn::Pat::Ident(p) => Some(&p.ident),
        _ => None
    }
}

pub(crate) struct Effect {
    function: ItemFn,
    scope: Ident,
    arguments: Vec<PatType>,
}

impl Parse for Effect {
    fn parse(input: ParseStream) -> Result<Self> {
        let function : ItemFn = input.parse()?;
        let mut arguments = Vec::new();
        function.sig.inputs.iter().try_for_each(|arg| {
            match arg {
                syn::FnArg::Typed(t) => {
                    arguments.push(t);
                    Ok(())
                },
                _ => Err(Error::new(arg.span(), "self not valid in effect")),
            }
        })?;
        let scope = arguments.last()
            .and_then(|arg| match arg.pat.as_ref() {
                syn::Pat::Ident(p) if p.ident == "scope" => Some(&arg.ty),
                _ => None
            }).and_then(|ty| {
                match &**ty {
                    Type::Path(p) => p.path.get_ident().clone(),
                    _ => None
                }
            }).unwrap().clone();
        arguments.pop();
        Ok(Effect {
            arguments: arguments.iter().map(|&val| val.clone()).collect(),
            function,
            scope,
        })
    }
}

impl Effect {
    fn name(&self) -> &Ident {
        &self.function.sig.ident
    }

    fn simple_ty(&self) -> Option<&Type> {
        match self.arguments.len() {
            1 => Some(&self.arguments[0].ty),
            _ => None
        }
    }
}

impl TableEntry for Effect {
    const ENUM_NAME : &'static str = "Effect";
    fn scope(&self) -> &Ident {
        &self.scope
    }

    fn parse_list(input: ParseStream) -> Result<Vec<Self>> {
        let mut list = Vec::new();
        while !input.is_empty() {
            list.push(Self::parse(input)?);
        }
        Ok(list)
    }

    fn make_decl(&self) -> TokenStream {
        let name = self.name();
        if let Some(ty) = self.simple_ty() {
            quote! { #name(#ty) }
        } else {
            let args = &self.arguments;
            quote! { #name{ #( #args ),* } }
        }
    }

    fn make_match_clause(&self) -> TokenStream {
        let name = self.name();
        let body = if let Some(ty) = self.simple_ty() {
            quote_spanned! { ty.span() =>
                let mut parsee: #ty = Default::default();
                parsee.read_from(parser, value)?;
                Ok(Self::#name(parsee))
            }
        } else {
            let args = &self.arguments;
            let copy_strings = self.arguments.iter().map(|arg| {
                let name = get_name(arg).unwrap();
                quote_spanned! { name.span() => #name: parsee.#name }
            });
            quote_spanned! { name.span() =>
                #[derive(Default, paradox::ParadoxParse)]
                struct StructAnon { #( #args ),* }
                let mut parsee : StructAnon = Default::default();
                parsee.read_from(parser, value)?;
                Ok(Self::#name { #( #copy_strings ),* })
            }
        };
        let stringy_name = stringify(&name);
        quote_spanned! { name.span() =>
            key if key == #stringy_name => {
                #body
            }
        }
    }
}

pub(crate) struct ScopedList<T: TableEntry> {
    by_scopes: HashMap<Ident, Vec<T>>
}

impl <T: TableEntry> Parse for ScopedList<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let raw_list = T::parse_list(input)?;
        let mut by_scopes : HashMap<Ident, Vec<T>> = HashMap::new();
        for entry in raw_list {
            by_scopes.entry(entry.scope().clone()).or_default().push(entry);
        }
        Ok(Self { by_scopes })
    }
}

impl <T: TableEntry> ScopedList<T> {
    pub fn generate_code(&self) -> TokenStream {
        self.by_scopes.keys().map(|scope| {
            let enum_decl = self.generate_enum(scope);
            let try_from = self.generate_try_from(scope);
            quote! { #enum_decl #try_from }
        }).collect()
    }

    fn generate_enum(&self, scope: &Ident) -> TokenStream {
        let modifiers = &self.by_scopes[scope];
        let enum_name = format_ident!("{}{}", scope, T::ENUM_NAME);
        let enum_decls = modifiers.iter()
            .map(T::make_decl);
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub enum #enum_name {
                #( #enum_decls ),*
            }
        }
    }

    fn generate_try_from(&self, scope: &Ident) -> TokenStream {
        let enum_name = format_ident!("{}{}", scope, T::ENUM_NAME);
        let modifiers = &self.by_scopes[scope];
        let match_clauses = modifiers.iter().map(T::make_match_clause);
        quote! {
            impl paradox::FromParadoxKeyPair for #enum_name {
                fn try_from(parser: &mut paradox::Parser, key: &str,
                            value: paradox::UnparsedValue)
                            -> Result<Self, paradox::ParseError> {
                    use paradox::ParadoxParse;
                    match key {
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

pub(crate) type ScopedModifierList = ScopedList<Modifier>;
pub(crate) type ScopedEffectList = ScopedList<Effect>;
