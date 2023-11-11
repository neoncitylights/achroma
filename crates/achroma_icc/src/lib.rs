#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

pub mod numbers;
pub mod tags;

use crate::numbers::DateTimeNum;
use crate::numbers::XYZNum;
use core::str::FromStr;

macro_rules! impl_enum {
    ($e:ident, $($variant:ident: $hex:literal: $s:literal),+) => {
        #[repr(u32)]
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
    }
}

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
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProfileClass {
	InputDeviceProfile = 0x73636E72,
	DisplayDeviceProfile = 0x6D6E7472,
	OutputDeviceProfile = 0x70727472,
	DeviceLinkProfile = 0x6C696E6B,
	ColorSpaceProfile = 0x73706163,
	AbstractProfile = 0x61627374,
	NamedColorProfile = 0x6E6D636C,
}

impl FromStr for ProfileClass {
	type Err = ();
	fn from_str(v: &str) -> Result<Self, Self::Err> {
		match v {
			"scnr" => Ok(Self::InputDeviceProfile),
			"mntr" => Ok(Self::DisplayDeviceProfile),
			"prtr" => Ok(Self::OutputDeviceProfile),
			"link" => Ok(Self::DeviceLinkProfile),
			"spac" => Ok(Self::ColorSpaceProfile),
			"abst" => Ok(Self::AbstractProfile),
			"nmcl" => Ok(Self::NamedColorProfile),
			_ => Err(()),
		}
	}
}

// Table 19
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ColorSpace {
	NCieXyz = 0x58595A20,
	CieLab = 0x4C616220,
	CieLuv = 0x4C757620,
	YcbCr = 0x59436272,
	CieYxy = 0x59787920,
	Rgb = 0x52474220,
	Gray = 0x47524159,
	Hsv = 0x48535620,
	Hls = 0x484C5320,
	Cmyk = 0x434D594B,
	Cmy = 0x434D5920,
	Color2 = 0x32434C52,
	Color3 = 0x33434C52,
	Color4 = 0x34434C52,
	Color5 = 0x35434C52,
	Color6 = 0x36434C52,
	Color7 = 0x37434C52,
	Color8 = 0x38434C52,
	Color9 = 0x39434C52,
	Color10 = 0x41434C52,
	Color11 = 0x42434C52,
	Color12 = 0x43434C52,
	Color13 = 0x44434C52,
	Color14 = 0x45434C52,
	Color15 = 0x46434C52,
}

impl FromStr for ColorSpace {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"XYZ " => Ok(Self::NCieXyz),
			"Lab " => Ok(Self::CieLab),
			"Luv " => Ok(Self::CieLuv),
			"YCbr" => Ok(Self::YcbCr),
			"Yxy " => Ok(Self::CieYxy),
			"RGB " => Ok(Self::Rgb),
			"GRAY" => Ok(Self::Gray),
			"HSV " => Ok(Self::Hsv),
			"HLS " => Ok(Self::Hls),
			"CMYK" => Ok(Self::Cmyk),
			"CMY " => Ok(Self::Cmy),
			"2CLR" => Ok(Self::Color2),
			"3CLR" => Ok(Self::Color3),
			"4CLR" => Ok(Self::Color4),
			"5CLR" => Ok(Self::Color5),
			"6CLR" => Ok(Self::Color6),
			"7CLR" => Ok(Self::Color7),
			"8CLR" => Ok(Self::Color8),
			"9CLR" => Ok(Self::Color9),
			"ACLR" => Ok(Self::Color10),
			"BCLR" => Ok(Self::Color11),
			"CCLR" => Ok(Self::Color12),
			"DCLR" => Ok(Self::Color13),
			"ECLR" => Ok(Self::Color14),
			"FCLR" => Ok(Self::Color15),
			_ => Err(()),
		}
	}
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
