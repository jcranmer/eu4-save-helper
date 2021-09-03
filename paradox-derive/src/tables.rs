use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use super::{Name, stringify};
use syn::{parenthesized, token, Error, Ident, ItemFn, PatType, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

mod kw {
    syn::custom_keyword!(modifier);
    syn::custom_keyword!(condition);
    syn::custom_keyword!(effect);
}

pub(crate) trait TableEntry : Parse {
    const ENUM_NAME: &'static str;
    fn make_decl(&self) -> TokenStream;
    fn make_match_clause(&self) -> TokenStream;

    fn parse_list(input: ParseStream) -> Result<Vec<Self>>;
    fn scope(&self) -> &Ident;

    fn make_special_decl() -> TokenStream {
        TokenStream::new()
    }

    fn early_parse() -> TokenStream {
        TokenStream::new()
    }
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
            #stringy_name => {
                let mut parsee : #ty = Default::default();
                parser.unget(value);
                parsee.read(parser)?;
                Ok(Self::#name(parsee))
            }
        }
    }
}

pub(crate) struct Condition {
    _modifier_token: kw::condition,
    paren_token: token::Paren,
    scope: Ident,
    _comma1_token: Token![,],
    name: Name,
    _comma2_token: Token![,],
    ty: Type
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Condition {
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

impl TableEntry for Condition {
    const ENUM_NAME : &'static str = "Condition";
    fn scope(&self) -> &Ident {
        &self.scope
    }

    fn parse_list(input: ParseStream) -> Result<Vec<Self>> {
        type ConditionList = Punctuated<Condition, Token![;]>;
        let raw_list = ConditionList::parse_terminated(input)?;
        Ok(raw_list.into_iter().collect())
    }

    fn make_decl(&self) -> TokenStream {
        let name = &self.name.name();
        let ty = &self.ty;
        let ty = quote! { Variable<#ty> };
        match &self.name {
            Name::Fixed(_) => quote! { #name(#ty) },
            Name::Dynamic(_, name_ty) =>
                quote! { #name(paradox::IdRef<#name_ty>, #ty) }
        }
    }

    fn make_match_clause(&self) -> TokenStream {
        let name = &self.name.name();
        let stringy_name = stringify(name);
        let ty = &self.ty;
        let ty = quote! { Variable<#ty> };
        match &self.name {
            Name::Fixed(_) => quote_spanned! { self.paren_token.span =>
                #stringy_name => {
                    let mut parsee : #ty = Default::default();
                    parser.unget(value);
                    parsee.read(parser)?;
                    Ok(Self::#name(parsee))
                }
            },
            Name::Dynamic(_, name_ty) => quote_spanned! { self.paren_token.span =>
                name if paradox::IdRef::<#name_ty>::from_str(name, parser.get_game_data()).is_some() => {
                    let id = paradox::IdRef::<#name_ty>::from_str(name, parser.get_game_data()).unwrap();
                    let mut parsee : #ty = Default::default();
                    parser.unget(value);
                    parsee.read(parser)?;
                    Ok(Self::#name(id, parsee))
                }
            }
        }
    }

    fn make_special_decl() -> TokenStream {
        let enum_name = format_ident!("{}", Self::ENUM_NAME);
        quote! {
            Scope(crate::CountryScope),
            Special(paradox::SpecialCondition<#enum_name>)
        }
    }

    fn early_parse() -> TokenStream {
        let enum_name = format_ident!("{}", Self::ENUM_NAME);
        quote! {
            if let Some(val) = paradox::SpecialCondition::<#enum_name>
                    ::try_parse(parser, key, value.clone())? {
                return Ok(Self::Special(val));
            }
            if let Some(val) = crate::CountryScope::get_scope(parser, key) {
                // XXX: Actually parse the inner scope.
                parser.unget(value);
                ().read(parser)?;
                return Ok(Self::Scope(val));
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
    name: Ident,
    scope: Ident,
    arguments: Vec<PatType>,
}

impl Parse for Effect {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::effect) {
            // effect(<Scope>, name, <arg>);
            let content;
            input.parse::<kw::effect>()?;
            let _paren_token = parenthesized!(content in input);
            let scope : Ident = content.parse()?;
            content.parse::<Token![,]>()?;
            let name : Ident = content.parse()?;
            content.parse::<Token![,]>()?;
            let pat_ty = PatType {
                attrs: Vec::new(),
                pat: Box::new(syn::parse_quote! { foo }),
                colon_token: syn::parse_quote! { : },
                ty: Box::new(content.parse()?)
            };
            input.parse::<Token![;]>()?;
            return Ok(Effect {
                name, scope, arguments: vec![pat_ty]
            });
        }
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
            name: function.sig.ident,
            scope,
        })
    }
}

impl Effect {
    fn name(&self) -> &Ident {
        &self.name
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
                parser.unget(value);
                parsee.read(parser)?;
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
                parser.unget(value);
                parsee.read(parser)?;
                Ok(Self::#name { #( #copy_strings ),* })
            }
        };
        let stringy_name = stringify(&name);
        quote_spanned! { name.span() =>
            #stringy_name => {
                #body
            }
        }
    }
}

pub(crate) struct ScopedList<T: TableEntry> {
    entries: Vec<T>
}

impl <T: TableEntry> Parse for ScopedList<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let raw_list = T::parse_list(input)?;
        Ok(Self { entries: raw_list })
    }
}

impl <T: TableEntry> ScopedList<T> {
    pub fn generate_code(&self) -> TokenStream {
        let enum_decl = self.generate_enum();
        let try_from = self.generate_try_from();
        quote! { #enum_decl #try_from }
    }

    fn generate_enum(&self) -> TokenStream {
        let modifiers = &self.entries;
        let enum_name = format_ident!("{}", T::ENUM_NAME);
        let enum_decls = modifiers.iter()
            .map(T::make_decl);
        let extra_decl = T::make_special_decl();
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub enum #enum_name {
                #( #enum_decls, )*
                #extra_decl
            }
        }
    }

    fn generate_try_from(&self) -> TokenStream {
        let enum_name = format_ident!("{}", T::ENUM_NAME);
        let modifiers = &self.entries;
        let match_clauses = modifiers.iter().map(T::make_match_clause);
        let early_parse = T::early_parse();
        quote! {
            impl paradox::FromParadoxKeyPair for #enum_name {
                fn try_from(parser: &mut paradox::Parser, key: &str,
                            value: paradox::Token)
                            -> Result<Self, paradox::ParseError> {
                    use paradox::ParadoxParse;
                    #early_parse
                    match key {
                        #( #match_clauses ),*
                        key => {
                            parser.validation_error(stringify!(#enum_name),
                                key, "no definition found", true, Some(value))?;
                            panic!("Should not reach this point");
                        }
                    }
                }
            }
        }
    }
}

pub(crate) type ScopedConditionList = ScopedList<Condition>;
pub(crate) type ScopedModifierList = ScopedList<Modifier>;
pub(crate) type ScopedEffectList = ScopedList<Effect>;
