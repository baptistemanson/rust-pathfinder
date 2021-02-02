use core::panic;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Vertex)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_parsed = parse_macro_input!(input as DeriveInput);
    let struct_name = input_parsed.ident;
    let struct_desc = match input_parsed.data {
        Data::Struct(s) => s,
        _ => panic!("Vertex needs to be structs"),
    };

    let mut vertex_attributes: Vec<proc_macro2::TokenStream> = vec![];
    let mut offset: u64 = 0;
    let mut i: u32 = 0;

    struct_desc.fields.iter().for_each(|field| {
        let _name = &field.ident;
        let ty = &field.ty;
        let (format, size) = get_matching_wgpu_type(&ty);
        let escaped_format = format_ident!("{}", format);
        let va = quote! {
            ::wgpu::VertexAttribute {
                offset: #offset,
                format: ::wgpu::VertexFormat::#escaped_format,
                shader_location: #i,
            }
        };

        vertex_attributes.push(quote!(#va));
        offset += size;
        i += 1;
    });

    let expanded = quote! {

        impl ::wgputils::Vertex for #struct_name {
            fn get_descriptor<'a>(module: &'a ::wgpu::ShaderModule) -> ::wgpu::VertexState<'a> {
                ::wgpu::VertexState {
                        module,
                        entry_point: "main",
                        buffers: &[::wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<#struct_name>() as ::wgpu::BufferAddress,
                            step_mode: ::wgpu::InputStepMode::Vertex,
                            attributes:&[#(#vertex_attributes, )*
                            ]
                        }],
                    }
            }
            fn create_index_buffer(device: &::wgpu::Device, data: &[u8]) -> ::wgpu::Buffer {
                use ::wgpu::util::DeviceExt;
                device.create_buffer_init(&::wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: data, // checks if a range of bytes can be turned into another and just do it. Works well to turn Struct into u8
                    usage: ::wgpu::BufferUsage::INDEX | ::wgpu::BufferUsage::COPY_DST,
                })
            }

            fn create_vertex_buffer(device: &::wgpu::Device, data: &[u8]) -> ::wgpu::Buffer {
                use ::wgpu::util::DeviceExt;
                device.create_buffer_init(&::wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: data, // checks if a range of bytes can be turned into another and just do it. Works well to turn Struct into u8
                    usage: ::wgpu::BufferUsage::VERTEX | ::wgpu::BufferUsage::COPY_DST,
                })
            }
        }
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

fn get_matching_wgpu_type(ty: &syn::Type) -> (&str, u64) {
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
                    ("f32", 1) => ("Float", ::wgpu::VertexFormat::Float.size()),
                    ("f32", 2) => ("Float2", ::wgpu::VertexFormat::Float2.size()),
                    ("f32", 3) => ("Float3", ::wgpu::VertexFormat::Float3.size()),
                    ("f32", 4) => ("Float4", ::wgpu::VertexFormat::Float4.size()),

                    ("u32", 1) => ("Uint", ::wgpu::VertexFormat::Uint.size()),
                    ("u32", 2) => ("Uint2", ::wgpu::VertexFormat::Uint2.size()),
                    ("u32", 3) => ("Uint3", ::wgpu::VertexFormat::Uint3.size()),
                    ("u32", 4) => ("Uint4", ::wgpu::VertexFormat::Uint4.size()),

                    ("i32", 1) => ("Int", ::wgpu::VertexFormat::Int.size()),
                    ("i32", 2) => ("Int2", ::wgpu::VertexFormat::Int2.size()),
                    ("i32", 3) => ("Int3", ::wgpu::VertexFormat::Int3.size()),
                    ("i32", 4) => ("Int4", ::wgpu::VertexFormat::Int4.size()),

                    ("u8", 2) => ("Uchar2", ::wgpu::VertexFormat::Uchar2.size()),
                    ("u8", 4) => ("Uchar4", ::wgpu::VertexFormat::Uchar4.size()),

                    ("i8", 2) => ("Char2", ::wgpu::VertexFormat::Char2.size()),
                    ("i8", 4) => ("Char4", ::wgpu::VertexFormat::Char4.size()),

                    ("u16", 2) => ("Ushort2", ::wgpu::VertexFormat::Ushort2.size()),
                    ("u16", 4) => ("Ushort4", ::wgpu::VertexFormat::Ushort4.size()),

                    ("i16", 2) => ("Short2", ::wgpu::VertexFormat::Short2.size()),
                    ("i16", 4) => ("Short4", ::wgpu::VertexFormat::Short4.size()),
                    _ => {
                        panic!("dont support this type of parameter.");
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

// Currently not supported, would need annotation
// norm not done, would need annotation
// /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
// Uchar2Norm = 4,
// /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
// Uchar4Norm = 5,
// /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
// Char2Norm = 6,
// /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
// Char4Norm = 7,
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
