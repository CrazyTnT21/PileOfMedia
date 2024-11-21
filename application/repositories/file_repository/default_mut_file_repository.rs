use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use rand::distributions::Alphanumeric;
use rand::Rng;

use domain::file_name::FileName;
use repositories::file_repository::mut_file_repository::MutFileRepository;

pub struct DefaultMutFileRepository {}

impl DefaultMutFileRepository {
  pub const fn new() -> DefaultMutFileRepository {
    DefaultMutFileRepository {}
  }
}
impl Default for DefaultMutFileRepository {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait]
impl MutFileRepository for DefaultMutFileRepository {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<FileName, Box<dyn Error>> {
    let file_name: String = file_name.map_or_else(|| random_string(8), ToString::to_string);
    let file_extension = infer::get(data);
    let file_name = match file_extension {
      Some(value) => Path::new(&file_name).with_extension(value.extension()),
      None => Path::new(&file_name).to_path_buf(),
    };
    let file_path = Path::new(file_path).join(&file_name);
    let uri = file_path.to_str().ok_or("failed to get file path")?.to_string();
    let mut file = File::create_new(file_path)?;
    file.write_all(data)?;
    Ok(FileName {
      name: file_name.to_str().ok_or("failed to get file name")?.to_string(),
      uri,
    })
  }

  async fn delete(&self, uri: &str) -> Result<(), Box<dyn Error>> {
    Ok(fs::remove_file(uri)?)
  }
}

fn random_string(length: usize) -> String {
  rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(char::from)
    .collect()
}
