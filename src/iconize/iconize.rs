use std::error::Error;
use std::path::Path;
use image;
use image::{open, RgbImage, ColorType, DynamicImage, GenericImageView};
use image::imageops::FilterType;
use image::ico::ICOEncoder;
use image::png::PNGEncoder;
use crate::iconize::{Dimensions, IconizeError};
use crate::cli::{OS_LINUX, OS_OSX, OS_WINDOWS};
use std::fs::File;
use std::io::BufWriter;

/// Iconize creates icons files from the provided input
/// `input_file_path`, and writes them in the provided `output_dir`.
pub struct Iconize<'a> {
  input_file_path: &'a Path,
  output_dir: &'a Path,
  target_os_list: Vec<String>,
}

impl<'a> Iconize<'a> {
  pub fn new(
    input_file_path: &'a Path,
    output_dir: &'a Path,
    target_os_list: Vec<String>
  ) -> Self {
    Iconize {
      input_file_path,
      output_dir,
      target_os_list
    }
  }

  /// Encodes the `input_file_path` file into
  /// the `target_os_list` files and writes them.
  /// If the provided file is not valid for its target
  /// panics.
  pub fn bake(&self) {
    let loaded_file = self.load_input_file();
    let img: DynamicImage = loaded_file;

    let image_dimensions: Dimensions = self.get_image_dimensions(&img);

    match self.validate(&image_dimensions) {
      Ok(_) => {
        self.build_targets(img);
      },
      Err(err) => {
        panic!(err.message);
      }
    }
  }

  /// Returns a DynamicImage for the provided
  /// file.
  fn load_input_file(&self) -> DynamicImage {
    open(self.input_file_path).unwrap()
  }

  /// Gathers the DynamicImage dimensions, returning
  /// a `Dimensions` instance
  fn get_image_dimensions(&self, img: &DynamicImage) -> Dimensions {
    Dimensions::new(
      img.dimensions().0,
      img.dimensions().1,
    )
  }

  /// Validate the provided file dimensions borrowing a `Dimensions`
  /// instance for the provided files
  fn validate(&self, dim: &'a Dimensions) -> Result<(), IconizeError> {
    if dim.height != dim.width {
      return Err(IconizeError::new(
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

  /// Validates the borrowed `Dimensions` for a MacOS icon file
  fn validate_macos(&self, dim: &'a Dimensions) -> Result<(), IconizeError> {
    if dim.width < 512 {
      return Err(IconizeError::new(
        &format!("The current file is dimensions are invalid for macOS, {}x{}. Expected a image greather than or equal to 512x512 dimensions image", dim.width, dim.height)
      ));
    }

    Ok(())
  }

  /// Validates the borrowed `Dimensions` for a Linux icon file
  fn validate_linux(&self, dim: &'a Dimensions) -> Result<(), IconizeError> {
    if dim.width < 16 {
      return Err(IconizeError::new(
        &format!("The current file is dimensions are invalid for Linux, {}x{}. Expected a image greather than 16x16 dimensions image",
          dim.width, dim.height)
      ));
    }

    Ok(())
  }

  /// Validates the borrowed `Dimensions` for a Windows icon file
  fn validate_windows(&self, dim: &'a Dimensions) -> Result<(), IconizeError> {
    if dim.width < 256 {
      return Err(IconizeError::new(
        &format!("The current file is dimensions are invalid for macOS, {}x{}. Expected a image greather than or equal to 256x256 dimensions image", dim.width, dim.height)
      ));
    }

    Ok(())
  }

  /// Encode each of the provided targets
  fn build_targets(&self, img: DynamicImage) {
    if self.target_os_list.iter().any(|os| os == OS_WINDOWS) {
      self.encode_ico(&img);
    }

    if self.target_os_list.iter().any(|os| os == OS_LINUX) {
      self.encode_png(&img);
    }

    if self.target_os_list.iter().any(|os| os == OS_OSX) {
      self.encode_icns();
    }
  }

  /// Encode an ICO file
  fn encode_ico(&self, img: &DynamicImage) -> Result<(), IconizeError> {
    if img.dimensions().0 != 256 {
      let resized = img.resize_exact(256, 256, FilterType::Gaussian);

      return self.encode_ico(&resized);
    }

    let file = File::create("icon.ico").unwrap();
    let ref mut buff = BufWriter::new(file);
    let encoder = ICOEncoder::new(buff);

    match encoder.encode(&img.to_bytes(), img.dimensions().0, img.dimensions().1, img.color()) {
      Ok(_) => Ok(()),
      Err(err) => Err(IconizeError::new(err.description()))
    }
  }

  /// Encode a PNG file
  fn encode_png(&self, img: &DynamicImage) -> Result<(), IconizeError> {
    if img.dimensions().0 != 256 {
      let resized = img.resize_exact(256, 256, FilterType::Gaussian);

      return self.encode_png(&resized);
    }

    let file = File::create("icon.png").unwrap();
    let ref mut buff = BufWriter::new(file);
    let encoder = PNGEncoder::new(buff);

    match encoder.encode(&img.to_bytes(), img.dimensions().0, img.dimensions().1, img.color()) {
      Ok(_) => Ok(()),
      Err(err) => Err(IconizeError::new(err.description()))
    }
  }

  /// Encode an ICNS file
  fn encode_icns(&self) -> Result<(), IconizeError> {
    Err(IconizeError::new("ICNS (macOS icon format) is not supported yet!"))
  }
}
