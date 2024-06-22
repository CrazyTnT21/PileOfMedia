pub fn to_i32(items: &[u32]) -> Vec<i32> {
  items.iter().map(|x| *x as i32).collect()
}
