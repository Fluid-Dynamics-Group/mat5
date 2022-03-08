use proc_macro::TokenStream;
use quote::quote;
use syn::Result;
use syn::spanned::Spanned;

use darling::{ast, FromDeriveInput, FromField};

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_any), attributes(mat5))]
struct InputReceiver {
    /// The struct ident.
    #[allow(dead_code)]
    ident: syn::Ident,

    #[allow(dead_code)]
    generics: syn::Generics,

    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<(), FieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(mat5))]
struct FieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,

    /// This magic field name pulls the type from the input.
    ty: syn::Type,

    #[darling(default)]
    /// whether or not to use `std::ops::Deref` on the field before 
    /// serializing the container
    deref: bool
}

/// creates all the function calls to the mat5 library within the function body of
/// MatFile::write_contents (without any of the function parameters)
fn create_trait_function_body(input: Vec<(syn::Ident, syn::Type, bool)>) -> proc_macro2::TokenStream {
    let mut out = quote!(
        mat5::write_default_header(&mut writer)?;
    );

    for (field_name, _field_type, deref) in input {
        let lit = syn::LitStr::new(&field_name.to_string(), field_name.span());

        if deref {
            out = quote!(
                #out
                mat5::Container::write_container(std::ops::Deref::deref(&self.#field_name), &mut writer, #lit)?;
            );
        } else {
            out = quote!(
                #out
                mat5::Container::write_container(&self.#field_name, &mut writer, #lit)?;
            );
        }
        
    }

    quote!(
        #out
        Ok(())
    )
}


pub fn derive(input: syn::DeriveInput) -> Result<TokenStream> {
    let receiver = InputReceiver::from_derive_input(&input).unwrap();

    let fields_information = match receiver.data {
        ast::Data::Enum(_) => unreachable!(),
        ast::Data::Struct(fields_with_style) => {
            let fields = fields_with_style.fields;
            fields.into_iter()
                .map(|rx: FieldReceiver| {

                    if let Some(ident) = rx.ident {
                        Ok(
                            (ident, rx.ty, rx.deref)
                        )
                    } else {
                        Err(
                            syn::parse::Error::new(rx.ty.span(), "Tuple structs are not accepted. Each field must be named")
                        )
                    }

                })
                .collect::<Result<Vec<(syn::Ident, syn::Type, bool)>>>()
        }
    }?;

    let function_body = create_trait_function_body(fields_information);
    let target_type = input.ident;
    let (imp, ty, wher) = input.generics.split_for_impl();

    let trait_impl = quote!(
        impl #imp mat5::MatFile for #target_type #ty #wher {
            fn write_contents<W: std::io::Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
                #function_body
            }
        }
    );

    Ok(trait_impl.into())
}
