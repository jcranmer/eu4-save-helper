use proc_macro2::{TokenStream};
use quote::{quote, quote_spanned};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Field, Lit, Meta, Type};

use crate::{Error};

fn get_tag(field: &Field, tag: &'static str) -> Option<Lit> {
    field.attrs.iter()
        .find_map(|attr| if attr.path.is_ident(tag) {
            attr.parse_meta().ok()
                .and_then(|meta| match meta {
                    Meta::NameValue(val) => Some(val.lit),
                    _ => None
                })
        } else {
            None
        })
}

pub(crate) fn implement_game(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => return Err(Error::new(input.span(),
                                   "Can only derive ParadoxParse for structs"))
    };
    let mut base_field = None;
    let mut eager_fields : Vec<_> = Vec::with_capacity(fields.len());

    for field in fields {
        let ty_str = match &field.ty {
            &Type::Path(ref p) => p.path.clone().into_token_stream().to_string(),
            _ => "".into()
        };

        // Check for the base field.
        if ty_str.ends_with("GameData") {
            base_field = Some(
                field.ident.as_ref()
                    .ok_or(Error::new(field.span(), "Unnamed field"))?);
            continue;
        }

        if let Some(path) = get_tag(&field, "parse") {
            let field_data = (&field.ident, path);
            eager_fields.push(field_data);
        } else {
            Err(Error::new(field.span(), "Field needs parse annotation"))?;
        }
    }

    let base_field = base_field
        .ok_or(Error::new(input.span(),
                          "Missing field containing paradox::GameData"))?;

    // Construct the constructor.
    let new_fields_init = eager_fields.iter()
        .map(|(name, _)| quote! { #name: Default::default() });
    let new_fields_parse = eager_fields.iter()
        .map(|(name, path)| quote! {
            result.#base_field
                .parse_directory::<crate::Eu4Trait>(#path, &mut result.#name)?;
        });
    let new_method = quote_spanned! { input.span() =>
        pub fn new(game_dir: &std::path::Path
                   ) -> Result<Self, paradox::ParseError> {
            let base_info = paradox::GameData::load(game_dir)?;
            let mut result = Self {
                base_info,
                #( #new_fields_init ),*
            };
            #( #new_fields_parse )*
            Ok(result)
        }
    };

    // Return the full thing.
    let expanded = quote! {
        #[automatically_derived]
        impl #name {
            #new_method
        }
    };

    Ok(TokenStream::from(expanded))
}

