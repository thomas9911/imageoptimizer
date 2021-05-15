use rgb::ComponentBytes;
use std::fs::write;
use webp::Encoder;

use crate::formats::{jpeg, png};
use crate::Error;

const WEBP_QUALITY: f32 = 80.0;

pub fn convert_from_png(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (info, data) = png::load(input)?;
    let img_data = apply(&data, info.width, info.height, info.color_type.samples())?;
    save(output, &img_data)
}

pub fn convert_from_jpeg(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (data, width, height) = jpeg::load(input)?;
    let img_data = apply(data.as_bytes(), width as u32, height as u32, 3)?;
    save(output, &img_data)
}

pub fn apply(
    data: &[u8],
    width: u32,
    height: u32,
    pixel_size: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if pixel_size == 3 {
        let data = Encoder::from_rgb(data, width, height)
            .encode(WEBP_QUALITY)
            .to_vec();
        return Ok(data);
    }

    if pixel_size == 4 {
        let data = Encoder::from_rgba(data, width, height)
            .encode(WEBP_QUALITY)
            .to_vec();
        return Ok(data);
    }

    Err(Error::boxed("currently unsupported by webp image".into()))
}

pub fn save(path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    write(path, data)?;
    Ok(())
}
