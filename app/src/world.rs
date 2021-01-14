use crate::utils::{BatTex, BatTexDimensions};

#[allow(dead_code)]
pub fn procedural_tex(size: u32) -> BatTex {
    BatTex {
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        dim: BatTexDimensions {
            width: size,
            height: size,
        },
        bytes: (0..size * size)
            .flat_map(|i| vec![(i % 256) as u8, 0, 0, 0])
            .collect::<Vec<u8>>(),
    }
}

pub fn pix(i: u8) -> Vec<u8> {
    vec![i, 0, 0, 0]
}

pub fn mask_bit_tex() -> BatTex {
    let bytes = vec![
        vec![
            10, 11, 11, 11, 11, 14, 11, 11, 15, 16, 16, 17, 11, 11, 14, 11, 11, 11, 11, 13,
        ],
        vec![
            20, 21, 21, 21, 21, 24, 21, 21, 25, 26, 26, 27, 21, 21, 24, 21, 21, 21, 21, 23,
        ],
        vec![
            20, 31, 32, 32, 32, 34, 32, 32, 35, 36, 36, 37, 32, 32, 34, 32, 32, 32, 32, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 32, 63, 64, 32, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 82, 85, 85, 85, 85, 85, 85, 85, 83, 84, 85, 85, 85, 85, 85, 85, 85, 85, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            60, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 61,
        ],
        vec![
            16, 69, 69, 69, 69, 69, 69, 69, 69, 73, 74, 69, 69, 69, 69, 69, 69, 69, 69, 16,
        ],
        vec![
            17, 26, 26, 26, 26, 26, 26, 26, 26, 73, 74, 26, 26, 26, 26, 26, 26, 26, 26, 15,
        ],
        vec![
            70, 36, 36, 36, 36, 36, 36, 36, 36, 73, 74, 36, 36, 36, 36, 36, 36, 36, 36, 71,
        ],
        vec![
            20, 31, 32, 32, 32, 32, 32, 32, 32, 93, 74, 32, 32, 32, 32, 32, 32, 32, 32, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            20, 41, 42, 42, 42, 42, 42, 42, 42, 93, 74, 42, 42, 42, 42, 42, 42, 42, 42, 33,
        ],
        vec![
            50, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 53,
        ],
    ];
    let width = bytes[0].len();
    let height = bytes.len();
    BatTex {
        dim: BatTexDimensions {
            width: width as u32,
            height: height as u32,
        },
        format: wgpu::TextureFormat::Rgba8Unorm,
        bytes: bytes
            .into_iter()
            .flatten()
            .flat_map(|i| pix(i))
            .collect::<Vec<u8>>(),
    }
}

pub fn image_tex(data: &[u8]) -> BatTex {
    let image = image::load_from_memory(data).unwrap().into_rgba8();
    BatTex {
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        dim: BatTexDimensions {
            width: image.width(),
            height: image.height(),
        },
        bytes: image.into_raw(),
    }
}
