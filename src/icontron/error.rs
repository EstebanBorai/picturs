pub struct IcontronError {
  pub message: String,
}

impl IcontronError {
  pub fn new(message: &str) -> Self {
    IcontronError {
      message: String::from(message)
    }
  }
}
