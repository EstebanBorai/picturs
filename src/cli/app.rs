use clap::{App, Arg, ArgMatches};
use std::vec::Vec;
use std::path::Path;
use crate::cli::{are_valid_os};
use crate::icontron::Icontron;

pub struct Cli<'a> {
  pub file_path: &'a Path,
  pub output_dir: &'a Path,
  pub target: Vec<String>,
}

impl<'a> Cli<'a> {
  /// Creates a new Cli instance
  /// with default values
  pub fn new(file_path: &'a Path, target: Vec<String>, output_dir: &'a Path) -> Self {
    Cli {
      file_path,
      target,
      output_dir,
    }
  }

  pub fn start() {
    let matches = Cli::make_cli_and_run();
    let params = Cli::build_from_matches(&matches);
    let icontron = Icontron::new(
      params.file_path,
      params.output_dir,
      params.target
    );

    icontron.bake();
  }

  fn make_cli_and_run() -> ArgMatches<'static> {
    let app = App::new("icontron")
      .version("0.1.0")
      .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)")
      .arg(
        Arg::with_name("input_file")
          .help("Image file to create the icons from")
          .takes_value(true)
          .short("f")
          .long("file")
          .value_name("FILE")
          .required(true)
      )
      .arg(
        Arg::with_name("targets")
          .help("Target OS icon files")
          .takes_value(true)
          .short("t")
          .long("target")
          .value_name("TARGET_LIST")
          .required(false)
          .validator(are_valid_os)
      )
      .arg(
        Arg::with_name("output_dir")
          .help("Path to directory where files will be written")
          .takes_value(true)
          .short("o")
          .long("output")
          .value_name("OUTPUT_DIRECTORY")
          .required(false)
      );

      app.get_matches()
  }

  fn build_from_matches(arg_matches: &'a ArgMatches<'a>) -> Self {
    let file_path: &Path;
    let input_targets: Vec<String>;
    let output_dir: &Path;

    if let Some(file) = arg_matches.value_of("input_file") {
      file_path = Path::new(file);
    } else {
      panic!("Missing required paremeter \"file\"!");
    }

    if let Some(target) = arg_matches.value_of("targets") {
      input_targets = Cli::parse_targets(target);
    } else {
      input_targets = Cli::make_target_list_all();
    }

    if let Some(output_d) = arg_matches.value_of("output_dir") {
      output_dir = Path::new(output_d);
    } else {
      output_dir = Path::new("./icons");
    }

    Cli::new(file_path, input_targets, output_dir)
  }

  fn parse_targets(input_target_string: &str) -> Vec<String> {
    let mut targets: Vec<String> = Vec::new();

    let values: Vec<&str> = input_target_string.split(',').collect();

    for target in values.iter() {
      targets.push(target.to_string());
    }

    return targets;
  }

  fn make_target_list_all() -> Vec<String> {
    vec!(
      "osx".to_string(),
      "linux".to_string(),
      "windows".to_string()
    )
  }
}
