extern crate proc_macro;

mod game;
mod scopes;
mod tables;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Field, Token, Type};

#[derive(Debug)]
struct Error(TokenStream);

impl Error {
    fn new(span: Span, message: &str) -> Error {
        Error(quote_spanned!{span => compile_error!(#message);})
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Self(err.to_compile_error())
    }
}

struct FieldHandler<'a> {
    name: &'a Ident,
    body: TokenStream,
    check_name: Option<Ident>,
    is_default: bool
}

fn stringify(ident: &Ident) -> String {
    let mut s = ident.to_string();
    if s.starts_with("r#") { s.replace_range(0..2, ""); }
    s
}

fn has_tag(field: &Field, tag: &'static str) -> bool {
    field.attrs.iter()
        .any(|attr| attr.path.is_ident(tag))
}

fn get_tag(field: &Field, tag: &'static str) -> Option<TokenStream> {
    field.attrs.iter()
        .find_map(|attr| if attr.path.is_ident(tag) {
            Some(attr.tokens.clone())
        } else {
            None
        })
}

fn handle_field<'a>(field: &'a Field) -> FieldHandler<'a> {
    let name = &field.ident.as_ref().expect("unnamed field?");
    let stringy_name = stringify(name);

    // Get the type as a string. This isn't fully accurate, but it's good enough
    // for any checks we need to do.
    let ty = match &field.ty {
        &Type::Path(ref p) => p.path.clone().into_token_stream().to_string(),
        _ => "".into()
    };

    // This type of field sets the default body instead.
    if has_tag(field, "collect") {
        let make_key = if ty.contains("IdKey") {
            quote_spanned!{field.ty.span() =>
                IdKey::new_via_gamedata(parser.get_game_data(), &key)
            }
        } else {
            quote_spanned!{field.ty.span() => key.into_owned() }
        };
        let body = quote_spanned!{ field.span() =>
            Some(key) => {
                use std::collections::hash_map::Entry;
                let entry = self.#name.entry(#make_key);
                match entry {
                    Entry::Occupied(ref e) =>
                        parser.validation_error(class_name, &format!("{:?}", e.key()),
                            "multiple definitions found", false, None)?,
                    _ => ()
                }
                entry.or_default().read_from(parser, value)?;
            },
        };
        return FieldHandler { name, body, check_name: None, is_default: true };
    }

    // This type of field sets the default body instead.
    if has_tag(field, "modifiers") {
        let body = quote_spanned!{ field.span() =>
            Some(key) => {
                let key = key.into_owned();
                self.#name.push(parser.try_parse(&key, value)?);
            },
        };
        return FieldHandler { name, body, check_name: None, is_default: true };
    }

    // Handle optional and repeated conditions: we build a list of boolean
    // checks, one for each field, to let us know how many times we've seen
    // them before.
    let check_name = if has_tag(field, "optional") || has_tag(field, "repeated") {
        None
    } else {
        Some(format_ident!("seen_{}", name))
    };
    let field_match = quote_spanned!{field.span() =>
        Some(key) if key == #stringy_name
    };
    let check_presence = if let Some(ref check_name) = check_name {
        Some(quote_spanned!{field.span() =>
            if #check_name {
                parser.validation_error(class_name, #stringy_name,
                    "multiple definitions found", true, None)?;
            }
            #check_name = true;
        })
    } else {
        None
    };

    // Build the body of the match.
    let body = if ty.starts_with("Vec <") &&
            (ty.ends_with("Modifier >") || ty.ends_with("Effect >") ||
             ty.ends_with("Condition >")) {
        // List of modifiers are handled with a special parser, due to issues
        // doing so with other types.
        quote_spanned!{field.span() =>
            #field_match => {
                #check_presence
                self.#name = paradox::parse_key_pair_list(parser, value)?;
            }
        }
    } else {
        // Regular types. Build up a check here.
        let parsee = quote_spanned!{field.span() =>
            let parsee : &mut dyn paradox::ParadoxParse
        };
        let get_parsee = if has_tag(field, "repeated") {
            quote_spanned!{field.span() =>
                self.#name.push(Default::default());
                #parsee = self.#name.last_mut().unwrap();
            }
        } else {
            quote_spanned!{field.span() =>
                #parsee = &mut self.#name;
            }
        };
        quote_spanned!{field.span() =>
            #field_match => {
                #check_presence
                #get_parsee
                parsee.read_from(parser, value)?;
            }
        }
    };

    FieldHandler { body, check_name, name, is_default: false }
}

fn implement_parse_method(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let body : Vec<_> = match &input.data {
        Data::Struct(data) => data.fields.iter().map(handle_field).collect(),
        _ => return Err(Error::new(input.span(),
                                   "Can only derive ParadoxParse for structs"))
    };

    let mut default_body = quote! {
        Some(key) => {
            parser.validation_error(class_name, &key, "unknown in struct",
                                    false, Some(value))?;
        },
    };
    let mut match_statements = Vec::new();
    let mut prologue = Vec::new();
    let mut epilogue = Vec::new();
    for field in body {
        if field.is_default {
            default_body = field.body;
            continue;
        }
        let field_name = field.name;
        let stringy_name = stringify(&field_name);
        match_statements.push(field.body);
        if let Some(check_name) = &field.check_name {
            prologue.push(quote! { let mut #check_name = false; });
            epilogue.push(quote! {
                if !#check_name {
                    parser.validation_error(class_name, #stringy_name,
                        "not found in definition", false, None)?;
                }
            });
        }
    }

    let expanded = quote! {
        #[automatically_derived]
        impl paradox::ParadoxParse for #name {
            fn read_from(&mut self, parser: &mut paradox::Parser,
                         val: paradox::UnparsedValue)
                    -> Result<(), paradox::ParseError> {
                let class_name = std::any::type_name::<Self>();
                #( #prologue )*
                val.expect_complex()?;
                while let Some((key, value)) = parser.get_next_value()? {
                    match key {
                        None => {
                            parser.validation_error(class_name, "",
                                "bad key", false, Some(value))?;
                        },
                        #( #match_statements, )*
                        #default_body
                    }
                }
                #( #epilogue )*
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

#[proc_macro_derive(ParadoxParse, attributes(collect, modifiers, optional, repeated))]
pub fn derive_paradox_parse(input: proc_macro::TokenStream)
        -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    implement_parse_method(&input)
        .unwrap_or_else(|err| err.0)
        .into()
}

#[proc_macro_derive(GameData, attributes(parse))]
pub fn derive_game_data(input: proc_macro::TokenStream)
        -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    game::implement_game(&input)
        .unwrap_or_else(|err| err.0)
        .into()
}

/// Generate a set of modifier enums and associated parsing.
///
/// This macro takes as input a table of modifiers, such as:
/// ```rust
///   modifier_list!{
///     modifier(Country, country_modifier, FixedPoint);
///   }
/// ```
///
/// The first parameter is the kind of scope that the modifier applies to.
///
/// The second parameter is the name of modifier.
///
/// The third and final parameter is the type of the modifier.
#[proc_macro]
pub fn modifier_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let table = parse_macro_input!(input as tables::ScopedModifierList);
    table.generate_code().into()
}

/// Generate a set of condition enums and associated parsing.
///
/// This macro takes as input a table of conditions, such as:
/// ```rust
///   condition_list!{
///     condition(Country, country_condition, FixedPoint);
///   }
/// ```
///
/// The first parameter is the kind of scope that the condition applies to.
///
/// The second parameter is the name of condition.
///
/// The third and final parameter is the type of the condition.
#[proc_macro]
pub fn condition_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let table = parse_macro_input!(input as tables::ScopedConditionList);
    table.generate_code().into()
}

#[proc_macro]
pub fn effect_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let table = parse_macro_input!(input as tables::ScopedEffectList);
    table.generate_code().into()
}

#[proc_macro]
pub fn scope_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scopes::scope_list(input)
}

enum Name {
    Fixed(Ident),
    Dynamic(Ident, Ident)
}

impl Parse for Name {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            let name : Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty : Ident = input.parse()?;
            input.parse::<Token![>]>()?;
            Ok(Self::Dynamic(name, ty))
        } else {
            Ok(Self::Fixed(input.parse()?))
        }
    }
}

impl Name {
    fn name(&self) -> &Ident {
        match self {
            Self::Fixed(n) => &n,
            Self::Dynamic(n, _) => &n
        }
    }
}

