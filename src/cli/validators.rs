/// Validate target String to have a valid value and also valid
/// OS names
pub fn is_valid_target(os_list: String) -> Result<(), String> {
  let values: Vec<String> = os_list.split(',').map(|s| s.to_string()).collect();

  for os in values.iter() {
    if os != "macos" && os != "linux" && os != "windows" {
      return Err(String::from("Provided target (s) are invalid. Valid targets are macos, linux, windows"));
    }
  }

  Ok(())
}
