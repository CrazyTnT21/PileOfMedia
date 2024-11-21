pub trait Selector: Send + Sync {
  fn sql(&self) -> String;
}

impl Selector for (&str, &str) {
  fn sql(&self) -> String {
    format!("{}.{}", self.0, self.1)
  }
}

impl Selector for ((&str, &str),) {
  fn sql(&self) -> String {
    format!("({}.{})", self.0 .0, self.0 .1)
  }
}

macro_rules! tuple_for {
  ($t: tt) => {
    (&str, &str)
  };
}
#[macro_export]
macro_rules! selector {
  ($($generics: tt),*) => {
    impl Selector for ($(tuple_for!($generics)),+) {
      fn sql(&self) -> String {
        let ($($generics),+) = self;
        format!("({})",
        [$([$generics.0,$generics.1].join(".")),+].join(","))
      }
    }
  };
}
selector!(a, b);
selector!(a, b, c);
selector!(a, b, c, d);
selector!(a, b, c, d, e);
selector!(a, b, c, d, e, f);
selector!(a, b, c, d, e, f, g);
