#[derive(Debug)]
pub struct ColumnTable<'a> {
  pub columns: Vec<&'a str>,
  pub alias: &'a str,
}
