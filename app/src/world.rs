use wgpu::{Device, Queue};

use wgputils::texture::{BatTexDimensions, Texture};

pub fn lower<'a>(device: &'a Device, queue: &'a Queue) -> Texture<'a> {
    let bytes: Vec<Vec<f32>> = vec![
        vec![
            32., 226., 1., 2., 1., 3., 1., 2., 1., 1., 4., 1., 2., 3., 3., 1., 1., 1., 3., 227.,
        ],
        vec![
            32., 258., 33., 34., 65., 34., 35., 35., 65., 35., 36., 35., 34., 33., 34., 34., 66.,
            101., 34., 259.,
        ],
        vec![
            32., 129., 130., 130., 130., 130., 130., 130., 130., 130., 130., 130., 163., 163.,
            130., 130., 130., 130., 130., 130.,
        ],
        vec![
            32., 129., 129., 130., 130., 129., 130., 130., 129., 129., 131., 163., 163., 129.,
            129., 129., 129., 131., 130., 130.,
        ],
        vec![
            32., 129., 129., 129., 129., 129., 129., 131., 129., 129., 129., 129., 129., 129.,
            129., 129., 161., 131., 131., 130.,
        ],
        vec![
            32., 129., 130., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129.,
            129., 129., 131., 131., 130., 130.,
        ],
        vec![
            32., 129., 130., 129., 131., 131., 131., 129., 129., 129., 129., 129., 129., 129.,
            129., 163., 130., 130., 129., 129.,
        ],
        vec![
            32., 129., 130., 129., 130., 163., 163., 163., 129., 129., 194., 129., 129., 129.,
            129., 294., 130., 130., 129., 129.,
        ],
        vec![
            32., 129., 163., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 353.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            32., 129., 129., 129., 129., 294., 129., 129., 129., 129., 129., 129., 129., 129.,
            129., 355., 129., 129., 129., 129.,
        ],
        vec![
            32., 129., 129., 129., 129., 129., 129., 356., 129., 129., 129., 129., 161., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            32., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            32., 129., 129., 129., 129., 161., 129., 129., 129., 129., 194., 129., 129., 161.,
            129., 357., 129., 129., 129., 129.,
        ],
        vec![
            32., 129., 129., 129., 129., 129., 129., 129., 129., 355., 129., 129., 161., 161.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            32., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 161., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            32., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            64., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            96., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            96., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
        vec![
            96., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 129., 355., 129.,
            129., 129., 129., 129., 129., 129.,
        ],
    ];
    let width = bytes[0].len();
    let height = bytes.len();
    let bytes = bytes
        .into_iter()
        .flatten()
        .flat_map(|i| (i as f32).to_le_bytes().to_vec())
        .collect::<Vec<u8>>();
    Texture::from_code(
        device,
        queue,
        bytes,
        BatTexDimensions {
            width: width as u32,
            height: height as u32,
        },
        wgpu::ShaderStage::FRAGMENT,
        wgpu::TextureFormat::R32Float,
    )
}

pub fn upper<'a>(device: &'a Device, queue: &'a Queue) -> Texture<'a> {
    let bytes = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 129, 0, 0, 0, 0, 129, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 595, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 165, 0, 0, 165, 0, 6, 0, 165, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 197, 0, 0, 197, 0, 102, 0, 197, 0, 0, 0, 129, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 229, 0, 0, 229, 0, 0, 0, 229, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 595, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 165, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 197, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 165, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 197, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 229, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 257, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256,
        ],
        vec![
            0, 290, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324, 324,
            324, 291,
        ],
        vec![
            0, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322, 322,
            322, 323,
        ],
    ];
    let width = bytes[0].len();
    let height = bytes.len();
    let bytes = bytes
        .into_iter()
        .flatten()
        .flat_map(|i| (i as f32).to_le_bytes().to_vec())
        .collect::<Vec<u8>>();
    Texture::from_code(
        device,
        queue,
        bytes,
        BatTexDimensions {
            width: width as u32,
            height: height as u32,
        },
        wgpu::ShaderStage::FRAGMENT,
        wgpu::TextureFormat::R32Float,
    )
}
