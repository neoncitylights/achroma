macro_rules! impl_tag_fn_new {
    ($sig:literal, $reserved:literal) => {
        pub fn new() -> Self {
            type_signature: $sig,
            reserved_1: $reserved,

        }
    };

    ($sig:literal, $reserved:literal, $($param:ident: $param_t:ty),+) => {
        pub fn new($($param: $param_t),*) -> Self {
            Self {
                type_signature: $sig,
                reserved_1: $reserved,
                $($param),*
            }
        }
    };
}

pub(crate) use impl_tag_fn_new;
