use tokio_postgres::types::ToSql;

pub fn convert_to_sql(value: &[impl ToSql + Sync]) -> Vec<&(dyn ToSql + Sync)> {
  value.iter().map(|x| x as &(dyn ToSql + Sync)).collect::<Vec<&(dyn ToSql + Sync)>>()
}
