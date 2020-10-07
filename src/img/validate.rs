use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use image;
use image::ico::ICOEncoder;
use image::imageops::FilterType;
use image::png::PNGEncoder;
use image::{DynamicImage, GenericImageView};
use crate::error::pictursError;
use crate::img::Dimensions;

/// Validate the provided file dimensions borrowing a `Dimensions`
/// instance for the provided files
fn validate(dim: &'a Dimensions) -> Result<(), pictursError> {
  if dim.height != dim.width {
    return Err(pictursError::new(&format!(
      "The current file is dimensions are invalid, {}x{}. Expected a 1:1 aspect image",
      dim.width, dim.height
    )));
  }

  if target_os_list.iter().any(|os| os == OS_OSX) {
    validate_macos(dim)?;
  }

  if target_os_list.iter().any(|os| os == OS_LINUX) {
    validate_linux(dim)?;
  }

  if target_os_list.iter().any(|os| os == OS_WINDOWS) {
    validate_windows(dim)?;
  }

  Ok(())
}

/// Validates the borrowed `Dimensions` for a MacOS icon file
fn validate_macos(dim: &'a Dimensions) -> Result<(), pictursError> {
  if dim.width < 512 {
    return Err(pictursError::new(
      &format!("The current file is dimensions are invalid for macOS, {}x{}. Expected a image greather than or equal to 512x512 dimensions image", dim.width, dim.height)
    ));
  }

  Ok(())
}

/// Validates the borrowed `Dimensions` for a Linux icon file
fn validate_linux(dim: &'a Dimensions) -> Result<(), pictursError> {
  if dim.width < 16 {
    return Err(pictursError::new(
      &format!("The current file is dimensions are invalid for Linux, {}x{}. Expected a image greather than 16x16 dimensions image",
        dim.width, dim.height)
    ));
  }

  Ok(())
}

/// Validates the borrowed `Dimensions` for a Windows icon file
fn validate_windows(dim: &'a Dimensions) -> Result<(), pictursError> {
  if dim.width < 256 {
    return Err(pictursError::new(
      &format!("The current file is dimensions are invalid for macOS, {}x{}. Expected a image greather than or equal to 256x256 dimensions image", dim.width, dim.height)
    ));
  }

  Ok(())
}
