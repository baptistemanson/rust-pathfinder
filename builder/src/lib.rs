use core::panic;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Data, DeriveInput};

// I could clean up by removing vecs fields from option types.
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let struct_name = input_parsed.ident;
    let struct_desc = match input_parsed.data {
        Data::Struct(s) => s,
        _ => panic!("Only accept struct"),
    };

    let builder_struct_name = format_ident!("{}Builder", struct_name);
    let mut field_name: Vec<proc_macro2::TokenStream> = vec![];
    let mut field_type: Vec<proc_macro2::TokenStream> = vec![];
    let mut field_setter_type: Vec<proc_macro2::TokenStream> = vec![];
    let mut setter_function: Vec<proc_macro2::TokenStream> = vec![];
    let mut field_build_expression: Vec<proc_macro2::TokenStream> = vec![];

    let first_error = struct_desc.fields.iter().find_map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        let attributes = &field.attrs;
        let is_option = is_option_type(&field.ty);
        let each_name = get_iteration_name(attributes);
        // if the name is incorrect
        if each_name.is_err() {
            return Some(
                syn::Error::new_spanned(&attributes[0].tokens, "expected builder(each = \"...\")")
                    .to_compile_error(),
            );
        }
        let each_name = each_name.ok().unwrap();
        let is_vector = each_name.is_some();
        // struct
        field_name.push(quote! {#name});
        field_type.push(quote! {#ty});

        // setter
        // Even if the field is Option<InnerType>, we still want the setter parameter to be of InnerType
        // we have no way to reset it, but that is the specs.
        let setter_type = match (is_option, is_vector) {
            (true, _) => &get_option_inner_type(&field.ty),
            (_, true) => &field.ty,
            (_, _) => &field.ty,
        };
        field_setter_type.push(quote! { #setter_type});

        // each support
        if let Some(each_name) = each_name {
            let each_name = format_ident!("{}", each_name);
            // @todo Here I assumed it was a string inside the vec, extract proper value
            setter_function.push(quote! {
                pub fn #each_name(&mut self, #each_name: String) -> &mut Self {
                    if let Some(ref mut options) = self.#name {
                        options.push(#each_name);
                    }
                    else{
                        self.#name = ::std::option::Option::Some(vec![#each_name]);
                    }
                    self
                }
            })
        } else {
            // regular setter
            setter_function.push(quote! {
                pub fn #name(&mut self, #name: #setter_type) -> &mut Self {
                    self.#name = ::std::option::Option::Some(#name);
                    self
                }
            });
        }

        // build
        let builder_expression = if is_option {
            quote! { let #name = self.#name.clone()}
        } else if is_vector {
            quote! { let #name = self.#name.clone().unwrap_or(vec![])}
        } else {
            // when the field is not an option, trigger and error when None.
            quote! { let #name = self.#name.clone().ok_or("a mandatory field is missing :")?}
        };
        field_build_expression.push(builder_expression);

        // Everyhing was ok
        None
    });

    if first_error.is_some() {
        proc_macro::TokenStream::from(first_error.unwrap())
    } else {
        let expanded = quote! {
        #[derive(Debug)]
                pub struct #builder_struct_name {
                    #(#field_name: ::std::option::Option<#field_setter_type>,)*
                }

                impl #struct_name {
                    pub fn builder() -> #builder_struct_name {
                        #builder_struct_name {
                            #(#field_name : None,)*
                        }
                    }
                }

                impl #builder_struct_name {

                    #(
                        #setter_function
                    )*

                    pub fn build(&mut self) -> ::std::result::Result<#struct_name, ::std::boxed::Box<dyn ::std::error::Error>> {
                        #(#field_build_expression;)*
                        ::std::result::Result::Ok(#struct_name {
                            #(#field_name,)*
                        })
                    }
                }
            };

        // Hand the output tokens back to the compiler
        proc_macro::TokenStream::from(expanded)
    }
}

// Utilities
/// Returns true if the type is Option<T>
fn is_option_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(p) => p.path.segments.len() > 0 && p.path.segments[0].ident == "Option",
        _ => false,
    }
}

/// Given Option<T>, returns T
/// not very resilient, should only be called on known option types.
fn get_option_inner_type(ty: &syn::Type) -> &syn::Type {
    match ty {
        syn::Type::Path(p) => match &p.path.segments[0].arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => {
                if args.len() == 0 {
                    panic!("Invalid Option type parsing - E01")
                }
                if let syn::GenericArgument::Type(optional_type) = &args[0] {
                    optional_type
                } else {
                    panic!("Invalid Option type parsing - E02")
                }
            }
            _ => panic!("Invalid Option type parsing - E03"),
        },
        _ => panic!("Only works on Option type - E04"),
    }
}

fn get_iteration_name(attrs: &Vec<Attribute>) -> Result<Option<String>, String> {
    let val = attrs.iter().find_map(|f| {
        // looking for #[builder(each = "..")]
        if f.path.segments.len() == 1 && f.path.segments[0].ident == "builder" {
            let meta = f.parse_meta().unwrap();
            match meta {
                // maybe I can combine all of this in one pattern?
                syn::Meta::List(syn::MetaList { nested, .. }) => {
                    if nested.len() == 0 {
                        Some(Err(String::from("expected builder(each = \"...\")")))
                    } else {
                        // @todo check here that the user typed "each" and not something else.
                        match &nested[0] {
                            syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                                path,
                                lit: syn::Lit::Str(function_name),
                                ..
                            })) => {
                                let syn::PathSegment { ref ident, .. } = path.segments[0];
                                if ident.to_string() != "each" {
                                    Some(Err(String::from("expected builder(each = \"...\")")))
                                } else {
                                    Some(Ok(Some(function_name.value())))
                                }
                            }
                            _ => None,
                        }
                    }
                }
                _ => None,
            }
            //  let meta = f.parse_meta().unwrap();
        } else {
            None
        }
    });
    val.unwrap_or(Ok(None))
}
