extern crate proc_macro;

mod game;
mod tables;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Field, Type};

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

fn handle_field<'a>(field: &'a Field, class: &Ident) -> FieldHandler<'a> {
    let name = &field.ident.as_ref().expect("unnamed field?");

    // This type of field sets the default body instead.
    if has_tag(field, "collect") {
        let body = quote_spanned!{ field.span() =>
            Some((Some(key), value)) => {
                use std::collections::hash_map::Entry;
                let entry = self.#name.entry(key);
                match entry {
                    Entry::Occupied(ref e) =>
                        value.validation_error(stringify!(#class), &e.key(),
                            "multiple definitions found", false)?,
                    _ => ()
                }
                entry.or_default().read_from(value)?;
            },
        };
        return FieldHandler { name, body, check_name: None, is_default: true };
    }

    // This type of field sets the default body instead.
    if has_tag(field, "effects") {
        let body = quote_spanned!{ field.span() =>
            Some((Some(key), value)) => {
                use std::convert::TryInto;
                self.#name.push((key, value).try_into()?);
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
    let key_check = if let Some(tokens) = get_tag(field, "id") {
        quote_spanned!{field.span() => || key == format!("{:04x}", #tokens)}
    } else {
        TokenStream::new()
    };
    let field_match = quote_spanned!{field.span() =>
        Some((Some(key), value)) if key == stringify!(#name) #key_check
    };
    let check_presence = if let Some(ref check_name) = check_name {
        Some(quote_spanned!{field.span() =>
            if #check_name {
                value.validation_error(stringify!(#class), stringify!(#name),
                    "multiple definitions found", true)?;
            }
            #check_name = true;
        })
    } else {
        None
    };

    // Get the type as a string. This isn't fully accurate, but it's good enough
    // for any checks we need to do.
    let ty = match &field.ty {
        &Type::Path(ref p) => p.path.clone().into_token_stream().to_string(),
        _ => "".into()
    };

    // Build the body of the match.
    let body = if ty.starts_with("Vec <") && ty.ends_with("Effect >") {
        // List of effects are handled with a special parser, due to issues
        // doing so with other types.
        quote_spanned!{field.span() =>
            #field_match => {
                #check_presence
                let vec = &mut self.#name;
                loop {
                    let next_pair = val.next_key_value_pair()?;
                    match next_pair {
                        None => break,
                        Some((None, value)) => {
                            value.validation_error(
                                stringify!(#class), "", "bad key", false)?;
                            value.drain()?;
                        },
                        Some((Some(key), value)) => {
                            use std::convert::TryInto;
                            vec.push((key, value).try_into()?);
                        }
                    }
                }
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
                parsee.read_from(value)?;
            }
        }
    };

    FieldHandler { body, check_name, name, is_default: false }
}

fn implement_parse_method(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let body : Vec<_> = match &input.data {
        Data::Struct(data) => data.fields.iter().map(|f| handle_field(f, name)).collect(),
        _ => return Err(Error::new(input.span(),
                                   "Can only derive ParadoxParse for structs"))
    };

    let mut default_body = quote! {
        Some((Some(key), val)) => {
            println!("{}/{}", stringify!(#name), key);
            val.drain()?;
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
        match_statements.push(field.body);
        if let Some(check_name) = &field.check_name {
            prologue.push(quote! { let mut #check_name = false; });
            epilogue.push(quote! {
                if !#check_name {
                    val.validation_error(stringify!(#name),
                        stringify!(#field_name), "not found in definition",
                        false)?;
                }
            });
        }
    }

    let expanded = quote! {
        #[automatically_derived]
        impl paradox::ParadoxParse for #name {
            fn read_from(&mut self, mut val: paradox::UnparsedValue<'_>)
                    -> Result<(), paradox::ParseError> {
                #( #prologue )*
                loop {
                    let next_pair = val.next_key_value_pair()?;
                    match next_pair {
                        None => break,
                        Some((None, val)) => {
                            val.validation_error(stringify!(#name), "",
                                "bad key", false)?;
                            val.drain()?;
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

#[proc_macro_derive(ParadoxParse, attributes(collect, effects, id, optional, repeated))]
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

/// Generate a set of effect enums and associated parsing.
///
/// This macro takes as input a table of effects, such as:
/// ```rust
///   effect_list!{
///     effect(Country, country_modifier, FixedPoint);
///   }
/// ```
///
/// The first parameter is the kind of scope that the modifier applies to.
///
/// The second parameter is the name of modifier.
///
/// The third and final parameter is the type of the modifier.
#[proc_macro]
pub fn effect_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let table = parse_macro_input!(input as tables::ScopedEffectList);
    table.generate_code().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = quote!{ struct Foo { field: u32 }};
        let input = syn::parse2::<DeriveInput>(input).unwrap();
        println!("{}", implement_parse_method(&input).unwrap());
    }
}
