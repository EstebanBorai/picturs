use clap::{App, Arg};
use std::vec::Vec;
use crate::img::ImageRef;
use crate::cli::{are_valid_os};

pub struct Cli {
  pub file_path: String,
  pub target: Vec<String>,
}

impl Cli {
  /// Creates a new Cli instance
  /// with default values
  pub fn new() -> Self {
    Cli {
      file_path: String::from(""),
      target: Vec::new(),
    }
  }

  pub fn read(&mut self) -> Self {
    let mut cli = Cli {
      file_path: "".to_string(),
      target: Vec::new(),
    };

    let matches = App::new("icontron")
      .version("0.1.0")
      .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)")
      .arg(
        Arg::with_name("file_path")
          .help("Image file to create the icons from")
          .takes_value(true)
          .short("f")
          .long("file")
          .value_name("FILE")
          .required(true)
      )
      .arg(
        Arg::with_name("target")
          .help("Target OS icon files")
          .takes_value(true)
          .short("t")
          .long("target")
          .value_name("TARGET_LIST")
          .required(false)
          .validator(are_valid_os)
      )
      .get_matches();

    if let Some(file) = matches.value_of("file_path") {
      cli.file_path = file.to_string();
    } else {
      panic!("Missing required paremeter \"file\"!");
    }

    if let Some(target) = matches.value_of("target") {
      let input_target = String::from(target);
      let values: Vec<&str> = input_target.split(',').collect();

      for targ in values.iter() {
        cli.target.push(targ.to_string());
      }
    } else {
      cli.target.push("osx".to_string());
      cli.target.push("linux".to_string());
      cli.target.push("windows".to_string());
    }

    for targ in cli.target.iter() {
      println!("{}", targ);
    }

    cli
  }

  pub fn work(&mut self) {
    let app = self.read();
    let input_file = ImageRef::new(&app.file_path);

    match input_file.resize(250) {
      Ok(v) => v,
      Err(e) => panic!("An error ocurred!\n{}", e),
    }
  }
}
