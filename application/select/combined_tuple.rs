pub trait CombinedType {
  type Combined<T>: CombinedType;
}
macro_rules! combined_type_tuple {
    ($($generics: tt),*) => {
      impl<$($generics),*> CombinedType for ($($generics),*,) {
        type Combined<C> = ($($generics),*,C );
      }
    }
}
combined_type_tuple!(T1);
combined_type_tuple!(T1, T2);
combined_type_tuple!(T1, T2, T3);
combined_type_tuple!(T1, T2, T3, T4);
combined_type_tuple!(T1, T2, T3, T4, T5);
combined_type_tuple!(T1, T2, T3, T4, T5, T6);
combined_type_tuple!(T1, T2, T3, T4, T5, T6, T7);
combined_type_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);

impl CombinedType for () {
  type Combined<C> = (C,);
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinedType for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
  type Combined<T> = Self;
}
