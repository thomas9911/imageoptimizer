pub mod error;
pub mod formats;

pub use error::Error;
pub use formats::Format;

/// Main function that converts the input image to the output image. Detects the image format based on the file extension
pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    convert_explicit(
        (input, Format::from_path(input)?),
        (output, Format::from_path(output)?),
    )
}

/// Main function that converts the input image to the output image. The format of each image need to be specified.
pub fn convert_explicit(
    input: (&str, Format),
    output: (&str, Format),
) -> Result<(), Box<dyn std::error::Error>> {
    use Format::*;

    match (input.1, output.1) {
        (Png, Png) => formats::png::convert(input.0, output.0)?,
        (Jpeg, Jpeg) => formats::jpeg::convert(input.0, output.0)?,
        (Jpeg, Png) => formats::png::convert_from_jpeg(input.0, output.0)?,
        (Png, Jpeg) => formats::jpeg::convert_from_png(input.0, output.0)?,
        (Png, Webp) => formats::webp::convert_from_png(input.0, output.0)?,
        (Jpeg, Webp) => formats::webp::convert_from_jpeg(input.0, output.0)?,
        (Svg, Svg) => formats::svg::convert(input.0, output.0)?,
        _ => return Err(Error::boxed("unsupported input output convertion".into())),
    }
    Ok(())
}
