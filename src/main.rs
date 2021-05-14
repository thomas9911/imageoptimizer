use structopt::StructOpt;

pub mod error;
pub mod formats;
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
    let config = Args::from_args();

    match Format::from_path(&config.input)? {
        Format::Png => formats::png::convert(&config.input, &config.output)?,
        Format::Jpeg => formats::jpeg::convert(&config.input, &config.output)?,
    }

    Ok(())
}
