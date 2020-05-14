extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
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
    body: TokenStream,
    check_name: Option<Ident>
}

fn has_tag(field: &Field, tag: &'static str) -> bool {
    field.attrs.iter()
        .any(|attr| attr.path.is_ident(tag))
}

fn handle_field(field: &Field) -> FieldHandler {
    let name = &field.ident.as_ref().expect("unnamed field?");
    let check_name = if has_tag(field, "optional") || has_tag(field, "repeated") {
        None
    } else {
        Some(format_ident!("seen_{}", name))
    };
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
    let check_presence = if let Some(ref check_name) = check_name {
        Some(quote_spanned!{field.span() =>
            if #check_name {
                panic!("Multiple definitions of {}", stringify!(#name));
            }
            #check_name = true;
        })
    } else {
        None
    };
    let body = quote_spanned!{field.span() =>
        #field_match => {
            #check_presence
            #get_parsee
            parsee.read_from(value)?;
        }
    };

    FieldHandler { body, check_name }
}

fn implement_parse_method(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let body : Vec<_> = match &input.data {
        Data::Struct(data) => data.fields.iter().map(handle_field).collect(),
        _ => return Err(Error::new(input.span(),
                                   "Can only derive ParadoxParse for structs"))
    };

    let match_statements = body.iter().map(|f| &f.body);
    let declare_checks : Vec<_> = body.iter().map(|f| {
        if let Some(check_name) = &f.check_name {
            quote! { let mut #check_name = false; }
        } else {
            TokenStream::new()
        }
    }).collect();
    let check_presence : Vec<_> = body.iter().map(|f| {
        if let Some(check_name) = &f.check_name {
            quote! { assert!(#check_name, concat!("Missing option ", stringify!(#check_name), " in ", stringify!(#name))); }
        } else {
            TokenStream::new()
        }
    }).collect();
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
                #( #declare_checks )*
                loop {
                    let next_pair = val.next_key_value_pair()?;
                    match next_pair {
                        None => break,
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
                #( #check_presence )*
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

#[proc_macro_derive(ParadoxParse, attributes(optional, repeated))]
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
