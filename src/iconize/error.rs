/// Iconize runtime error representation
pub struct IconizeError {
  pub message: String,
}

impl IconizeError {
  pub fn new(message: &str) -> Self {
    IconizeError {
      message: String::from(message)
    }
  }
}
