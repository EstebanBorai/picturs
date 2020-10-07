use crate::cli::is_valid_target;
use crate::files::create_dir_if_not_exists;
use crate::img::encode_from_cli_args;
use clap::{App, Arg, crate_version};
use std::fs;
use std::path::Path;

const INPUT_FILE: &str = "input_file";
const TARGET: &str = "target";
const OUTPUT_PATH: &str = "output_path";

pub type Targets = Vec<String>;

/// Creates a Clap App instance and binds arguments,
/// then gather matches from the instance during the
/// CLI execution
pub fn run() {
  let app = App::new("picturs")
    .version(crate_version!())
    .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai)")
    .arg(
      Arg::with_name(INPUT_FILE)
        .help("File used to generate icons from")
        .takes_value(true)
        .value_name(INPUT_FILE)
        .required(true),
    )
    .arg(
      Arg::with_name(TARGET)
        .help("Target OS to build icons for. Eg: \"osx, linux, windows\" or \"osx\"")
        .takes_value(true)
        .short("t")
        .long(TARGET)
        .value_name(TARGET)
        .required(true)
        .validator(is_valid_target),
    )
    .arg(
      Arg::with_name(OUTPUT_PATH)
        .help("Output drectory for icons")
        .takes_value(true)
        .short("o")
        .long(OUTPUT_PATH)
        .value_name(OUTPUT_PATH)
        .required(false),
    );

  let matches = app.get_matches();

  build_from_args(
    matches.value_of(INPUT_FILE).unwrap().to_string(),
    get_targets(matches.value_of(TARGET).unwrap().to_string()),
    get_output_path(matches.value_of(OUTPUT_PATH)),
  );
}

fn get_targets(target_list: String) -> Targets {
  if target_list == String::default() {
    // if no target is defines from the CLI
    // use all targets
    return vec![
      "macos".to_string(),
      "linux".to_string(),
      "windows".to_string(),
    ];
  }

  target_list.split(",").map(|str| str.to_string()).collect()
}

fn get_output_path(output_path: Option<&str>) -> String {
  let final_path = match output_path {
    Some(path_str) => path_str.to_string(),
    _ => Path::new("./icons").to_str().unwrap().to_string(),
  };

  // check if the provided path is available
  create_dir_if_not_exists(final_path.clone());

  return final_path;
}

fn build_from_args(input_file: String, targets: Targets, output_path: String) {
  encode_from_cli_args(input_file, targets, output_path);
}
