use clap::Parser;

use log::debug;

use bundle_schema::util::{inputs, logging};

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
  #[arg(short = 'o', long, value_name = "OUTPUT_FILE")]
  /// The output file to use for the bundled output.
  output: Option<String>,

  #[arg(short = 'i', long, value_name = "INPUT FILES")]
  /// The input files to use in bundling. Use the flag multiple times for multiple files, like `-i foo.json -i bar.json`.
  input: Vec<String>,

  #[arg(short=None, long)]
  /// Output debug information
  debug: bool,
}

fn main() {
  let opts = CliArgs::parse();

  logging::init_logging(opts.debug);

  debug!("Args: {opts:#?}");

  let input_details = inputs::parse_inputs(opts.input);

  debug!("Inputs: {input_details:#?}");
}
