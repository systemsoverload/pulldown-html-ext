use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn html_writer(attr: TokenStream, input: TokenStream) -> TokenStream {
    let skip_docs = attr.to_string().contains("skip_docs");
    let input = parse_macro_input!(input as DeriveInput);

    match process_html_writer(&input, skip_docs) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn process_html_writer(
    input: &DeriveInput,
    skip_docs: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let _generics = &input.generics;

    // Validate the input is a struct
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => fields,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "struct must have named fields",
                ))
            }
        },
        Data::Enum(_) => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "html_writer can only be applied to structs, not enums",
            ))
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "html_writer can only be applied to structs, not unions",
            ))
        }
    };

    // Validate base field exists and has correct type
    let base_field = fields
        .named
        .iter()
        .find(|f| f.ident.as_ref().map_or(false, |i| i == "base"))
        .ok_or_else(|| syn::Error::new_spanned(fields, "struct must have a field named 'base'"))?;

    // Check base field type is HtmlWriterBase<W>
    let base_type = &base_field.ty;
    let is_valid_base_type = quote!(#base_type).to_string().contains("HtmlWriterBase");
    if !is_valid_base_type {
        return Err(syn::Error::new_spanned(
            base_type,
            "base field must be of type HtmlWriterBase<W>",
        ));
    }

    // Generate implementation
    let docs = if !skip_docs {
        quote! {
            #[doc = "An HTML writer implementation."]
            #[doc = "This type implements methods for writing HTML elements."]
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #docs
        #input

        impl<W: StrWrite> HtmlWriter<W> for #name<W> {
            fn get_writer(&mut self) -> &mut W {
                self.base.get_writer()
            }

            fn get_config(&self) -> &HtmlConfig {
                self.base.get_config()
            }

            fn get_state(&mut self) -> &mut HtmlState {
                self.base.get_state()
            }
        }
    };

    Ok(expanded)
}
