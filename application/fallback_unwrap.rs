use std::fmt::Debug;

pub fn fallback_unwrap<T: Debug>(item: Option<T>, fallback: Option<T>) -> T {
  let expect = &format!("Fallback for ${:?} should exist", &item);
  item.unwrap_or_else(|| fallback.expect(expect))
}

pub fn fallback_unwrap_ref<'a, T: Debug>(item: Option<&'a T>, fallback: Option<&'a T>) -> &'a T {
  let expect = &format!("Fallback for ${:?} should exist", &item);
  item.unwrap_or_else(|| fallback.expect(expect))
}
