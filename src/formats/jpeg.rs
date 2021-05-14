use crate::error::Error;
// use rgb::RGB8;

use std::fs::{read, write};

pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    run(input, output)
}

fn run(path: &str, out: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (pixels, width, height) = load(path)?;

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
        let jpeg_bytes = comp.data_to_vec().or(Err("broken:("))?;
        Ok(jpeg_bytes)
    })
    .map_err(any_to_error)
    .and_then(std::convert::identity)?;

    write(out, jpeg_bytes)?;
    Ok(())
}

fn load(path: &str) -> Result<(Vec<[u8; 3]>, usize, usize), Box<dyn std::error::Error>> {
    std::panic::catch_unwind(|| {
        let binary = read(path)?;
        let d = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_mem(&binary)?;

        let width = d.width();
        let height = d.height();
        let mut image = d.rgb()?;

        let pixels = image.read_scanlines().ok_or("broken:(")?;
        assert!(image.finish_decompress());

        Ok((pixels, width, height))
    })
    .map_err(any_to_error)
    .and_then(std::convert::identity)
}

fn any_to_error(x: Box<dyn std::any::Any + std::marker::Send>) -> Box<dyn std::error::Error> {
    Box::new(Error::new(format!("{:?}", x)))
}
