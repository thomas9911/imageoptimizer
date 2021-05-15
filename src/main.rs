use structopt::StructOpt;

pub mod error;
pub mod formats;
pub use error::Error;
pub use formats::Format;

#[derive(Debug, StructOpt)]
/// cli wrapper around image optimizers formats
///
/// Input format should be the same as the output format
///
/// Supported formats: [Jpeg, Png]
///
///
struct Args {
    /// input file
    input: String,
    /// output file
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use Format::*;
    let config = Args::from_args();

    match (
        Format::from_path(&config.input)?,
        Format::from_path(&config.output)?,
    ) {
        (Png, Png) => formats::png::convert(&config.input, &config.output)?,
        (Jpeg, Jpeg) => formats::jpeg::convert(&config.input, &config.output)?,
        (Jpeg, Png) => formats::png::convert_from_jpeg(&config.input, &config.output)?,
        (Png, Jpeg) => formats::jpeg::convert_from_png(&config.input, &config.output)?,
        (Png, Webp) => formats::webp::convert_from_png(&config.input, &config.output)?,
        (Jpeg, Webp) => formats::webp::convert_from_jpeg(&config.input, &config.output)?,
        (Svg, Svg) => formats::svg::convert(&config.input, &config.output)?,
        _ => return Err(Error::boxed("unsupported input output convertion".into())),
    }

    Ok(())
}
