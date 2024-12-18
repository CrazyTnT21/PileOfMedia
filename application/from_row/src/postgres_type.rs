use tokio_postgres::types::Type;
#[derive(Debug)]
pub enum TypeKind {
  Postgres(Type),
  SimpleType {
    name: &'static str,
    kind: tokio_postgres::types::Kind,
  },
}
pub trait PostgresType {
  const POSTGRES_TYPES: &'static [TypeKind];
  const NULLABLE: bool = false;
}
impl PostgresType for &str {
  const POSTGRES_TYPES: &'static [TypeKind] = &[
    TypeKind::Postgres(Type::VARCHAR),
    TypeKind::Postgres(Type::TEXT),
    TypeKind::Postgres(Type::NAME),
  ];
}
impl PostgresType for String {
  const POSTGRES_TYPES: &'static [TypeKind] = &[
    TypeKind::Postgres(Type::VARCHAR),
    TypeKind::Postgres(Type::TEXT),
    TypeKind::Postgres(Type::NAME),
  ];
}
impl PostgresType for bool {
  const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::BOOL)];
}
impl PostgresType for i8 {
  const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::CHAR)];
}
impl PostgresType for i16 {
  const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::INT2)];
}
impl PostgresType for i32 {
  const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::INT4)];
}
impl PostgresType for i64 {
  const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::INT8)];
}
impl PostgresType for f32 {
  const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::FLOAT4)];
}
impl PostgresType for f64 {
  const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::FLOAT8)];
}
impl<T: PostgresType> PostgresType for Option<T> {
  const POSTGRES_TYPES: &'static [TypeKind] = T::POSTGRES_TYPES;
  const NULLABLE: bool = true;
}
