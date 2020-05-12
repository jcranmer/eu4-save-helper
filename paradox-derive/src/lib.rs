extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Field};

#[derive(Debug)]
struct Error(TokenStream);

impl Error {
    fn new(span: Span, message: &str) -> Error {
        Error(quote_spanned!{span => compile_error!(#message)})
    }
}

#[derive(Default)]
struct FieldHandler {
    body: TokenStream
}

fn has_tag(field: &Field, tag: &'static str) -> bool {
    field.attrs.iter()
        .any(|attr| attr.path.is_ident(tag))
}

fn handle_field(field: &Field) -> FieldHandler {
    let name = &field.ident;
    let field_match = quote_spanned!{field.span() =>
        Some((Some(key), value)) if key == stringify!(#name)
    };
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
    let body = quote_spanned!{field.span() =>
        #field_match => {
            #get_parsee
            parsee.read_from(value)?;
        }
    };

    FieldHandler { body }
}

fn implement_parse_method(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let body : Vec<_> = match &input.data {
        Data::Struct(data) => data.fields.iter().map(handle_field).collect(),
        _ => return Err(Error::new(input.span(),
                                   "Can only derive ParadoxParse for structs"))
    };

    let match_statements = body.iter().map(|f| &f.body);
    let default_body = quote! {
        Some((Some(key), val)) => {
            println!("{}/{}", stringify!(#name), key);
            val.drain()?;
        },
    };

    let expanded = quote! {
        impl paradox::ParadoxParse for #name {
            fn read_from(&mut self, mut val: paradox::UnparsedValue<'_>)
                    -> Result<(), paradox::ParseError> {
                loop {
                    let next_pair = val.next_key_value_pair()?;
                    match next_pair {
                        None => return Ok(()),
                        Some((None, val)) => {
                            val.drain()?;
                            return Err(paradox::ParseError::Constraint(
                                format!("Unexpected missing key in {}",
                                        stringify!(#name))));
                        },
                        #( #match_statements, )*
                        #default_body
                    }
                }
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

#[proc_macro_derive(ParadoxParse, attributes(repeated))]
pub fn derive_paradox_parse(input: proc_macro::TokenStream)
        -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    implement_parse_method(&input)
        .unwrap_or_else(|err| err.0)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = quote!{ struct Foo { field: u32 }};
        let input = syn::parse2::<DeriveInput>(input).unwrap();
        println!("{}", implement_parse_method(&input).unwrap());
        unimplemented!();
    }
}
