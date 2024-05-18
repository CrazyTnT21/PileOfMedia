#[cfg(feature = "utoipa")]
pub mod utoipa {
  use utoipa::openapi::{Object, ObjectBuilder, SchemaFormat};
  pub fn naive_date() -> Object {
    ObjectBuilder::new()
      .schema_type(utoipa::openapi::SchemaType::String)
      .format(naive_date_format())
      // .description(Some("description"))
      .build()
  }
  pub fn naive_date_optional() -> Object {
    ObjectBuilder::new()
      .schema_type(utoipa::openapi::SchemaType::String)
      .format(naive_date_format())
      .nullable(true)
      // .description(Some("description"))
      .build()
  }

  fn naive_date_format() -> Option<SchemaFormat> {
    // None
    Some(SchemaFormat::Custom(
      "YYYY-MM-DD".to_string(),
    ))
  }

}
