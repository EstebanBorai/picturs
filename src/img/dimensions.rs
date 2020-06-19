/// Dimensions represents the dimensions of an
/// image file
pub struct Dimensions {
  pub height: u32,
  pub width: u32,
}

impl Dimensions {
  pub fn new(width: u32, height: u32) -> Self {
    Dimensions {
      height,
      width
    }
  }
}
