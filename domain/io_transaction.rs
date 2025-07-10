use std::future::Future;

pub struct IOTransaction<'a, T, F = ()> {
  value: T,
  rollback_fn: Box<dyn FnOnce() -> F + 'a>,
}

impl<'a, T, F> IOTransaction<'a, T, F> {
  pub fn new(value: T, rollback_fn: impl FnOnce() -> F + 'a) -> IOTransaction<'a, T, F> {
    IOTransaction {
      value,
      rollback_fn: Box::new(rollback_fn),
    }
  }
  pub fn commit(self) -> T {
    self.value
  }
  pub const fn value(&mut self) -> &mut T {
    &mut self.value
  }
}

impl<T> IOTransaction<'_, T, ()> {
  pub fn rollback(self) -> T {
    let function = self.rollback_fn;
    function();
    self.value
  }
}

impl<T, F: Future<Output = ()>> IOTransaction<'_, T, F> {
  pub async fn rollback_async(self) -> T {
    let function = self.rollback_fn;
    function().await;
    self.value
  }
}
