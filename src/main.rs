use structopt::StructOpt;

pub mod error;
pub mod formats;

#[derive(Debug, StructOpt)]
struct Args {
    input: String,
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Args::from_args();

    println!("start");
    // formats::jpeg::convert(&config.input, &config.output)?;
    formats::png::convert(&config.input, &config.output)?;
    println!("done");

    Ok(())
}
