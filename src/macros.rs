#[macro_export]
macro_rules! from_enum {
    ($test:ident::$variant:ident($t:ty)) => {
        impl<'a> From<$t> for $test<'a>
        where
            Self: 'a,
            $t: 'a,
        {
            fn from(e: $t) -> Self {
                <$test>::$variant(e)
            }
        }
    };
}
