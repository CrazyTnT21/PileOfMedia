use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use base64::Engine;
use rand::distributions::Alphanumeric;
use rand::Rng;
use domain::file_name::FileName;

use repositories::file_repository::mut_file_repository::MutFileRepository;

pub struct DefaultMutFileRepository {}

impl DefaultMutFileRepository {
  pub fn new() -> DefaultMutFileRepository {
    DefaultMutFileRepository {}
  }
}

#[async_trait]
impl MutFileRepository for DefaultMutFileRepository {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<FileName, Box<dyn Error>> {
    let file_name: String = file_name.map(|x| x.to_string())
      .unwrap_or_else(|| random_string(8));
    let file_extension = infer::get(data).ok_or("failed to get file extension")?.extension();
    let file_name = Path::new(&file_name).with_extension(file_extension);
    let file_path = Path::new(file_path).join(&file_name);
    let uri = file_path.to_str().ok_or("failed to get file path")?.to_string();
    let mut file = File::create_new(file_path)?;
    file.write_all(data)?;
    Ok(FileName {
      name: file_name.to_str().ok_or("failed to get file name")?.to_string(),
      uri,
    })
  }

  async fn create_base64(&self, data: &str, file_path: &str, file_name: Option<&str>) -> Result<FileName, Box<dyn Error>> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(data)?;
    self.create(&bytes, file_path, file_name).await
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
