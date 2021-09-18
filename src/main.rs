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
/// Supported formats: [Jpeg, Png, Svg, Webp]
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

    imgopt_lib::convert(&config.input, &config.output)
}
