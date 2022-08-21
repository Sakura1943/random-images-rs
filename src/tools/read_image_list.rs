use std::fs;
use crate::read_dir;

#[derive(Debug)]
pub struct ReadImageList {
  pub image_list: Vec<String>
}

impl ReadImageList {
  pub fn new(dir_path: String) -> Self {
    let path_tmpl = fs::canonicalize(&dir_path).unwrap();
    let image_list = read_dir!(&path_tmpl.to_string_lossy().to_string()).unwrap();
    Self {
      image_list
    }
  }
}
