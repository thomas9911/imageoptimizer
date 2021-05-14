use rgb::{ComponentBytes, FromSlice};
use rgb::{RGB8, RGBA8};

use std::fs::{read, File};
use std::io::BufWriter;

pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    run(input, output)
}

fn load(path: &str) -> Result<(png::OutputInfo, std::vec::Vec<u8>), Box<dyn std::error::Error>> {
    let data = read(path)?;
    let decoder = png::Decoder::new(&data[..]);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    Ok((info, buf))
}

fn convert_to_pixels<'a>(data: Vec<u8>, _: usize) -> Vec<RGBA8> {
    data.as_slice()
        .as_rgb()
        .into_iter()
        .map(|x| x.alpha(255))
        .collect()
}

fn run(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (info, png_image) = load(input)?;

    let width = info.width as usize;
    let height = info.height as usize;
    let pixel_size = info.color_type.samples();

    let mut liq = imagequant::new();
    liq.set_speed(5);
    liq.set_quality(70, 99);

    let pixels = convert_to_pixels(png_image, pixel_size);

    let mut image = liq.new_image(&pixels[..], width, height, 0.0).unwrap();

    let mut res = liq.quantize(&image)?;

    res.set_dithering_level(1.0);

    let (palette, pixels) = res.remapped(&mut image).unwrap();

    save(output, palette, pixels, width, height)?;
    Ok(())
}

fn save(
    path: &str,
    palette: std::vec::Vec<RGBA8>,
    pixels: Vec<u8>,
    width: usize,
    height: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);
    let new_palette: Vec<RGB8> = palette.into_iter().map(|x| x.rgb()).collect();

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_palette(new_palette.as_bytes().to_vec());
    encoder.set_color(png::ColorType::Indexed);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&pixels)?;

    Ok(())
}
