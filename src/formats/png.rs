use rgb::{ComponentBytes, FromSlice};
use rgb::{RGB8, RGBA8};

use crate::error::Error;
use std::fs::{read, File};
use std::io::BufWriter;

pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (info, png_image) = load(input)?;
    let (palette, pixels) = apply(&info, png_image)?;
    save(
        output,
        palette,
        pixels,
        info.width as usize,
        info.height as usize,
    )
}

pub fn apply(
    info: &png::OutputInfo,
    png_image: Vec<u8>,
) -> Result<(Vec<RGBA8>, Vec<u8>), Box<dyn std::error::Error>> {
    let width = info.width as usize;
    let height = info.height as usize;
    let pixel_size = info.color_type.samples();

    let mut liq = imagequant::new();
    // liq.set_speed(1);

    let pixels = convert_to_pixels(png_image, pixel_size)?;

    let mut image = liq.new_image(&pixels[..], width, height, 0.0)?;

    let mut res = liq.quantize(&image)?;

    res.set_dithering_level(1.0);

    let (palette, pixels) = res.remapped(&mut image)?;
    Ok((palette, pixels))
}

pub fn save(
    path: &str,
    palette: Vec<RGBA8>,
    pixels: Vec<u8>,
    width: usize,
    height: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);
    let new_palette: Vec<RGB8> = palette.iter().map(|x| x.rgb()).collect();
    let transparent: Vec<u8> = palette.iter().map(|x| x.a).collect();

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_palette(new_palette.as_bytes().to_vec());
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_trns(transparent);

    let mut writer = encoder.write_header()?;

    writer.write_image_data(&pixels)?;

    Ok(())
}

pub fn load(
    path: &str,
) -> Result<(png::OutputInfo, std::vec::Vec<u8>), Box<dyn std::error::Error>> {
    let data = read(path)?;
    let decoder = png::Decoder::new(&data[..]);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    Ok((info, buf))
}

fn convert_to_pixels<'a>(
    data: Vec<u8>,
    pixel_size: usize,
) -> Result<Vec<RGBA8>, Box<dyn std::error::Error>> {
    if pixel_size == 3 {
        return Ok(data
            .as_slice()
            .as_rgb()
            .into_iter()
            .map(|x| x.alpha(255))
            .collect());
    }
    if pixel_size == 4 {
        return Ok(data.as_slice().as_rgba().into_iter().copied().collect());
    }

    return Err(Error::boxed("png format not supported".into()));
}
