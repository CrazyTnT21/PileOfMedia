use std::error::Error;
use std::io::Cursor;
use std::sync::Arc;

use async_trait::async_trait;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use tokio_postgres::Transaction;

use domain::entities::image::Image;
use domain::entities::image::create_partial_image::CreatePartialImage;
use repositories::file_repository::FileRepository;
use repositories::file_repository::mut_file_repository::MutFileRepository;
use repositories::image_repository::ImageRepository;
use repositories::image_repository::mut_image_repository::MutImageRepository;

use crate::insert::Insert;
use crate::schemas::db_image::DbImage;
use crate::schemas::db_image_data::DbImageData;

pub struct DefaultMutImageRepository<'a> {
  transaction: &'a Transaction<'a>,
  image_repository: Arc<dyn ImageRepository + 'a>,
  mut_file_repository: Arc<dyn MutFileRepository + 'a>,
  file_repository: Arc<dyn FileRepository + 'a>,
}

impl<'a> DefaultMutImageRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    image_repository: Arc<dyn ImageRepository + 'a>,
    mut_file_repository: Arc<dyn MutFileRepository + 'a>,
    file_repository: Arc<dyn FileRepository + 'a>,
  ) -> DefaultMutImageRepository<'a> {
    DefaultMutImageRepository {
      transaction,
      image_repository,
      mut_file_repository,
      file_repository,
    }
  }
}

#[async_trait]
impl MutImageRepository for DefaultMutImageRepository<'_> {
  async fn create(&self, image: CreatePartialImage<'_>) -> Result<Image, Box<dyn Error>> {
    let id: i32 = Insert::new::<DbImage>([])
      .returning_transaction("id", self.transaction)
      .await?;
    let image_data = self.file_repository.get(image.uri).await?;

    let file_image = image::ImageReader::new(Cursor::new(image_data.clone())).with_guessed_format()?;
    let format = file_image.format().ok_or("Unknown format")?;
    let file_image = file_image.decode()?;
    let original_path = combined(image.display_path.to_string(), image.file_name);
    let (x, y) = (
      i16::try_from(file_image.width()).unwrap(),
      i16::try_from(file_image.height()).unwrap(),
    );

    let (medium_path, medium_x, medium_y) = self.resize(2, &file_image, &format, &image).await?;
    let (low_path, low_x, low_y) = self.resize(4, &file_image, &format, &image).await?;

    let insert = Insert::new::<DbImageData>(["fkimage", "uri", "width", "height"])
      .values([&id, &original_path, &x, &y])
      .values([&id, &medium_path, &medium_x, &medium_y])
      .values([&id, &low_path, &low_x, &low_y]);

    insert.execute_transaction(self.transaction).await?;

    Ok(
      self
        .image_repository
        .get_by_id(id as u32)
        .await?
        .expect("image was just created, it should exist"),
    )
  }
}

fn combined(mut value: String, b: &str) -> String {
  value.push_str(b);
  value
}

impl DefaultMutImageRepository<'_> {
  async fn resize(
    &self,
    factor: u32,
    file_image: &DynamicImage,
    format: &ImageFormat,
    image: &CreatePartialImage<'_>,
  ) -> Result<(String, i16, i16), Box<dyn Error>> {
    let mut bytes: Vec<u8> = Vec::new();
    let (x, y) = (file_image.width() / factor, file_image.height() / factor);
    let temp_image = file_image.resize(x, y, FilterType::Triangle);
    temp_image.write_to(&mut Cursor::new(&mut bytes), *format)?;
    let path = self.mut_file_repository.create(&bytes, image.file_path, None).await?;
    let path = combined(image.display_path.to_string(), &path.name);
    Ok((path, x.try_into().unwrap(), y.try_into().unwrap()))
  }
}
