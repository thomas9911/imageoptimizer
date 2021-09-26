use std::fs::File;
use std::io::Read;
use structopt::StructOpt;

pub use error::Error;
pub use formats::Format;
pub use imgopt_lib::error;
pub use imgopt_lib::formats;

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
    /// use the header of the file to determine the file format
    #[structopt(long)]
    use_header: bool,
    #[structopt(long, required_if("use_header", "true"))]
    output_format: Option<Format>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Args::from_args();

    if config.use_header {
        let input_format = check_file(&config.input)?;
        imgopt_lib::convert_explicit(
            (&config.input, input_format),
            (&config.output, config.output_format.unwrap()),
        )
    } else {
        imgopt_lib::convert(&config.input, &config.output)
    }
}

fn check_file(path: &str) -> Result<Format, Box<dyn std::error::Error>> {
    let mut buffer = [0; 64];
    {
        let mut handle = File::open(path)?.take(64);
        handle.read(&mut buffer)?;
    }
    Format::from_magic_bytes(&buffer).map_err(|e| e.into())
}
