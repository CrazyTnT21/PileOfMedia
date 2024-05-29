use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use base64::Engine;
use rand::distributions::Alphanumeric;
use rand::Rng;

use repositories::file_repository::FileRepository;

pub struct DefaultFileRepository {}

impl DefaultFileRepository {
  pub fn new() -> DefaultFileRepository {
    DefaultFileRepository {}
  }
}

#[async_trait]
impl FileRepository for DefaultFileRepository {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<String, Box<dyn Error>> {
    let file_name: String = file_name.map(|x| x.to_string())
      .unwrap_or_else(|| random_string(8));
    let file_extension = infer::get(data).ok_or("failed to get file extension")?.extension();
    let file_name = Path::new(&file_name).with_extension(file_extension);

    let file_path = Path::new(file_path).join(&file_name);
    let mut file = File::create_new(&file_path)?;
    file.write(data)?;
    Ok(file_name.to_str().ok_or("failed to get file name")?.to_string())
  }

  async fn create_base64(&self, data: &str, file_path: &str, file_name: Option<&str>) -> Result<String, Box<dyn Error>> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(data)?;
    self.create(&bytes, file_path, file_name).await
  }
}

fn random_string(length: usize) -> String {
  rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(char::from)
    .collect()
}
