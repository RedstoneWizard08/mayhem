#[macro_export]
macro_rules! force_error_from {
    ($parent: ident: $($variant: ident => $real: path;)+) => {
        $(
            impl From<$real> for $parent {
                fn from(value: $real) -> $parent {
                    $parent::$variant(value)
                }
            }
        )+
    };
}
