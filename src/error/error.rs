/// Iconize runtime error representation
#[derive(Debug)]
pub struct IconizeError {
  pub message: String,
}

impl IconizeError {
  pub fn new(message: &str) -> Self {
    IconizeError {
      message: String::from(message),
    }
  }
}
