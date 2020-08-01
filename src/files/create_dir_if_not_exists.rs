use std::fs::create_dir;
use std::path::Path;

/// Create a directory if not exists, otherwise does nothing
pub fn create_dir_if_not_exists(dir_path: String) {
  if !Path::new(&dir_path).exists() {
    create_dir(&dir_path).expect(&format!("Unable to create {} directory!", dir_path));
  }
}
