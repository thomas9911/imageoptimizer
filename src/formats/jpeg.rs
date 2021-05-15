use crate::error::Error;
// use rgb::RGB8;

use std::fs::{read, write};

pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (pixels, width, height) = load(input)?;
    let jpeg_bytes = apply(pixels, width, height)?;
    save(output, jpeg_bytes)
}

pub fn apply(
    pixels: Vec<[u8; 3]>,
    width: usize,
    height: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let jpeg_bytes = std::panic::catch_unwind(|| {
        let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);

        comp.set_size(width, height);
        comp.set_mem_dest();
        comp.start_compress();

        let data: Vec<u8> = pixels
            .into_iter()
            .flat_map(|[a, b, c]| vec![a, b, c])
            .collect();

        comp.write_scanlines(&data[..]);

        comp.finish_compress();
        let jpeg_bytes = comp.data_to_vec().or(Err("unable to convert jpeg bytes"))?;
        Ok(jpeg_bytes)
    })
    .map_err(any_to_error)
    .and_then(std::convert::identity)?;

    Ok(jpeg_bytes)
}

pub fn load(path: &str) -> Result<(Vec<[u8; 3]>, usize, usize), Box<dyn std::error::Error>> {
    std::panic::catch_unwind(|| {
        let binary = read(path)?;
        let d = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_mem(&binary)?;

        let width = d.width();
        let height = d.height();
        let mut image = d.rgb()?;

        let pixels = image.read_scanlines().ok_or("unable to read jpeg bytes")?;
        assert!(image.finish_decompress());

        Ok((pixels, width, height))
    })
    .map_err(any_to_error)
    .and_then(std::convert::identity)
}

pub fn save(path: &str, jpeg_bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    write(path, jpeg_bytes)?;
    Ok(())
}

fn any_to_error(x: Box<dyn std::any::Any + std::marker::Send>) -> Box<dyn std::error::Error> {
    Box::new(Error::new(format!("{:?}", x)))
}
