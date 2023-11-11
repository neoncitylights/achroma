macro_rules! impl_num_primitive {
	($n:ident, $n_ty:ty) => {
		#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
		pub struct $n($n_ty);

		impl $n {
			pub const fn new(value:$n_ty) -> Self {
				Self(value)
			}

			pub const fn get(&self) -> $n_ty {
				self.0
			}
		}

		impl Default for $n {
			fn default() -> Self {
				Self(<$n_ty>::default())
			}
		}

		impl From<$n_ty> for $n {
			fn from(value: $n_ty) -> Self {
				Self(value)
			}
		}
	};
}

// macro_rules! impl_num_fixed_int {
// 	($n:ident, $t:ty, $bit_and:literal, $fractional_parts: literal) => {
// 		impl FixedInt<$t> for $n {
// 			fn from_parts(integer: $t, fractional: $t) -> Self {
// 				let value = ((integer as $t) << $fractional_parts) | fractional as $t;
// 				Self(value)
// 			}

// 			fn integer_part(&self) -> $t {
// 				(self.0 >> $fractional_parts) as $t
// 			}

// 			fn fractional_part(&self) -> $t {
// 				(self.0 & $bit_and) as $t
// 			}
// 		}
// 	};
// }

pub trait FixedInt<T> {
	fn from_parts(integer: T, fractional: T) -> Self;
	fn integer_part(&self) -> T;
	fn fractional_part(&self) -> T;
}

// primitives
// impl_num_primitive!(Response16, u64);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Response16 {
	pub u16_slot: u16,
	pub(crate) reserved: u16,
	pub measurement_value: S15Fixed16,
}

impl_num_primitive!(S15Fixed16, i32);
// impl_num_fixed_int!(S15Fixed16, i16, 0xFFFF, 16);

impl_num_primitive!(U16Fixed16, u32);
// impl_num_fixed_int!(U16Fixed16, u16, 0xFFFF, 16);

impl_num_primitive!(U1Fixed15, u16);
// impl_num_fixed_int!(U1Fixed15, u8, 0xFF, 15);

impl_num_primitive!(U8Fixed8, u16);
// impl_num_fixed_int!(U8Fixed8, u8, 0xFF, 8);

// primitives w/ fixed-size arrays
impl_num_primitive!(XYZNum, [S15Fixed16; 3]);
impl_num_primitive!(PositionNum, [u32; 2]);
impl_num_primitive!(DateTimeNum, [u16; 8]);
impl_num_primitive!(Bit7Ascii, bitvec::array::BitArray<[usize; 7]>);
