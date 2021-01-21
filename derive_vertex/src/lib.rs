use core::panic;
//use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};
// use wgpu::VertexFormat;
// wgpu::VertexStateDescriptor {
//     index_format: Some(wgpu::IndexFormat::Uint16),
//     vertex_buffers: &[wgpu::VertexBufferDescriptor {
//         stride: std::mem::size_of::<$T>() as wgpu::BufferAddress,
//         step_mode: wgpu::InputStepMode::Vertex,
//         attributes: &wgpu::vertex_attr_array![$($loc => $fmt ,)*],
//     }],
// };

// need to change the return type of VertexLike
// also probably needs to be separated.

#[proc_macro]
pub fn get_vertex_layout(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "::wgpu::VertexStateDescriptor {
        index_format: Some(::wgpu::IndexFormat::Uint16),
        vertex_buffers: &[::wgpu::VertexBufferDescriptor {
            stride: 4 as ::wgpu::BufferAddress,
            step_mode: ::wgpu::InputStepMode::Vertex,
            attributes: &::wgpu::vertex_attr_array![0 => Float4],
        }],
    }"
    .parse()
    .unwrap()
}

// I could clean up by removing vecs fields from option types.
#[proc_macro_derive(Vertex)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let struct_name = input_parsed.ident;
    let struct_desc = match input_parsed.data {
        Data::Struct(s) => s,
        _ => panic!("Vertex needs to be structs"),
    };
    let _vertex_descriptor = format_ident!("{}Descriptor", struct_name);
    let mut corresponding_wgpu_vertex_types: Vec<proc_macro2::TokenStream> = vec![];

    struct_desc.fields.iter().for_each(|field| {
        let _name = &field.ident;
        let ty = &field.ty;
        let matching_type = get_matching_wgpu_type(&ty);
        corresponding_wgpu_vertex_types.push(matching_type);
    });

    let expanded = quote! {

        impl app::VertexLike for #struct_name {
            fn get_descriptor() -> i32 {
                42
            }
        }
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

fn get_matching_wgpu_type(ty: &syn::Type) -> proc_macro2::TokenStream {
    match &ty {
        syn::Type::Array(syn::TypeArray {
            elem,
            len:
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }),
            ..
        }) => match *elem.clone() {
            syn::Type::Path(syn::TypePath {
                path: syn::Path { segments, .. },
                ..
            }) => {
                let nb_elements: i32 = lit_int.to_string().parse().unwrap();
                let the_type: &str = &segments.first().unwrap().ident.to_string();
                match (the_type, nb_elements) {
                    ("f32", 2) => quote!("::wgpu::VertexFormat::Float2"),
                    ("f32", 3) => quote!("::wgpu::VertexFormat::Float3"),
                    ("f32", 4) => quote!("::wgpu::VertexFormat::Float4"),
                    _ => {
                        panic!("dont understand this struct");
                    }
                }
            }
            _ => {
                panic!("dont understand this struct");
            }
        },

        _ => {
            panic!("dont understand this struct");
        }
    }
}

// /// Two unsigned bytes (u8). `uvec2` in shaders.
// Uchar2 = 0,
// /// Four unsigned bytes (u8). `uvec4` in shaders.
// Uchar4 = 1,
// /// Two signed bytes (i8). `ivec2` in shaders.
// Char2 = 2,
// /// Four signed bytes (i8). `ivec4` in shaders.
// Char4 = 3,
// /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
// Uchar2Norm = 4,
// /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
// Uchar4Norm = 5,
// /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
// Char2Norm = 6,
// /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
// Char4Norm = 7,
// /// Two unsigned shorts (u16). `uvec2` in shaders.
// Ushort2 = 8,
// /// Four unsigned shorts (u16). `uvec4` in shaders.
// Ushort4 = 9,
// /// Two signed shorts (i16). `ivec2` in shaders.
// Short2 = 10,
// /// Four signed shorts (i16). `ivec4` in shaders.
// Short4 = 11,
// /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
// Ushort2Norm = 12,
// /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
// Ushort4Norm = 13,
// /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
// Short2Norm = 14,
// /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
// Short4Norm = 15,
// /// Two half-precision floats (no Rust equiv). `vec2` in shaders.
// Half2 = 16,
// /// Four half-precision floats (no Rust equiv). `vec4` in shaders.
// Half4 = 17,
// /// One single-precision float (f32). `float` in shaders.
// Float = 18,
// /// Two single-precision floats (f32). `vec2` in shaders.
// Float2 = 19,
// /// Three single-precision floats (f32). `vec3` in shaders.
// Float3 = 20,
// /// Four single-precision floats (f32). `vec4` in shaders.
// Float4 = 21,
// /// One unsigned int (u32). `uint` in shaders.
// Uint = 22,
// /// Two unsigned ints (u32). `uvec2` in shaders.
// Uint2 = 23,
// /// Three unsigned ints (u32). `uvec3` in shaders.
// Uint3 = 24,
// /// Four unsigned ints (u32). `uvec4` in shaders.
// Uint4 = 25,
// /// One signed int (i32). `int` in shaders.
// Int = 26,
// /// Two signed ints (i32). `ivec2` in shaders.
// Int2 = 27,
// /// Three signed ints (i32). `ivec3` in shaders.
// Int3 = 28,
// /// Four signed ints (i32). `ivec4` in shaders.
// Int4 = 29,
