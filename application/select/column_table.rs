#[derive(Debug, Clone)]
pub struct ColumnTable<'a> {
  pub columns: Vec<&'a str>,
  pub alias: &'a str,
}

#[derive(Debug, Clone)]
pub enum SelectElement<'a> {
  Column(ColumnTable<'a>),
  Raw(&'a str),
}
