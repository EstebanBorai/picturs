use crate::cli::{OS_LINUX, OS_OSX, OS_WINDOWS};

pub fn are_valid_os(os_list: String) -> Result<(), String> {
  let values: Vec<String> = os_list.split(',').map(|s| s.to_string()).collect();

  for os in values.iter() {
    if os != OS_OSX && os != OS_LINUX && os != OS_WINDOWS {
      return Err(String::from("Provided target (s) are invalid. Valid targets are osx, linux, windows"));
    }
  }

  Ok(())
}
