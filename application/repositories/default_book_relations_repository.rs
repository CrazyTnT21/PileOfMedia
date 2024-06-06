use std::error::Error;

use async_trait::async_trait;

use domain::entities::book::book_character::BookCharacter;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::genre::Genre;
use domain::entities::person::person_role::PersonRole;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::Table;
use repositories::book_relations_repository::BookRelationsRepository;
use repositories::book_repository::BookRepository;
use repositories::character_repository::CharacterRepository;
use repositories::genre_repository::GenreRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::RoleRepository;
use repositories::theme_repository::ThemeRepository;

use crate::enums::db_language::DbLanguage;
use crate::Pooled;
use crate::schemas::db_book_character::DbBookCharacter;
use crate::schemas::db_book_genre::DbBookGenre;
use crate::schemas::db_book_involved::DbBookInvolved;
use crate::schemas::db_book_theme::DbBookTheme;
use crate::schemas::db_role::DbRole;
use crate::schemas::db_role_translation::DbRoleTranslation;
use crate::select::comparison::Comparison::Equal;
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookRelationsRepository<'a> {
  pool: &'a Pooled<'a>,
  default_language: DbLanguage,
  book_repository: &'a dyn BookRepository,
  genre_repository: &'a dyn GenreRepository,
  theme_repository: &'a dyn ThemeRepository,
  character_repository: &'a dyn CharacterRepository,
  person_repository: &'a dyn PersonRepository,
  role_repository: &'a dyn RoleRepository,
}

impl<'a> DefaultBookRelationsRepository<'a> {
  pub fn new(pool: &'a Pooled<'a>,
             default_language: Language,
             book_repository: &'a dyn BookRepository,
             genre_repository: &'a dyn GenreRepository,
             theme_repository: &'a dyn ThemeRepository,
             character_repository: &'a dyn CharacterRepository,
             person_repository: &'a dyn PersonRepository,
             role_repository: &'a dyn RoleRepository,
  ) -> DefaultBookRelationsRepository<'a> {
    DefaultBookRelationsRepository {
      pool,
      default_language: default_language.into(),
      book_repository,
      genre_repository,
      theme_repository,
      character_repository,
      person_repository,
      role_repository,
    }
  }
}

#[async_trait]
impl<'a> BookRelationsRepository for DefaultBookRelationsRepository<'a> {
  async fn get_themes(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<Theme>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let theme_ids = Select::new::<DbBookTheme>()
      .column::<i32>(DbBookTheme::TABLE_NAME, "fktheme")
      .where_expression(Expression::new(Value((DbBookTheme::TABLE_NAME, "fkbook"), Equal(&book_id))));

    let total = theme_ids.count(self.pool).await? as usize;

    let theme_ids: Vec<i32> = theme_ids
      .pagination(pagination)
      .query(self.pool)
      .await?
      .into_iter()
      .map(|x| x.0)
      .collect();

    let items = match theme_ids.is_empty() {
      true => vec![],
      false => self.theme_repository.get_by_ids(&theme_ids, language).await?
    };
    Ok(ItemsTotal {
      items,
      total,
    })
  }

  async fn get_genres(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let genre_ids = Select::new::<DbBookGenre>()
      .column::<i32>(DbBookGenre::TABLE_NAME, "fkgenre")
      .where_expression(Expression::new(Value((DbBookGenre::TABLE_NAME, "fkbook"), Equal(&book_id))));

    let total = genre_ids.count(self.pool).await? as usize;

    let genre_ids: Vec<i32> = genre_ids
      .pagination(pagination)
      .query(self.pool)
      .await?
      .into_iter()
      .map(|x| x.0)
      .collect();

    let items = match genre_ids.is_empty() {
      true => vec![],
      false => self.genre_repository.get_by_ids(&genre_ids, language).await?
    };
    Ok(ItemsTotal {
      items,
      total,
    })
  }

  async fn get_characters(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookCharacter>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let character_books_ids = Select::new::<DbBookCharacter>()
      .column::<i32>(DbBookCharacter::TABLE_NAME, "fkcharacter")
      .where_expression(Expression::new(Value((DbBookCharacter::TABLE_NAME, "fkbook"), Equal(&book_id))));

    let total = character_books_ids.count(self.pool).await? as usize;

    let character_books_ids = character_books_ids
      .pagination(pagination)
      .query(self.pool)
      .await?;

    if character_books_ids.is_empty() {
      return Ok(ItemsTotal {
        items: vec![],
        total,
      });
    }

    let character_ids: Vec<i32> = character_books_ids
      .iter()
      .map(|x| x.0)
      .collect();

    let characters = self.character_repository.get_by_ids(&character_ids, language).await?;

    let items = characters
      .into_iter()
      .map(|character| BookCharacter { character })
      .collect();

    Ok(ItemsTotal {
      items,
      total,
    })
  }

  async fn get_involved(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookInvolved>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let db_language = DbLanguage::from(language);
    let involved = Select::new::<DbBookInvolved>()
      .columns::<DbRole>(DbRole::TABLE_NAME)
      .columns::<Option<DbRoleTranslation>>("role_translation")
      .columns::<Option<DbRoleTranslation>>("role_translation_fallback")
      .column::<i32>(DbBookInvolved::TABLE_NAME, "fkperson")
      .column::<i32>(DbBookInvolved::TABLE_NAME, "fkbook")
      .inner_join::<DbRole>(None, Expression::new(Column((DbRole::TABLE_NAME, "id"), (DbBookInvolved::TABLE_NAME, "fkperson"))))
      .left_join::<DbRoleTranslation>(
        Some("role_translation"),
        Expression::new(Column(("role_translation", "fktranslation"), (DbRole::TABLE_NAME, "id")))
          .and(Expression::column_equal("role_translation", "language", &db_language)))
      .left_join::<DbRoleTranslation>(
        Some("role_translation_fallback"),
        Expression::new(Column(("role_translation_fallback", "fktranslation"), (DbRole::TABLE_NAME, "id")))
          .and(Expression::column_equal("role_translation_fallback", "language", &self.default_language))
          .and(Expression::column_null("role_translation", "fktranslation")))
      .where_expression(Expression::new(Value((DbBookInvolved::TABLE_NAME, "fkbook"), Equal(&book_id))));

    let total = involved.count(self.pool).await? as usize;

    let mut involved = involved
      .pagination(pagination)
      .query(self.pool)
      .await?;

    if involved.is_empty() {
      return Ok(ItemsTotal {
        items: vec![],
        total,
      });
    }

    let person_ids: Vec<i32> = involved
      .iter()
      .map(|x| x.3)
      .collect();

    let role_ids: Vec<i32> = involved
      .iter()
      .map(|x| x.4)
      .collect();

    let mut people = self.person_repository.get_by_ids(&person_ids, language).await?;
    let mut roles = self.role_repository.get_by_ids(&role_ids, language).await?;

    let items = involved.into_iter().map(|x| {
      let person_index = people.iter().position(|y| y.id == x.3).unwrap();
      let role_index = roles.iter().position(|y| y.id == x.4).unwrap();
      let person = people.swap_remove(person_index);
      let role = roles.swap_remove(role_index);
      BookInvolved { person, role: PersonRole { role } }
    })
      .collect();

    Ok(ItemsTotal {
      items,
      total,
    })
  }
}
