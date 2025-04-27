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
    format!("({}.{})", self.0.0, self.0.1)
  }
}

macro_rules! tuple_for {
  ($t: tt) => {
    (&str, &str)
  };
}
#[macro_export]
macro_rules! selector {
  ($first_generic: tt,$second_generic: tt, $($generics: tt),*) => {
    impl Selector for (tuple_for!($first_generic),tuple_for!($second_generic),$(tuple_for!($generics)),+) {
      fn sql(&self) -> String {
        let ($first_generic,$second_generic,$($generics),+) = self;
        format!("({})",
        [[$first_generic.0,$first_generic.1].join("."),[$second_generic.0,$second_generic.1].join("."),$([$generics.0,$generics.1].join(".")),+].join(","))
      }
    }
     selector!($second_generic,$($generics),*);
  };
 ($first_generic: tt,$second_generic: tt) => {
    impl Selector for (tuple_for!($first_generic),tuple_for!($second_generic),) {
      fn sql(&self) -> String {
        let ($first_generic,$second_generic,) = self;
        format!("({})",
        [[$first_generic.0,$first_generic.1].join("."),[$second_generic.0,$second_generic.1].join(".")].join(","))
      }
    }
  };
}
selector!(a, b, c, d, e, f, g);
