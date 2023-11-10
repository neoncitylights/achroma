use crate::numbers::{S15Fixed16, U16Fixed16};

macro_rules! impl_array {
    ($a:ident, $a_ty:ty, $sig:literal, $reserved:literal) => {
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        pub struct $a {
            pub(crate) type_signature: u32,
            pub(crate) reserved_1: u32,
            pub values: Vec<$a_ty>,
        }

        impl $a {
            pub const fn new(values: Vec<$a_ty>) -> Self {
                Self {
                    type_signature: $sig,
                    reserved_1: $reserved,
                    values
                }
            }

            pub fn bytes(&self) -> usize {
                8usize + // size of type_signature + reserved_1
                (self.values.len() * std::mem::size_of::<Vec<$a_ty>>())
            }
        }

        impl From<$a> for Vec<$a_ty> {
            fn from(value: $a) -> Self {
                value.values
            }
        }

        impl From<Vec<$a_ty>> for $a {
            fn from(value: Vec<$a_ty>) -> Self {
                Self::new(value)
            }
        }

        impl Default for $a {
            fn default() -> Self {
                Self::new(Vec::default())
            }
        }
    };
}

impl_array!(S15Fixed16Array, S15Fixed16, 0x73663332u32, 0u32); // Table 76
impl_array!(U16Fixed16Array, U16Fixed16, 0x75663332u32, 0u32); // Table 79
impl_array!(U16Array, u16, 0x75693136u32, 0u32); // Table 80
impl_array!(U32Array, u32, 0x75693332u32, 0u32); // Table 81
impl_array!(U64Array, u32, 0x75693634u32, 0u32); // Table 82
impl_array!(U8Array, u8, 0x75693038u32, 0u32); // Table 83
