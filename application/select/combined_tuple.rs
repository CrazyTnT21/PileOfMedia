pub trait CombinedType {
  type Combined<T>: CombinedType;
}
macro_rules! combined_type_tuple {
    ($first_generic: tt,$($generics: tt),*) => {
      impl<$first_generic,$($generics),*> CombinedType for ($first_generic,$($generics),*,) {
        type Combined<C> = ($first_generic,$($generics),*,C);
      }
     combined_type_tuple!($($generics),*);
    };
  ($first_generic: tt) => {
   impl<$first_generic> CombinedType for ($first_generic,) {
        type Combined<C> = ($first_generic,C);
      }
  }
}
combined_type_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);

impl CombinedType for () {
  type Combined<C> = (C,);
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinedType for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
  type Combined<T> = Self;
}
