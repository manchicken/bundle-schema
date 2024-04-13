use log::{debug, error};
use std::fs::File;
use std::io::BufReader;

fn parse_one_file(fname: String) -> Option<serde_json::Value> {
  debug!("Parsing file «{fname}»");

  let fh = match File::open(fname.clone()) {
    Err(e) => {
      error!("Failed to parse file «{fname}»: {e:#?}");
      return None;
    }
    Ok(fh) => fh,
  };
  let reader = BufReader::new(fh);

  match serde_json::from_reader(reader) {
    Err(e) => {
      error!("Failed to parse «{fname}»: {e:#?}");
      None
    }
    Ok(val) => Some(val),
  }
}

pub fn parse_inputs(input_files: Vec<String>) -> Vec<serde_json::Value> {
  let all_of_them: Vec<serde_json::Value> =
    input_files.into_iter().filter_map(parse_one_file).collect();

  all_of_them
}
