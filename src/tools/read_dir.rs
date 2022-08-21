use std::{
  fs,
  path::{
    PathBuf,
  }
};

#[macro_export]
macro_rules! read_dir {
    ($resource:expr) => (crate::tools::read_dir::read_dir_func($resource));
    ($resource:ident) => (crate::tools::read_dir::read_dir::read_dir_func($resource));
}

pub fn read_dir_func(dir_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let path_tmpl = fs::canonicalize(PathBuf::from(dir_path))?;
  let mut file_arr = Vec::new();

  for entry in fs::read_dir(&path_tmpl)? {
    let entry = entry?;
    let path = entry.path();
    let file_name = path.file_name().ok_or(format!("no file: {}", dir_path))?.to_string_lossy().to_string();
    file_arr.push(file_name);
  }
  Ok(file_arr)
}
