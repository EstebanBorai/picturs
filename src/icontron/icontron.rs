use std::error::Error;
use std::path::Path;
use image;
use image::GenericImageView;
use image::imageops::FilterType;
use crate::icontron::{Dimensions, IcontronError};
use crate::cli::{OS_LINUX, OS_OSX, OS_WINDOWS};

pub struct Icontron<'a> {
  input_file_path: &'a Path,
  output_dir: &'a Path,
  target_os_list: Vec<String>,
}

impl<'a> Icontron<'a> {
  pub fn new(
    input_file_path: &'a Path,
    output_dir: &'a Path,
    target_os_list: Vec<String>
  ) -> Self {
    Icontron {
      input_file_path,
      output_dir,
      target_os_list
    }
  }

  pub fn bake(&self) {
    let image_dimensions: Dimensions = self.get_image_dimensions();

    match self.validate(&image_dimensions) {
      Ok(_) => {
        println!("Image is ok!");
      },
      Err(err) => {
        println!("{}", err.message)
      }
    }
  }

  fn get_image_dimensions(&self) -> Dimensions {
    let image_file = image::open(self.input_file_path).unwrap();

    Dimensions::new(
      image_file.dimensions().0,
      image_file.dimensions().1,
    )
  }

  fn validate(&self, dim: &'a Dimensions) -> Result<(), IcontronError> {
    if dim.height != dim.width {
      return Err(IcontronError::new(
        &format!("The current file is dimensions are invalid, {}x{}. Expected a 1:1 aspect image", dim.width, dim.height)
      ));
    }

    if self.target_os_list.iter().any(|os| os == OS_OSX) {
      self.validate_macos(dim)?;
    }

    if self.target_os_list.iter().any(|os| os == OS_LINUX) {
      self.validate_linux(dim)?;
    }

    if self.target_os_list.iter().any(|os| os == OS_WINDOWS) {
      self.validate_windows(dim)?;
    }

    Ok(())
  }

  fn validate_macos(&self, dim: &'a Dimensions) -> Result<(), IcontronError> {
    if dim.width < 512 {
      return Err(IcontronError::new(
        &format!("The current file is dimensions are invalid for macOS, {}x{}. Expected a image greather than or equal to 512x512 dimensions image", dim.width, dim.height)
      ));
    }

    Ok(())
  }

  fn validate_linux(&self, dim: &'a Dimensions) -> Result<(), IcontronError> {
    if dim.width < 16 {
      return Err(IcontronError::new(
        &format!("The current file is dimensions are invalid for Linux, {}x{}. Expected a image greather than 16x16 dimensions image",
          dim.width, dim.height)
      ));
    }

    Ok(())
  }

  fn validate_windows(&self, dim: &'a Dimensions) -> Result<(), IcontronError> {
    if dim.width < 256 {
      return Err(IcontronError::new(
        &format!("The current file is dimensions are invalid for macOS, {}x{}. Expected a image greather than or equal to 256x256 dimensions image", dim.width, dim.height)
      ));
    }

    Ok(())
  }
}
