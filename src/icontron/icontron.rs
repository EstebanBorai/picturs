use std::path::Path;
use crate::cli::Cli;

pub struct Icontron<'a> {
  input_file_path: &'a Path,
  output_dir: &'a Path,
  target_os_list: Vec<String>,
}

impl<'a> Icontron<'a> {
  pub fn new(
    input_file_path: &'a Path,
    output_dir: &'a Path,
    target_os_list: Vec<String>
  ) -> Self {
    Icontron {
      input_file_path,
      output_dir,
      target_os_list
    }
  }
}

impl<'a> From<Cli<'a>> for Icontron<'_> {
  fn from(cli: Cli<'a>) -> Self {
    Icontron::new(
      &Path::new(&cli.file_path.clone()),
      &cli.output_dir,
      cli.target.clone()
    )
  }
}
