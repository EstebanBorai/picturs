use clap::{App, Arg};
use crate::img::ImageRef;

pub struct Cli {
  pub file_path: String,
}

impl Cli {
  /// Creates a new Cli instance
  /// with default values
  pub fn new() -> Self {
    Cli {
      file_path: String::from(""),
    }
  }

  pub fn read(&mut self) -> Self {
    let mut cli = Cli {
      file_path: "".to_string()
    };

    let matches = App::new("icontron")
      .version("0.1.0")
      .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai")
      .arg(
        Arg::with_name("file_path")
          .help("Image file to create the icons from")
          .takes_value(true)
          .short("f")
          .long("file")
          .value_name("FILE")
          .required(true)
      )
      .get_matches();

    if let Some(file) = matches.value_of("file_path") {
      cli.file_path = file.to_string();
    } else {
      panic!("Missing required paremeter \"file\"!");
    }

    cli
  }

  pub fn work(&mut self) {
    let app = self.read();
    let input_file = ImageRef::new(&app.file_path);

    input_file.resize(100);
  }
}
