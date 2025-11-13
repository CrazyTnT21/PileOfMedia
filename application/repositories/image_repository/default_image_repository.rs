use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::image::Image;
use domain::entities::image::image_data::ImageData;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::image_repository::ImageRepository;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_image::DbImage;
use crate::schemas::db_image_data::DbImageData;
use crate::select::Select;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultImageRepository<'a> {
  client: &'a Client,
}

impl<'a> DefaultImageRepository<'a> {
  pub const fn new(client: &'a Client) -> DefaultImageRepository<'a> {
    DefaultImageRepository { client }
  }
}

#[async_trait]
impl ImageRepository for DefaultImageRepository<'_> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Image>, Box<dyn Error>> {
    let total = Select::new::<DbImage>().query_count(self.client).await?;

    let images = Select::new::<DbImage>()
      .columns::<DbImage>("image")
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0)
      .collect::<Vec<DbImage>>();

    let image_ids = image_ids(&images);
    let image_data = self.get_image_data(&image_ids).await?;

    let images = to_entities(images, image_data);
    Ok(ItemsTotal { items: images, total })
  }

  async fn get_by_id(&self, id: u32) -> Result<Option<Image>, Box<dyn Error>> {
    let id = id as i32;
    let image = Select::new::<DbImage>()
      .columns::<DbImage>("image")
      .where_expression(Expression::new(ValueEqual::new(("image", "id"), id)))
      .get_single(self.client)
      .await?
      .map(|x| x.0);

    let Some(image) = image else {
      return Ok(None);
    };

    let mut image_data = self.get_image_data(&[image.id as u32]).await?;
    Ok(Some(to_entity(image, &mut image_data)))
  }

  async fn get_by_ids(&self, ids: &[u32]) -> Result<Vec<Image>, Box<dyn Error>> {
    let ids = to_i32(ids);

    let images = Select::new::<DbImage>()
      .columns::<DbImage>("image")
      .where_expression(Expression::new(ValueIn::new(("image", "id"), &ids)))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0)
      .collect::<Vec<DbImage>>();

    let image_ids = image_ids(&images);
    let image_data = self.get_image_data(&image_ids).await?;

    Ok(to_entities(images, image_data))
  }
}

fn get_versions(id: u32, image_data: &mut Vec<DbImageData>) -> Vec<ImageData> {
  let mut filtered = Vec::new();
  let mut indices: Vec<usize> = vec![];
  for (i, x) in image_data.iter().enumerate() {
    if x.image_id as u32 == id {
      indices.push(i);
    }
  }
  indices.reverse();
  for x in indices {
    filtered.push(image_data.remove(x).to_entity());
  }
  filtered
}

fn to_entities(images: Vec<DbImage>, mut versions: Vec<DbImageData>) -> Vec<Image> {
  images.into_iter().map(|x| to_entity(x, &mut versions)).collect()
}

fn to_entity(image: DbImage, versions: &mut Vec<DbImageData>) -> Image {
  let id = image.id;
  image.to_entity(get_versions(id as u32, versions))
}

impl DefaultImageRepository<'_> {
  async fn get_image_data(&self, image_ids: &[u32]) -> Result<Vec<DbImageData>, Box<dyn Error>> {
    let image_ids = to_i32(image_ids);

    Ok(
      Select::new::<DbImageData>()
        .columns::<DbImageData>("image_data")
        .where_expression(Expression::new(ValueIn::new(("image_data", "image_id"), &image_ids)))
        .query(self.client)
        .await?
        .into_iter()
        .map(|x| x.0)
        .collect(),
    )
  }
}

fn image_ids(images: &[DbImage]) -> Vec<u32> {
  images.iter().map(|x| x.id as u32).collect()
}
