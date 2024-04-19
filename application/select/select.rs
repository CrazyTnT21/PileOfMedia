pub fn select<T: from_row::RowColumns>(alias: &str) -> String {
  let columns = T::columns();

  let result = columns.iter().map(|column| format!("{alias}.{column}")).collect::<Vec<String>>().join(",");
  result
}
