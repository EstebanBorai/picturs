use std::path::Path;
use image;
use image::GenericImageView;
use image::imageops::FilterType;

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

  pub fn resize(&self, square_side: u32) -> Result<(), image::error::ImageError> {
    self.validate(square_side);

    let image_file = image::open(self.path).unwrap();
    let resized = image_file.resize_exact(square_side, square_side, FilterType::Gaussian);
    let cwd = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let output_path = format!("{}/result.png", cwd);
    let target_path = Path::new(&output_path);

    resized.save(target_path)
  }
}
