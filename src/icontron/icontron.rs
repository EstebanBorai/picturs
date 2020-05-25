use std::error::Error;
use std::path::Path;
use image;
use image::{open, RgbImage, ColorType};
use image::ico::ICOEncoder;
use image::png::PNGEncoder;
use crate::icontron::{Dimensions, IcontronError};
use crate::cli::{OS_LINUX, OS_OSX, OS_WINDOWS};
use std::fs::File;
use std::io::BufWriter;

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
    let img = self.load_input_file();
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

  fn load_input_file(&self) -> RgbImage {
    open(self.input_file_path).unwrap().into_rgb()
  }

  fn get_image_dimensions(&self, img: &RgbImage) -> Dimensions {
    Dimensions::new(
      img.dimensions().0,
      img.dimensions().1,
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

  fn build_targets(&self, img: RgbImage) {
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

  fn encode_ico(&self, img: &RgbImage) -> Result<(), IcontronError> {
    let file = File::create("icon.ico").unwrap();
    let ref mut buff = BufWriter::new(file);
    let encoder = ICOEncoder::new(buff);

    match encoder.encode(img, 256, 256, ColorType::Rgb16) {
      Ok(_) => Ok(()),
      Err(err) => Err(IcontronError::new(err.description()))
    }
  }

  fn encode_png(&self, img: &RgbImage) -> Result<(), IcontronError> {
    let file = File::create("icon.png").unwrap();
    let ref mut buff = BufWriter::new(file);
    let encoder = PNGEncoder::new(buff);

    match encoder.encode(img, 256, 256, ColorType::Rgb16) {
      Ok(_) => Ok(()),
      Err(err) => Err(IcontronError::new(err.description()))
    }
  }

  fn encode_icns(&self) -> Result<(), IcontronError> {
    Err(IcontronError::new("ICNS (macOS icon format) is not supported yet!"))
  }
}
