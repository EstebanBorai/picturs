use crate::cli::Targets;
use crate::error::pictursError;
use crate::files::create_dir_if_not_exists;
use icns::{IconFamily, IconType, Image};
use image;
use image::ico::ICOEncoder;
use image::imageops::FilterType;
use image::io::Reader;
use image::png::PNGEncoder;
use image::{DynamicImage, GenericImageView};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn encode_from_cli_args(input_file_path: String, targets: Targets, output_directory: String) {
  let image_file = Reader::open(input_file_path.clone())
    .expect("Unable to read image file")
    .decode()
    .expect("Unable to decode image file");

  if targets.iter().any(|t| t == "windows") {
    encode_ico(&image_file, output_directory.clone()).expect("Unable to encode into an ICO file");
  }

  if targets.iter().any(|t| t == "linux") {
    encode_png(&image_file, output_directory.clone()).expect("Unable to encode into a PNG file");
  }

  if targets.iter().any(|t| t == "macos") {
    encode_icns(input_file_path.clone(), output_directory.clone())
      .expect("Unable to encode into ICNS file");
  }
}

/// Encode an ICO file
fn encode_ico(img: &DynamicImage, output_directory: String) -> Result<(), pictursError> {
  if img.dimensions().0 != 256 {
    let resized = img.resize_exact(256, 256, FilterType::Gaussian);

    return encode_ico(&resized, output_directory);
  }

  let file_dirname = format!("{}/windows", output_directory);
  // make sure the directory exists in the output directory
  create_dir_if_not_exists(file_dirname.clone());

  let file = File::create(format!("{}/icon.ico", file_dirname)).unwrap();
  let ref mut buff = BufWriter::new(file);
  let encoder = ICOEncoder::new(buff);

  match encoder.encode(
    &img.to_bytes(),
    img.dimensions().0,
    img.dimensions().1,
    img.color(),
  ) {
    Ok(_) => Ok(()),
    Err(err) => Err(pictursError::new(&err.to_string())),
  }
}

/// Encode a PNG file
fn encode_png(img: &DynamicImage, output_directory: String) -> Result<(), pictursError> {
  if img.dimensions().0 != 256 {
    let resized = img.resize_exact(256, 256, FilterType::Gaussian);

    return encode_png(&resized, output_directory);
  }

  let file_dirname = format!("{}/linux", output_directory);
  // make sure the directory exists in the output directory
  create_dir_if_not_exists(file_dirname.clone());

  let file = File::create(format!("{}/icon.png", file_dirname)).unwrap();
  let ref mut buff = BufWriter::new(file);
  let encoder = PNGEncoder::new(buff);

  match encoder.encode(
    &img.to_bytes(),
    img.dimensions().0,
    img.dimensions().1,
    img.color(),
  ) {
    Ok(_) => Ok(()),
    Err(err) => Err(pictursError::new(&err.to_string())),
  }
}

/// Encode an ICNS file
fn encode_icns(input_file_path: String, output_directory: String) -> Result<(), pictursError> {
  // create new ICNS family
  let mut icon_family = IconFamily::new();
  let file = BufReader::new(File::open(input_file_path.clone()).unwrap());
  let image = Image::read_png(file).unwrap();

  icon_family.add_icon(&image).unwrap();

  let file_dirname = format!("{}/macos", output_directory);
  // make sure the directory exists in the output directory
  create_dir_if_not_exists(file_dirname.clone());

  let out = BufWriter::new(File::create(format!("{}/icon.icns", file_dirname)).unwrap());
  match icon_family.write(out) {
    Ok(_) => Ok(()),
    Err(err) => Err(pictursError::new(&err.to_string())),
  }
}
