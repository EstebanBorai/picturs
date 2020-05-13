use std::path::Path;
use image;
use image::GenericImageView;

use crate::img::Dimensions;

pub struct ImageRef<'a> {
  pub path: &'a Path,
  pub dimensions: Dimensions,
}

impl<'a> ImageRef<'a> {
  pub fn new(path: &'a str) -> Self {
    let path: &'a Path = Path::new(path);
    let image_file = image::open(path).unwrap();
    let dimensions = Dimensions::new(
      image_file.dimensions().0,
      image_file.dimensions().0
    );
  
    ImageRef {
      path,
      dimensions,
    }
  }

  fn validate(&self, square_side: u32) {
    if self.dimensions.width < square_side || self.dimensions.height < square_side {
      panic!(
        format!("Invalid image dimensions, expected {}x{}. Current image dimensions: {}x{}.",
          square_side, square_side, self.dimensions.width, self.dimensions.height)
      )
    }
  }

  pub fn resize(&self, square_side: u32) {
    self.validate(square_side);

    println!("{}x{}", self.dimensions.width, self.dimensions.height);
  }
}
