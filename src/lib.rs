pub mod error;
pub mod formats;

pub use error::Error;
pub use formats::Format;

/// Main function that converts the input image to the output image
pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    use Format::*;

    match (Format::from_path(input)?, Format::from_path(output)?) {
        (Png, Png) => formats::png::convert(input, output)?,
        (Jpeg, Jpeg) => formats::jpeg::convert(input, output)?,
        (Jpeg, Png) => formats::png::convert_from_jpeg(input, output)?,
        (Png, Jpeg) => formats::jpeg::convert_from_png(input, output)?,
        (Png, Webp) => formats::webp::convert_from_png(input, output)?,
        (Jpeg, Webp) => formats::webp::convert_from_jpeg(input, output)?,
        (Svg, Svg) => formats::svg::convert(input, output)?,
        _ => return Err(Error::boxed("unsupported input output convertion".into())),
    }
    Ok(())
}
