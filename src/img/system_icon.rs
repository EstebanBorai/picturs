use std::vec::Vec;
use crate::img::Dimensions;

pub trait SystemIcon {
  fn save_resized(data: &[u8], sizes: Vec::<Dimensions>);
}
