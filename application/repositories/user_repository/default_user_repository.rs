use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::image::Image;
use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::Table;
use repositories::image_repository::ImageRepository;
use repositories::user_repository::UserRepository;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_user::DbUser;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultUserRepository<'a> {
  client: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
}

impl<'a> DefaultUserRepository<'a> {
  pub fn new(client: &'a Client, image_repository: Arc<dyn ImageRepository + 'a>) -> DefaultUserRepository<'a> {
    DefaultUserRepository {
      client,
      image_repository,
    }
  }
}

#[async_trait]
impl UserRepository for DefaultUserRepository<'_> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<User>, Box<dyn Error>> {
    let total = Select::new::<DbUser>().query_count(self.client).await?;

    let users = Select::new::<DbUser>()
      .columns::<DbUser>(DbUser::TABLE_NAME)
      .pagination(pagination)
      .query(self.client)
      .await?;

    Ok(ItemsTotal {
      items: self.to_entities(users).await?,
      total,
    })
  }

  async fn get_by_id(&self, id: u32) -> Result<Option<User>, Box<dyn Error>> {
    let id = id as i32;
    let user = Select::new::<DbUser>()
      .columns::<DbUser>(DbUser::TABLE_NAME)
      .where_expression(Expression::new(ValueEqual::new((DbUser::TABLE_NAME, "id"), id)))
      .get_single(self.client)
      .await?;
    let image_id = user.as_ref().and_then(|x| x.0.fk_profile_picture);
    let image = match image_id {
      None => None,
      Some(id) => self.image_repository.get_by_id(id as u32).await?,
    };
    Ok(user.map(|x| to_entity(x, image)))
  }

  async fn get_by_ids(&self, ids: &[u32]) -> Result<Vec<User>, Box<dyn Error>> {
    let ids = to_i32(ids);

    let users = Select::new::<DbUser>()
      .columns::<DbUser>(DbUser::TABLE_NAME)
      .where_expression(Expression::new(ValueIn::new((DbUser::TABLE_NAME, "id"), &ids)))
      .query(self.client)
      .await?;

    Ok(self.to_entities(users).await?)
  }

  async fn get_by_name(&self, name: &str, pagination: Pagination) -> Result<ItemsTotal<User>, Box<dyn Error>> {
    let name = format!("%{name}%");

    let total = Select::new::<DbUser>()
      .where_expression(Expression::new(ValueILike::new((DbUser::TABLE_NAME, "name"), &name)))
      .query_count(self.client)
      .await?;

    let users = Select::new::<DbUser>()
      .columns::<DbUser>(DbUser::TABLE_NAME)
      .where_expression(Expression::new(ValueILike::new((DbUser::TABLE_NAME, "name"), &name)))
      .pagination(pagination)
      .query(self.client)
      .await?;

    Ok(ItemsTotal {
      items: self.to_entities(users).await?,
      total,
    })
  }

  async fn filter_existing(&self, users: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let users = to_i32(users);

    let count = Select::new::<DbUser>()
      .column::<i32>(DbUser::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbUser::TABLE_NAME, "id"), &users)))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}

fn to_entity(user: (DbUser,), image: Option<Image>) -> User {
  user.0.to_entity(image)
}

impl DefaultUserRepository<'_> {
  async fn to_entities(&self, items: Vec<(DbUser,)>) -> Result<Vec<User>, Box<dyn Error>> {
    let image_ids: Vec<u32> = items
      .iter()
      .filter_map(|x| x.0.fk_profile_picture.map(|x| x as u32))
      .collect();

    let mut images = match image_ids.is_empty() {
      true => vec![],
      false => self.image_repository.get_by_ids(&image_ids).await?,
    };
    Ok(
      items
        .into_iter()
        .map(|x| {
          let image_index = x
            .0
            .fk_profile_picture
            .and_then(|x| images.iter().position(|y| y.id == x as u32));
          let image = image_index.map(|x| images.swap_remove(x));
          x.0.to_entity(image)
        })
        .collect(),
    )
  }
}
