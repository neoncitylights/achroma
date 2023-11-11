#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

pub mod numbers;
pub mod tags;

use crate::numbers::DateTimeNum;
use crate::numbers::XYZNum;
use core::str::FromStr;

macro_rules! impl_enum {
	($repr:ty, $e:ident, $($variant:ident: $hex:literal: $s:literal),+) => {
		#[repr($repr)]
		#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
		pub enum $e {
			$($variant = $hex),+
		}

		impl core::str::FromStr for $e {
			type Err = ();
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				match s {
					$($s => Ok(Self::$variant),)+
					_ => Err(()),
				}
			}
		}
	};
}

pub(crate) use impl_enum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RenderingIntent {
	IccAbsColorimetric,
	MediaRelativeColoriemtric,
	Perceptual,
	Saturation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct IccProfileReader();

// Table 17
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct IccProfileHeaderU128 {
	pub profile_size: u32,
	pub cmm_type: u32,
	pub version: u32,
	pub device_class: u32,
	pub color_space: u32,
	pub pcs: u32,
	pub creation_date_time: DateTimeNum,
	pub profile_file_signature: u32,
	pub primary_platform: u32,
	pub flags: u32,
	pub device_manufacturer: u32,
	pub device_model: u32,
	pub device_attributes: DeviceAttributes,
	pub rendering_intent: u32,
	pub illuminant: XYZNum,
	pub creator: u32,
	pub profile_id: u128,
	/*pub(crate)*/ reserved_1: [u8; 28],
}

// Custom traits
pub trait TypeSignature {
	fn type_signature(&self) -> u32;
}

// Table 18
impl_enum! {
	u32,
	ProfileClass,
	InputDeviceProfile: 0x73636E72: "scnr",
	DisplayDeviceProfile: 0x6D6E7472: "mntr",
	OutputDeviceProfile: 0x70727472: "prtr",
	DeviceLinkProfile: 0x6C696E6B: "link",
	ColorSpaceProfile: 0x73706163: "spac",
	AbstractProfile: 0x61627374: "abst",
	NamedColorProfile: 0x6E6D636C: "nmcl"
}

// Table 19
impl_enum! {
	u32,
	ColorSpace,
	NCieXyz: 0x58595A20: "XYZ ",
	CieLab: 0x4C616220: "Lab ",
	CieLuv: 0x4C757620: "Luv ",
	YcbCr: 0x59436272: "YCbr",
	CieYxy: 0x59787920: "Yxy ",
	Rgb: 0x52474220: "RGB ",
	Gray: 0x47524159: "GRAY",
	Hsv: 0x48535620: "HSV ",
	Hls: 0x484C5320: "HLS ",
	Cmyk: 0x434D594B: "CMYK",
	Cmy: 0x434D5920: "CMY ",
	Color2: 0x32434C52: "2CLR",
	Color3: 0x33434C52: "3CLR",
	Color4: 0x34434C52: "4CLR",
	Color5: 0x35434C52: "5CLR",
	Color6: 0x36434C52: "6CLR",
	Color7: 0x37434C52: "7CLR",
	Color8: 0x38434C52: "8CLR",
	Color9: 0x39434C52: "9CLR",
	Color10: 0x41434C52: "ACLR",
	Color11: 0x42434C52: "BCLR",
	Color12: 0x43434C52: "CCLR",
	Color13: 0x44434C52: "DCLR",
	Color14: 0x45434C52: "ECLR",
	Color15: 0x46434C52: "FCLR"
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DeviceAttributes(u64);

impl DeviceAttributes {
	pub fn new(value: u64) -> Self {
		Self(value)
	}
}

impl From<u64> for DeviceAttributes {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

impl From<[u32; 2]> for DeviceAttributes {
	fn from(value: [u32; 2]) -> Self {
		let combined_value = ((value[0] as u64) << 32) | (value[1] as u64);
		Self(combined_value)
	}
}
