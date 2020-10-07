/// picturs runtime error representation
#[derive(Debug)]
pub struct pictursError {
  pub message: String,
}

impl pictursError {
  pub fn new(message: &str) -> Self {
    pictursError {
      message: String::from(message),
    }
  }
}
