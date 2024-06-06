use std::error::Error;

use async_trait::async_trait;

use domain::entities::image::Image;
use domain::entities::image::image_data::ImageData;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::image_repository::ImageRepository;

use crate::convert_to_sql::convert_to_sql;
use crate::Pooled;
use crate::schemas::db_image::DbImage;
use crate::schemas::db_image_data::DbImageData;
use crate::select::comparison::Comparison::{Equal, In};
use crate::select::condition::Condition::Value;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultImageRepository<'a> {
  pool: &'a Pooled<'a>,
}

impl<'a> DefaultImageRepository<'a> {
  pub fn new(pool: &'a Pooled) -> DefaultImageRepository<'a> {
    DefaultImageRepository { pool }
  }
}

#[async_trait]
impl<'a> ImageRepository for DefaultImageRepository<'a> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Image>, Box<dyn Error>> {
    let select = Select::new::<DbImage>()
      .columns::<DbImage>("image");

    let total = select.count(self.pool).await? as usize;

    let images = select
      .pagination(pagination)
      .query(self.pool)
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
    let image = Select::new::<DbImage>()
      .columns::<DbImage>("image")
      .where_expression(Expression::new(Value(("image", "id"), Equal(&(id as i32)))))
      .get_single(self.pool)
      .await?
      .map(|x| x.0);

    let Some(image) = image else {
      return Ok(None);
    };

    let mut image_data = self.get_image_data(&[image.id]).await?;
    Ok(Some(to_entity(image, &mut image_data)))
  }

  async fn get_by_ids(&self, ids: &[i32]) -> Result<Vec<Image>, Box<dyn Error>> {
    let ids = convert_to_sql(ids);
    let images = Select::new::<DbImage>()
      .columns::<DbImage>("image")
      .where_expression(Expression::new(Value(("image", "id"), In(&ids))))
      .query(self.pool)
      .await?
      .into_iter()
      .map(|x| x.0)
      .collect::<Vec<DbImage>>();

    let image_ids = image_ids(&images);
    let image_data = self.get_image_data(&image_ids).await?;

    Ok(to_entities(images, image_data))
  }
}

fn get_versions(id: i32, image_data: &mut Vec<DbImageData>) -> Vec<ImageData> {
  let mut filtered = Vec::new();
  let mut indices: Vec<usize> = vec![];
  for (i, x) in image_data.iter().enumerate() {
    if x.fk_image == id {
      indices.push(i)
    }
  }
  indices.reverse();
  for x in indices {
    filtered.push(image_data.remove(x).to_entity());
  }
  filtered
}

fn to_entities(images: Vec<DbImage>, mut versions: Vec<DbImageData>) -> Vec<Image> {
  images
    .into_iter()
    .map(|x| to_entity(x, &mut versions))
    .collect()
}

fn to_entity(image: DbImage, versions: &mut Vec<DbImageData>) -> Image {
  let id = image.id;
  image.to_entity(get_versions(id, versions))
}

impl<'a> DefaultImageRepository<'a> {
  async fn get_image_data(&self, image_ids: &[i32]) -> Result<Vec<DbImageData>, Box<dyn Error>> {
    let image_ids = convert_to_sql(image_ids);
    Ok(Select::new::<DbImageData>()
      .columns::<DbImageData>("imagedata")
      .where_expression(Expression::new(Value(("imagedata", "fkimage"), In(&image_ids))))
      .query(self.pool)
      .await?
      .into_iter()
      .map(|x| x.0)
      .collect())
  }
}

fn image_ids(images: &[DbImage]) -> Vec<i32> {
  images.iter().map(|x| x.id).collect::<Vec<i32>>()
}
