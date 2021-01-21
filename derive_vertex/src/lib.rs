use core::panic;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};
use wgpu::VertexFormat;

// I could clean up by removing vecs fields from option types.
#[proc_macro_derive(Vertex)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let struct_name = input_parsed.ident;
    let struct_desc = match input_parsed.data {
        Data::Struct(s) => s,
        _ => panic!("Vertex needs to be structs"),
    };
    // let vertex_state = vertex_layout![Vertex : 0 => Float4];

    let vertex_descriptor = format_ident!("{}Descriptor", struct_name);

    struct_desc.fields.iter().for_each(|field| {
        let name = &field.ident;
        let ty = &field.ty;

        // if the name is incorrect
    });

    let expanded = quote! {
        fn test() -> i32 {
            42
        }
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}
