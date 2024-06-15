use tokio_postgres::types::ToSql;

pub fn convert_to_sql(value: &[impl ToSql + Sync]) -> Vec<&(dyn ToSql + Sync)> {
  value.iter().map(|x| x as &(dyn ToSql + Sync)).collect()
}

pub fn to_i32(items: &[u32]) -> Vec<i32> {
  items.iter().map(|x| *x as i32).collect()
}
