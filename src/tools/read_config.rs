use std::{
  fs,
  error::Error
};
use toml::from_str;
use serde_derive::Deserialize;

use crate::read_dir;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct BaseConfig {
  pub image_path: String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Configuration {
  pub base: BaseConfig
}

pub struct ReadConfig {
  path: String
}

impl ReadConfig {
  pub fn dir_exists(dir_path: &str) -> Result<Self, Box<dyn Error>> {
    let path_tmpl = fs::canonicalize(dir_path)?;
    if !path_tmpl.exists() {
      panic!("Folder is not exists!, path: {}", path_tmpl.to_string_lossy().to_string())
    }
    Ok(Self {
      path: dir_path.to_string()
    })
  }
  pub fn read_config(self) -> Configuration {
    let file_list = read_dir!(&self.path).unwrap();
    let mut configuration: Configuration = Configuration { base: BaseConfig { image_path: "./images".to_string() } };
    for file_name in file_list {
      if file_name == "config.toml" {
        let content = fs::read_to_string(format!("{}/{}", &self.path, file_name)).unwrap();
        configuration = from_str(content.as_str()).unwrap();
      }
    }
    configuration
  }
}
