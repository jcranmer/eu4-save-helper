use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parenthesized, parse_macro_input, token, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(scope);
    syn::custom_keyword!(scope_many);
    syn::custom_punctuation!(All, *);
}

enum ScopeType {
    All(kw::All),
    Specific(Ident)
}

impl Parse for ScopeType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![*]) {
            Ok(Self::All(input.parse()?))
        } else {
            Ok(Self::Specific(input.parse()?))
        }
    }
}

enum ScopeKw {
    Scope(kw::scope),
    ScopeMany(kw::scope_many)
}

impl Parse for ScopeKw {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::scope) {
            Ok(Self::Scope(input.parse()?))
        } else {
            Ok(Self::ScopeMany(input.parse()?))
        }
    }
}

struct Scope {
    scope_kind: ScopeKw,
    paren_token: token::Paren,
    outer_scope: ScopeType,
    _comma1_token: Token![,],
    inner_scope: Ident,
    _comma2_token: Token![,],
    name: Ident
}

impl Parse for Scope {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            scope_kind: input.parse()?,
            paren_token: parenthesized!(content in input),
            outer_scope: content.parse()?,
            _comma1_token: content.parse()?,
            inner_scope: content.parse()?,
            _comma2_token: content.parse()?,
            name: content.parse()?,
        })
    }
}

impl Scope {
    fn declare_enum(&self) -> TokenStream {
        let name = &self.name;
        let params = match self.scope_kind {
            ScopeKw::Scope(_) => TokenStream::new(),
            ScopeKw::ScopeMany(_) => quote! { (bool) }
        };
        quote_spanned!{self.name.span() => #name #params }
    }

    fn parse_stmt(&self) -> TokenStream {
        let name = &self.name;
        match self.scope_kind {
            ScopeKw::Scope(_) => quote_spanned!{self.name.span() =>
                stringify!(#name) => Some(Self::#name),
            },
            ScopeKw::ScopeMany(_) => quote_spanned!{self.name.span() =>
                concat!("any_", stringify!(#name)) |
                concat!("random_", stringify!(#name)) =>
                    Some(Self::#name(true)),
                concat!("all_", stringify!(#name)) |
                concat!("every_", stringify!(#name)) =>
                    Some(Self::#name(false)),
            },
        }
    }
}

struct ScopeList {
    scopes: Vec<Scope>
}

impl Parse for ScopeList {
    fn parse(input: ParseStream) -> Result<Self> {
        type ScopeList = Punctuated<Scope, Token![;]>;
        let raw_list = ScopeList::parse_terminated(input)?;
        Ok(Self { scopes: raw_list.into_iter().collect() })
    }
}


impl ScopeList {
    fn generate_enum(&self) -> TokenStream {
        let scope_decl = self.scopes.iter().map(Scope::declare_enum);
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub enum Scope {
                From,
                Root,
                Prev,
                This,
                #( #scope_decl ),*
            }
        }
    }

    fn generate_parse(&self) -> TokenStream {
        let match_stmt = self.scopes.iter().map(Scope::parse_stmt);
        quote! {
            pub fn get_scope(parser: &mut paradox::Parser,
                             key: &str) -> Option<Self> {
                let data = parser.get_game_data();
                match key {
                    "FROM" => Some(Self::From),
                    "ROOT" => Some(Self::Root),
                    "PREV" => Some(Self::Prev),
                    "THIS" => Some(Self::This),
                    #( #match_stmt )*
                    _ => None
                }
            }
        }
    }

    fn generate_code(&self) -> TokenStream {
        let enum_decl = self.generate_enum();
        let parse_code = self.generate_parse();
        quote! {
            #enum_decl

            impl Scope {
                #parse_code
            }
        }
    }
}

pub fn scope_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let list = parse_macro_input!(input as ScopeList);
    list.generate_code().into()
}
