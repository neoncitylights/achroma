//! A tiny crate for encoding data related to color vision and color vision deficiency (CVD)
//!
//! ## Usage
//! ```
//! use achroma::*;
//!
//! // Partial blindness to red light
//! let protanomaly = ColorVision::Protanomaly;
//! let protanomaly_summary = ConeCellSummary::from(protanomaly);
//!
//! // query for information of overall CVD
//! assert_eq!(protanomaly.is_red_green(), true);
//! assert_eq!(protanomaly.is_anomalous_trichromacy(), true);
//!
//! // query for individual cone cells by length
//! assert_eq!(protanomaly_summary.long(),   ConeCellCond::Anomalous);
//! assert_eq!(protanomaly_summary.medium(), ConeCellCond::Normal);
//! assert_eq!(protanomaly_summary.short(),  ConeCellCond::Normal);
//! assert_eq!(protanomaly_summary.is_cone_normal(ConeCell::Long), false);
//!
//! // query for individual cone cells by color
//! assert_eq!(protanomaly_summary.red().is_anomalous(), true);
//! assert_eq!(protanomaly_summary.green(), ConeCellCond::Normal);
//! assert_eq!(protanomaly_summary['b'].is_normal(), true);
//! assert_eq!(protanomaly_summary['B'].is_normal(), true);
//! ```
#![no_std]

use core::ops::{Index, IndexMut};

/// A type of photoreceptor cell which exists in the retina
/// of a vertebrate's eye, and responsible for color vision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConeCell {
	/// A cone cell with a long wavelength known as **L**.
	/// Responds mostly to red wavelength, and expresses the [OPN1LW][pubchem-ospin] ospin
	///
	/// [pubchem-ospin]: <https://pubchem.ncbi.nlm.nih.gov/gene/5956>
	Long,
	/// A cone cell with a medium wavelength known as **M**.
	/// Responds mostly to yellow and green wavelength, and expresses the [OPN1MW][pubchem-ospin] ospin
	///
	/// [pubchem-ospin]: <https://pubchem.ncbi.nlm.nih.gov/gene/2652>
	Medium,
	/// A cone cell with a short wavelength known as **S**.
	/// Responds mostly to blue wavelength, and expresses the [OPN1SW][pubchem-ospin] ospin
	///
	/// [pubchem-ospin]: <https://pubchem.ncbi.nlm.nih.gov/gene/611>
	Short,
}

impl TryFrom<char> for ConeCell {
	type Error = ();
	/// Attempt to convert an ASCII character to a well-representing cone cell.
	///
	/// This accepts case-insensitive variants of letters representing either:
	/// - Length: either S (short), M (medium), or L (long)
	/// - Primary color: either B (blue), G (green), or R (Red)
	/// ```
	/// use achroma::ConeCell;
	///
	/// // Long/red cone cells
	/// assert_eq!(ConeCell::try_from('L'), Ok(ConeCell::Long));
	/// assert_eq!(ConeCell::try_from('l'), Ok(ConeCell::Long));
	/// assert_eq!(ConeCell::try_from('R'), Ok(ConeCell::Long));
	/// assert_eq!(ConeCell::try_from('r'), Ok(ConeCell::Long));
	///
	/// // Medium/green cone cells
	/// assert_eq!(ConeCell::try_from('M'), Ok(ConeCell::Medium));
	/// assert_eq!(ConeCell::try_from('m'), Ok(ConeCell::Medium));
	/// assert_eq!(ConeCell::try_from('G'), Ok(ConeCell::Medium));
	/// assert_eq!(ConeCell::try_from('g'), Ok(ConeCell::Medium));
	///
	/// // Short/blue cone cells
	/// assert_eq!(ConeCell::try_from('S'), Ok(ConeCell::Short));
	/// assert_eq!(ConeCell::try_from('s'), Ok(ConeCell::Short));
	/// assert_eq!(ConeCell::try_from('B'), Ok(ConeCell::Short));
	/// assert_eq!(ConeCell::try_from('b'), Ok(ConeCell::Short));
	/// ```
	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'l' | 'L' | 'r' | 'R' => Ok(Self::Long),
			'm' | 'M' | 'g' | 'G' => Ok(Self::Medium),
			's' | 'S' | 'b' | 'B' => Ok(Self::Short),
			_ => Err(()),
		}
	}
}

/// The condition (or state of health) for a cone cell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ConeCellCond {
	/// A cone cell which exists and is in a healthy state
	#[default]
	Normal,
	/// A cone cell with a lower spectral sensitivity
	Anomalous,
	/// A non-existing cone cell
	Missing,
}

impl ConeCellCond {
	pub const fn is_normal(&self) -> bool {
		matches!(self, Self::Normal)
	}

	pub const fn is_anomalous(&self) -> bool {
		matches!(self, Self::Anomalous)
	}

	pub const fn is_missing(&self) -> bool {
		matches!(self, Self::Missing)
	}
}

/// A discrete representation of the conditions for all three cone cells
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConeCellSummary {
	pub l: ConeCellCond,
	pub m: ConeCellCond,
	pub s: ConeCellCond,
}

impl ConeCellSummary {
	/// see documentation for [`ColorVision::Normal`]
	pub const NORMAL: Self = Self::new(
		ConeCellCond::Normal,
		ConeCellCond::Normal,
		ConeCellCond::Normal,
	);

	/// See documentation for [`ColorVision::Protanomaly`]
	pub const PROTANOMALY: Self = Self::new(
		ConeCellCond::Anomalous,
		ConeCellCond::Normal,
		ConeCellCond::Normal,
	);
	/// See documentation for [`ColorVision::Protanopia`]
	pub const PROTANOPIA: Self = Self::new(
		ConeCellCond::Missing,
		ConeCellCond::Normal,
		ConeCellCond::Normal,
	);

	/// See documentation for [`ColorVision::Deuteranomaly`]
	pub const DEUTERANOMALY: Self = Self::new(
		ConeCellCond::Normal,
		ConeCellCond::Anomalous,
		ConeCellCond::Normal,
	);
	/// See documentation for [`ColorVision::Deuteranopia`]
	pub const DEUTERANOPIA: Self = Self::new(
		ConeCellCond::Normal,
		ConeCellCond::Missing,
		ConeCellCond::Normal,
	);

	/// See documentation for [`ColorVision::Tritanomaly`]
	pub const TRITANOMALY: Self = Self::new(
		ConeCellCond::Normal,
		ConeCellCond::Normal,
		ConeCellCond::Anomalous,
	);
	/// See documentation for [`ColorVision::Tritanopia`]
	pub const TRITANOPIA: Self = Self::new(
		ConeCellCond::Normal,
		ConeCellCond::Normal,
		ConeCellCond::Missing,
	);

	/// See documentation for [`ColorVision::Achromatomaly`]
	pub const ACHROMATOMALY: Self = Self::new(
		ConeCellCond::Missing,
		ConeCellCond::Missing,
		ConeCellCond::Normal,
	);
	/// See documentation for [`ColorVision::Achromatopsia`]
	pub const ACHROMATOPSIA: Self = Self::new(
		ConeCellCond::Missing,
		ConeCellCond::Missing,
		ConeCellCond::Missing,
	);

	/// Creates a new instance of a [ConeCellSummary] in descending order
	/// of cone cell's wavelength sensitivity (long, medium, short)
	pub const fn new(l: ConeCellCond, m: ConeCellCond, s: ConeCellCond) -> Self {
		Self { l, m, s }
	}

	/// Creates a new instance of a [ConeCellSummary] by cone cell colors
	pub const fn rgb(r: ConeCellCond, g: ConeCellCond, b: ConeCellCond) -> Self {
		Self::new(r, g, b)
	}

	/// The state of a long cone cell
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let protanomaly = ConeCellSummary::new(
	///     ConeCellCond::Anomalous,
	///     ConeCellCond::Normal,
	///     ConeCellCond::Normal,
	/// );
	///
	/// assert_eq!(protanomaly.long(), ConeCellCond::Anomalous);
	/// ```
	pub const fn long(&self) -> ConeCellCond {
		self.l
	}

	/// The condition of a medium cone cell
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let deuteranopia = ConeCellSummary::new(
	///     ConeCellCond::Normal,
	///     ConeCellCond::Missing,
	///     ConeCellCond::Normal,
	/// );
	///
	/// assert_eq!(deuteranopia.medium(), ConeCellCond::Missing);
	/// ```
	pub const fn medium(&self) -> ConeCellCond {
		self.m
	}

	/// The condition of a short cone cell
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let tritanomaly = ConeCellSummary::new(
	///     ConeCellCond::Normal,
	///     ConeCellCond::Normal,
	///     ConeCellCond::Anomalous,
	/// );
	///
	/// assert_eq!(tritanomaly.short(), ConeCellCond::Anomalous);
	/// ```
	pub const fn short(&self) -> ConeCellCond {
		self.s
	}

	/// The condition of a long cone cell, the cell
	/// most sensitive to red wavelength
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let protanomaly = ConeCellSummary::new(
	///    ConeCellCond::Anomalous,
	///    ConeCellCond::Normal,
	///    ConeCellCond::Normal
	/// );
	///
	/// assert!(protanomaly.red().is_anomalous());
	/// ```
	pub const fn red(&self) -> ConeCellCond {
		self.l
	}

	/// The condition of a medium cone cell, the cell
	/// most sensitive to green wavelength
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let deuteranopia = ConeCellSummary::new(
	///     ConeCellCond::Normal,
	///     ConeCellCond::Missing,
	///     ConeCellCond::Normal
	/// );
	///
	/// assert!(deuteranopia.green().is_missing());
	/// ```
	pub const fn green(&self) -> ConeCellCond {
		self.m
	}

	/// The condition of a short cone cell, the cell
	/// most sensitive to blue wavelength
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let tritanomaly = ConeCellSummary::new(
	///     ConeCellCond::Normal,
	///     ConeCellCond::Normal,
	///     ConeCellCond::Anomalous
	/// );
	///
	/// assert!(tritanomaly.blue().is_anomalous());
	/// ```
	pub const fn blue(&self) -> ConeCellCond {
		self.s
	}

	/// Reports whether a given cone cell is normal
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCell, ConeCellCond};
	///
	/// let protanopia = ConeCellSummary::new(
	///    ConeCellCond::Anomalous,
	///    ConeCellCond::Normal,
	///    ConeCellCond::Normal,
	/// );
	/// assert_eq!(protanopia.is_cone_normal(ConeCell::Long), false);
	/// ```
	pub const fn is_cone_normal(&self, cone: ConeCell) -> bool {
		match cone {
			ConeCell::Long => self.l,
			ConeCell::Medium => self.m,
			ConeCell::Short => self.s,
		}
		.is_normal()
	}

	/// Reports whether a given cone cell is anomalous
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCell, ConeCellCond};
	///
	/// let deuteranomaly = ConeCellSummary::new(
	///    ConeCellCond::Normal,
	///    ConeCellCond::Anomalous,
	///    ConeCellCond::Normal,
	/// );
	/// assert_eq!(deuteranomaly.is_cone_anomalous(ConeCell::Medium), true);
	/// ```
	pub const fn is_cone_anomalous(&self, cone: ConeCell) -> bool {
		match cone {
			ConeCell::Long => self.l,
			ConeCell::Medium => self.m,
			ConeCell::Short => self.s,
		}
		.is_anomalous()
	}

	/// Reports whether a given cone cell is missing
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCell, ConeCellCond};
	///
	/// let tritanopia = ConeCellSummary::new(
	///    ConeCellCond::Normal,
	///    ConeCellCond::Normal,
	///    ConeCellCond::Missing,
	/// );
	/// assert_eq!(tritanopia.is_cone_missing(ConeCell::Short), true);
	/// ```
	pub const fn is_cone_missing(&self, cone: ConeCell) -> bool {
		match cone {
			ConeCell::Long => self.l,
			ConeCell::Medium => self.m,
			ConeCell::Short => self.s,
		}
		.is_missing()
	}

	/// Converts to a 3-tuple of [`ConeCellCond`]
	/// in descending order (long, medium, short)
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let tritanopia = ConeCellSummary::new(
	///    ConeCellCond::Normal,
	///    ConeCellCond::Normal,
	///    ConeCellCond::Missing
	/// );
	/// let tritanopia_tuple = tritanopia.as_tuple();
	///
	/// assert_eq!(tritanopia_tuple.0, ConeCellCond::Normal);
	/// assert_eq!(tritanopia_tuple.1, ConeCellCond::Normal);
	/// assert_eq!(tritanopia_tuple.2, ConeCellCond::Missing);
	/// ```
	pub const fn as_tuple(&self) -> (ConeCellCond, ConeCellCond, ConeCellCond) {
		(self.l, self.m, self.s)
	}

	/// Converts to an array of 3 [`ConeCellCond`] elements
	/// in descending order (long, medium, short)
	///
	/// ```
	/// use achroma::{ConeCellSummary, ConeCellCond};
	///
	/// let protanomaly = ConeCellSummary::new(
	///    ConeCellCond::Anomalous,
	///    ConeCellCond::Normal,
	///    ConeCellCond::Normal,
	/// );
	/// let protanomaly_array = protanomaly.as_array();
	///
	/// assert_eq!(protanomaly_array[0], ConeCellCond::Anomalous);
	/// assert_eq!(protanomaly_array[1], ConeCellCond::Normal);
	/// assert_eq!(protanomaly_array[2], ConeCellCond::Normal);
	/// ```
	pub const fn as_array(&self) -> [ConeCellCond; 3] {
		[self.l, self.m, self.s]
	}
}

impl Default for ConeCellSummary {
	fn default() -> Self {
		Self::new(
			ConeCellCond::default(),
			ConeCellCond::default(),
			ConeCellCond::default(),
		)
	}
}

impl From<(ConeCellCond, ConeCellCond, ConeCellCond)> for ConeCellSummary {
	fn from(v: (ConeCellCond, ConeCellCond, ConeCellCond)) -> Self {
		Self::new(v.0, v.1, v.2)
	}
}

impl From<[ConeCellCond; 3]> for ConeCellSummary {
	fn from(v: [ConeCellCond; 3]) -> Self {
		Self::new(v[0], v[1], v[2])
	}
}

impl Index<usize> for ConeCellSummary {
	type Output = ConeCellCond;
	fn index(&self, index: usize) -> &ConeCellCond {
		match index {
			0 => &self.l,
			1 => &self.m,
			2 => &self.s,
			n => panic!("Invalid index: {}", n),
		}
	}
}

impl IndexMut<usize> for ConeCellSummary {
	fn index_mut(&mut self, index: usize) -> &mut ConeCellCond {
		match index {
			0 => &mut self.l,
			1 => &mut self.m,
			2 => &mut self.s,
			n => panic!("Invalid index: {}", n),
		}
	}
}

impl Index<char> for ConeCellSummary {
	type Output = ConeCellCond;
	fn index(&self, index: char) -> &ConeCellCond {
		match index {
			'l' | 'L' | 'r' | 'R' => &self.l,
			'm' | 'M' | 'g' | 'G' => &self.m,
			's' | 'S' | 'b' | 'B' => &self.s,
			n => panic!("Invalid index: {}", n),
		}
	}
}

impl IndexMut<char> for ConeCellSummary {
	fn index_mut(&mut self, index: char) -> &mut ConeCellCond {
		match index {
			'l' | 'L' | 'r' | 'R' => &mut self.l,
			'm' | 'M' | 'g' | 'G' => &mut self.m,
			's' | 'S' | 'b' | 'B' => &mut self.s,
			n => panic!("Invalid index: {}", n),
		}
	}
}

impl From<ColorVision> for ConeCellSummary {
	fn from(vision: ColorVision) -> Self {
		match vision {
			ColorVision::Normal => ConeCellSummary::NORMAL,
			ColorVision::Protanomaly => ConeCellSummary::PROTANOMALY,
			ColorVision::Protanopia => ConeCellSummary::PROTANOPIA,
			ColorVision::Deuteranomaly => ConeCellSummary::DEUTERANOMALY,
			ColorVision::Deuteranopia => ConeCellSummary::DEUTERANOPIA,
			ColorVision::Tritanomaly => ConeCellSummary::TRITANOMALY,
			ColorVision::Tritanopia => ConeCellSummary::TRITANOPIA,
			ColorVision::Achromatomaly => ConeCellSummary::ACHROMATOMALY,
			ColorVision::Achromatopsia => ConeCellSummary::ACHROMATOPSIA,
		}
	}
}

/// Various types of color vision, including normal trichromatic vision and
/// the types of color vision deficiency (CVD)
///
/// ### Summary of cones for all types of color vision
/// | Color vision  | L cone cell    | M cone cell   | S cone cell    |
/// | ------------- | -------------- | ------------- | -------------- |
/// | Normal        | ✅ Normal      | ✅ Normal      | ✅ Normal      |
/// | Protanomaly   | 🔴 Anomalous   | ✅ Normal      | ✅ Normal      |
/// | Protanopia    | ⭕ Missing     | ✅ Normal      | ✅ Normal      |
/// | Deuteranomaly | ✅ Normal      | 🔴 Anomalous   | ✅ Normal      |
/// | Deuteranopia  | ✅ Normal      | ⭕ Missing     | ✅ Normal      |
/// | Tritanomaly   | ✅ Normal      | ✅ Normal      | 🔴 Anomalous   |
/// | Tritanopia    | ✅ Normal      | ✅ Normal      | ⭕ Missing     |
/// | Achromatomaly | ⭕ Missing     | ⭕ Missing     | ✅ Normal      |
/// | Achromatopsia | ⭕ Missing     | ⭕ Missing     | ⭕ Missing     |
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ColorVision {
	/// Normal trichromatic vision, where all three cone cells are normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ✅ Normal      | ✅ Normal      | ✅ Normal      |
	#[default]
	Normal = 0,
	/// **Protanomaly** is a type of red-green color blindness
	/// that causes reduced sensitivity to red light.
	///
	/// This occurs when the long cone cell is anomalous,
	/// and the medium and short cone cells are normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | 🔴 Anomalous   | ✅ Normal      | ✅ Normal      |
	Protanomaly = 1,
	/// **Protanopia** is a type of red-green color blindness
	/// that causes complete insensitivity/blindness to red light.
	///
	/// This occurs when the long cone cell is missing,
	/// and the medium and short cone cells are normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ⭕ Missing     | ✅ Normal      | ✅ Normal      |
	Protanopia = 2,
	/// **Deuteranomaly** is a type of red-green color blindness
	/// that causes reduced sensitivity to green light.
	///
	/// This occurs when the medium cone cell is anomalous,
	/// and the long and short cone cells are normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ✅ Normal      | 🔴 Anomalous   | ✅ Normal      |
	Deuteranomaly = 4,
	/// **Deuteranopia** is a type of red-green color blindness
	/// that cause complete insensitivity/blindness to green light.
	///
	/// This occurs when the medium cone cell is missing,
	/// and the long and short cone cells are normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ✅ Normal      | ⭕ Missing     | ✅ Normal      |
	Deuteranopia = 8,
	/// **Tritanomaly** is a type of blue-yellow color blindness
	/// that causes reduced sensitivity to blue light.
	///
	/// This occurs when the short cone cell is anomalous,
	/// and the long and medium cone cells are normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ✅ Normal      | ✅ Normal      | 🔴 Anomalous   |
	Tritanomaly = 16,
	/// **Tritanopia** is a type of blue-yellow color blindness
	/// that causes complete insensitivity/blindness to blue light.
	///
	/// This occurs when the short cone cell is missing,
	/// and the long and medium cone cells are normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ✅ Normal      | ✅ Normal      | ⭕ Missing     |
	Tritanopia = 32,
	/// **Achromatomaly** is a type of monochromacy that causes
	/// reduced sensitivity to all colors.
	///
	/// This occurs when the long and medium cone cells are missing,
	/// and the short cone cell is normal.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ⭕ Missing     | ⭕ Missing     | ✅ Normal      |
	Achromatomaly = 64,
	/// **Achromatopsia** is a type of monochromacy that causes
	/// complete insensitivity/blindness to all colors.
	///
	/// This occurs when all cone cells are missing.
	///
	/// | L cone cell   | M cone cell    | S cone cell    |
	/// | ------------- | -------------- | -------------- |
	/// | ⭕ Missing     | ⭕ Missing     | ⭕ Missing     |
	Achromatopsia = 128,
}

impl ColorVision {
	/// Returns whether the color vision is red-green color vision deficiency (CVD).
	///
	/// This returns true if the color vision is either protanomaly, protanopia,
	/// deuteranomaly, or deuteranopia.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Protanomaly.is_red_green(), true);
	/// assert_eq!(ColorVision::Protanopia.is_red_green(), true);
	/// assert_eq!(ColorVision::Deuteranomaly.is_red_green(), true);
	/// assert_eq!(ColorVision::Deuteranopia.is_red_green(), true);
	/// assert_eq!(ColorVision::Tritanomaly.is_red_green(), false);
	/// ```
	pub const fn is_red_green(&self) -> bool {
		self.is_protan() || self.is_deutan()
	}

	/// Returns whether the color vision is blue-yellow color vision deficiency (CVD)
	///
	/// This returns true if the color vision is either tritanomaly or tritanopia.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Protanopia.is_blue_yellow(), false);
	/// assert_eq!(ColorVision::Deuteranomaly.is_blue_yellow(), false);
	/// assert_eq!(ColorVision::Tritanomaly.is_blue_yellow(), true);
	/// assert_eq!(ColorVision::Tritanopia.is_blue_yellow(), true);
	/// ```
	pub const fn is_blue_yellow(&self) -> bool {
		self.is_tritan()
	}

	/// Returns whether the color vision is protan. Protan color vision deficiency
	/// occurs when the long cone cell is either anomalous or missing.
	///
	/// This returns true if the color vision is either protanomaly or protanopia.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Protanomaly.is_protan(), true);
	/// assert_eq!(ColorVision::Protanopia.is_protan(), true);
	/// ```
	pub const fn is_protan(&self) -> bool {
		matches!(self, Self::Protanomaly | Self::Protanopia)
	}

	/// Returns whether the color vision is deutan. Deutan color vision deficiency
	/// occurs when the medium cone cell is either anomalous or missing.
	///
	/// This returns true if the color vision is either deuteranomaly or deuteranopia.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Deuteranomaly.is_deutan(), true);
	/// assert_eq!(ColorVision::Deuteranopia.is_deutan(), true);
	/// ```
	pub const fn is_deutan(&self) -> bool {
		matches!(self, Self::Deuteranomaly | Self::Deuteranopia)
	}

	/// Returns whether the color vision is tritan. Tritan color vision deficiency
	/// occurs when the short cone cell is either anomalous or missing.
	///
	/// This returns true if the color vision is either tritanomaly or tritanopia.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Tritanomaly.is_tritan(), true);
	/// assert_eq!(ColorVision::Tritanopia.is_tritan(), true);
	/// ```
	pub const fn is_tritan(&self) -> bool {
		matches!(self, Self::Tritanomaly | Self::Tritanopia)
	}

	/// Returns whether the color vision is achroma. Achroma color vision deficiency
	/// occurs when all cone cells are missing, or when the short cone cell is normal.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Achromatomaly.is_monochromacy(), true);
	/// assert_eq!(ColorVision::Achromatopsia.is_monochromacy(), true);
	/// ```
	pub const fn is_monochromacy(&self) -> bool {
		matches!(self, Self::Achromatomaly | Self::Achromatopsia)
	}

	/// Returns whether the color vision is anomalous trichromacy. Anomalous trichromacy
	/// occurs when one of the cone cells is anomalous, and the other two are normal.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Protanomaly.is_anomalous_trichromacy(), true);
	/// assert_eq!(ColorVision::Deuteranomaly.is_anomalous_trichromacy(), true);
	/// assert_eq!(ColorVision::Tritanomaly.is_anomalous_trichromacy(), true);
	/// ```
	pub const fn is_anomalous_trichromacy(&self) -> bool {
		matches!(
			self,
			Self::Protanomaly | Self::Deuteranomaly | Self::Tritanomaly
		)
	}

	/// Returns whether the color vision is dichromacy. Dichromacy occurs when one of
	/// the cone cells are missing, and the other two are normal.
	///
	/// ```
	/// use achroma::ColorVision;
	///
	/// assert_eq!(ColorVision::Protanopia.is_dichromacy(), true);
	/// assert_eq!(ColorVision::Deuteranopia.is_dichromacy(), true);
	/// ```
	pub const fn is_dichromacy(&self) -> bool {
		matches!(
			self,
			Self::Protanopia | Self::Deuteranopia | Self::Tritanopia
		)
	}
}

impl TryFrom<ConeCellSummary> for ColorVision {
	type Error = ();
	fn try_from(summary: ConeCellSummary) -> Result<Self, Self::Error> {
		match summary {
			ConeCellSummary::NORMAL => Ok(Self::Normal),
			ConeCellSummary::PROTANOMALY => Ok(Self::Protanomaly),
			ConeCellSummary::PROTANOPIA => Ok(Self::Protanopia),
			ConeCellSummary::DEUTERANOMALY => Ok(Self::Deuteranomaly),
			ConeCellSummary::DEUTERANOPIA => Ok(Self::Deuteranopia),
			ConeCellSummary::TRITANOMALY => Ok(Self::Tritanomaly),
			ConeCellSummary::TRITANOPIA => Ok(Self::Tritanopia),
			ConeCellSummary::ACHROMATOMALY => Ok(Self::Achromatomaly),
			ConeCellSummary::ACHROMATOPSIA => Ok(Self::Achromatopsia),
			_ => Err(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_summary_default() {
		let normal = ConeCellSummary::default();
		assert_eq!(normal.l, ConeCellCond::Normal);
		assert_eq!(normal.m, ConeCellCond::Normal);
		assert_eq!(normal.s, ConeCellCond::Normal);
	}

	#[test]
	fn test_summary_index_usize() {
		let tritanopia = ConeCellSummary::new(
			ConeCellCond::Normal,
			ConeCellCond::Normal,
			ConeCellCond::Anomalous,
		);

		assert_eq!(tritanopia[0], tritanopia.l);
		assert_eq!(tritanopia[1], tritanopia.m);
		assert_eq!(tritanopia[2], tritanopia.s);
	}

	#[test]
	fn test_summary_index_char() {
		let tritanopia = ConeCellSummary::new(
			ConeCellCond::Normal,
			ConeCellCond::Normal,
			ConeCellCond::Anomalous,
		);

		assert_eq!(tritanopia['l'], tritanopia.l);
		assert_eq!(tritanopia['m'], tritanopia.m);
		assert_eq!(tritanopia['s'], tritanopia.s);
	}

	#[test]
	fn test_is_cv_red_green() {
		assert!(!ColorVision::Normal.is_red_green());
		assert!(ColorVision::Protanomaly.is_red_green());
		assert!(ColorVision::Protanopia.is_red_green());
		assert!(ColorVision::Deuteranomaly.is_red_green());
		assert!(ColorVision::Deuteranopia.is_red_green());
		assert!(!ColorVision::Tritanomaly.is_red_green());
		assert!(!ColorVision::Tritanopia.is_red_green());
		assert!(!ColorVision::Achromatomaly.is_red_green());
		assert!(!ColorVision::Achromatopsia.is_red_green());
	}

	#[test]
	fn test_is_cv_blue_yellow() {
		assert!(!ColorVision::Normal.is_blue_yellow());
		assert!(!ColorVision::Protanomaly.is_blue_yellow());
		assert!(!ColorVision::Protanopia.is_blue_yellow());
		assert!(!ColorVision::Deuteranomaly.is_blue_yellow());
		assert!(!ColorVision::Deuteranopia.is_blue_yellow());
		assert!(ColorVision::Tritanomaly.is_blue_yellow());
		assert!(ColorVision::Tritanopia.is_blue_yellow());
		assert!(!ColorVision::Achromatomaly.is_blue_yellow());
		assert!(!ColorVision::Achromatopsia.is_blue_yellow());
	}

	#[test]
	fn test_is_cv_protan() {
		assert!(!ColorVision::Normal.is_protan());
		assert!(ColorVision::Protanomaly.is_protan());
		assert!(ColorVision::Protanopia.is_protan());
		assert!(!ColorVision::Deuteranomaly.is_protan());
		assert!(!ColorVision::Deuteranopia.is_protan());
		assert!(!ColorVision::Tritanomaly.is_protan());
		assert!(!ColorVision::Tritanopia.is_protan());
		assert!(!ColorVision::Achromatomaly.is_protan());
		assert!(!ColorVision::Achromatopsia.is_protan());
	}

	#[test]
	fn test_is_cv_deutan() {
		assert!(!ColorVision::Normal.is_deutan());
		assert!(!ColorVision::Protanomaly.is_deutan());
		assert!(!ColorVision::Protanopia.is_deutan());
		assert!(ColorVision::Deuteranomaly.is_deutan());
		assert!(ColorVision::Deuteranopia.is_deutan());
		assert!(!ColorVision::Tritanomaly.is_deutan());
		assert!(!ColorVision::Tritanopia.is_deutan());
		assert!(!ColorVision::Achromatomaly.is_deutan());
		assert!(!ColorVision::Achromatopsia.is_deutan());
	}

	#[test]
	fn test_is_cv_tritan() {
		assert!(!ColorVision::Normal.is_tritan());
		assert!(!ColorVision::Protanomaly.is_tritan());
		assert!(!ColorVision::Protanopia.is_tritan());
		assert!(!ColorVision::Deuteranomaly.is_tritan());
		assert!(!ColorVision::Deuteranopia.is_tritan());
		assert!(ColorVision::Tritanomaly.is_tritan());
		assert!(ColorVision::Tritanopia.is_tritan());
		assert!(!ColorVision::Achromatomaly.is_tritan());
		assert!(!ColorVision::Achromatopsia.is_tritan());
	}

	#[test]
	fn test_is_cv_monochromacy() {
		assert!(!ColorVision::Normal.is_monochromacy());
		assert!(!ColorVision::Protanomaly.is_monochromacy());
		assert!(!ColorVision::Protanopia.is_monochromacy());
		assert!(!ColorVision::Deuteranomaly.is_monochromacy());
		assert!(!ColorVision::Deuteranopia.is_monochromacy());
		assert!(!ColorVision::Tritanomaly.is_monochromacy());
		assert!(!ColorVision::Tritanopia.is_monochromacy());
		assert!(ColorVision::Achromatomaly.is_monochromacy());
		assert!(ColorVision::Achromatopsia.is_monochromacy());
	}

	#[test]
	fn test_is_cv_anomalous_trichromacy() {
		assert!(!ColorVision::Normal.is_anomalous_trichromacy());
		assert!(ColorVision::Protanomaly.is_anomalous_trichromacy());
		assert!(!ColorVision::Protanopia.is_anomalous_trichromacy());
		assert!(ColorVision::Deuteranomaly.is_anomalous_trichromacy());
		assert!(!ColorVision::Deuteranopia.is_anomalous_trichromacy());
		assert!(ColorVision::Tritanomaly.is_anomalous_trichromacy());
		assert!(!ColorVision::Tritanopia.is_anomalous_trichromacy());
		assert!(!ColorVision::Achromatomaly.is_anomalous_trichromacy());
		assert!(!ColorVision::Achromatopsia.is_anomalous_trichromacy());
	}

	#[test]
	fn test_is_dichromacy() {
		assert!(ColorVision::Protanopia.is_dichromacy());
		assert!(ColorVision::Deuteranopia.is_dichromacy());
		assert!(ColorVision::Tritanopia.is_dichromacy());
		assert!(!ColorVision::Normal.is_dichromacy());
		assert!(!ColorVision::Protanomaly.is_dichromacy());
		assert!(!ColorVision::Deuteranomaly.is_dichromacy());
		assert!(!ColorVision::Tritanomaly.is_dichromacy());
		assert!(!ColorVision::Achromatomaly.is_dichromacy());
		assert!(!ColorVision::Achromatopsia.is_dichromacy());
	}
}
