#[macro_export]
macro_rules! convert {
    ($from: ident,$to: ident,$($variants: ident),+) => {
        impl From <$to> for  $from {
            fn from(item: $to) -> Self {
                match item {
                    $($to::$variants => $from::$variants),+
                }
            }
        }

        impl From <$from> for  $to {
            fn from( item: $from ) -> Self {
                match item {
                    $($from::$variants => $to::$variants),+
                }
            }
        }
    }
}
