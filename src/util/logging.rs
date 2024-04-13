use log::debug;
use simple_logger::{set_up_color_terminal, SimpleLogger};

pub fn init_logging(debug: bool) {
  let mut _debug = debug;
  if cfg!(test) {
    _debug = true;
  }
  eprintln!("initting logging: {debug}:{_debug}");
  // <logging>
  set_up_color_terminal();
  let logger = SimpleLogger::new();

  if _debug {
    log::set_max_level(log::LevelFilter::Debug);
  } else {
    log::set_max_level(log::LevelFilter::Info);
  }
  log::set_boxed_logger(Box::new(logger)).unwrap();
  // </logging>
  debug!("Debugging enabled.");
}
