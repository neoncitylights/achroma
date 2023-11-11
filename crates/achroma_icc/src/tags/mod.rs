mod arrays;
mod macros;

pub use self::arrays::*;
use crate::numbers::*;
use crate::impl_enum;

// Table 26
impl_enum! {
	u32,
	ColorimetricIntentImageStateTag,
	SceneColorimetryEstimates: 0x73636565: "scene",
	SceneAppearanceEstimates: 0x73617065: "sape",
	FocalPlaneColorimetryEstimates: 0x66706565: "fpce",
	ReflectionHardcopyOriginalColorimetry: 0x72686F63: "rhoc",
	ReflectionPrintOutputColorimetry: 0x7269706F: "rpoc"
}

// Table 29
impl_enum! {
	u32,
	TechnologySignature,
	FilmScanner: 0x6673636E: "fscn",
	DigitalCamera: 0x6463616D: "dcam",
	ReflectiveScanner: 0x7273636E: "rscn",
	InkJetPrinter: 0x696A6574: "ijet",
	ThermalWaxPrinter: 0x74776178: "twax",
	ElectrophotographicPrinter: 0x6570686F: "epho",
	ElectrostaticPrinter: 0x65737461: "esta",
	DyeSublimationPrinter: 0x64737562: "dsub",
	PhotographicPaperPrinter: 0x70687072: "rpho",
	FilmWriter: 0x6670726E: "fprn",
	VideoMonitor: 0x7669646D: "vidm",
	VideoCamera: 0x76696463: "vidc",
	ProjectionTelevision: 0x706A7476: "pjtv",
	CRTDisplay: 0x43525420: "CRT ",
	PMDisplay: 0x504D4420: "PMD ",
	AMDisplay: 0x414D4420: "AMD ",
	OLEDDisplay: 0x4F4C4544: "OLED",
	PhotoCD: 0x4B504344: "KPCD",
	PhotoImageSetter: 0x696D6773: "imgs",
	Gravure: 0x67726176: "grav",
	OffsetLithography: 0x6F666673: "offs",
	SilkScreen: 0x73696C6B: "silk",
	Flexography: 0x666C6578: "flex",
	MotionPictureFilmScanner: 0x6D706673: "mpfs",
	MotionPictureFilmRecorder: 0x6D706672: "mpfr",
	DigitalMotionPictureCamera: 0x646D7063: "dmpc",
	DigitalCinemaProjector: 0x64636D70: "dcmp"
}

// Table 30
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Chromaticity {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub device_channels: u16,
	pub phosphor_colorant: PhosphorColorant,
	pub channel_1_ciexy_coord: [U16Fixed16; 2],
	pub channel_ciexy_coords: Option<Vec<[U16Fixed16; 2]>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
pub enum PhosphorColorant {
	Unknown = 0x0000,
	ItuRBt709 = 0x0001,
	SmpteRp145 = 0x0002,
}

// Table 32
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cicp {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub color_primaries: u8,
	pub transfer_characteristics: u8,
	pub matrix_coefficients: u8,
	pub video_full_range_flag: u8,
}

// Table 33
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ColorantOrder {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub colorants_count: u32,
	pub colorant_num_fp: u8,
	pub colorants: Vec<u8>,
}

// Table 34
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ColorantTable {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub colorants_count: u32,
	pub entries_count: u32,
	pub curve_values: Vec<u16>,
}

// Table 36
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DataType {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub data_flag: u32,
}

// Table 40
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lut16 {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u8,
	pub output_channels: u8,
	pub clut_grid_points: u8,
	reserved_2: u8,
	pub encoded_e1p: S15Fixed16,
	pub encoded_e2p: S15Fixed16,
	pub encoded_e3p: S15Fixed16,
	pub encoded_e4p: S15Fixed16,
	pub encoded_e5p: S15Fixed16,
	pub encoded_e6p: S15Fixed16,
	pub encoded_e7p: S15Fixed16,
	pub encoded_e8p: S15Fixed16,
	pub encoded_e9p: S15Fixed16,
	pub input_table_entries: u16,
	pub output_table_entries: u16,
	pub input_values: Vec<u16>,
	pub clut_values: Vec<u16>,
	pub output_tables: Vec<u16>,
}

// Table 44
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lut8 {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u8,
	pub output_channels: u8,
	pub clut_grid_points: u8,
	reserved_2: u8,
	pub encoded_e1p: S15Fixed16,
	pub encoded_e2p: S15Fixed16,
	pub encoded_e3p: S15Fixed16,
	pub encoded_e4p: S15Fixed16,
	pub encoded_e5p: S15Fixed16,
	pub encoded_e6p: S15Fixed16,
	pub encoded_e7p: S15Fixed16,
	pub encoded_e8p: S15Fixed16,
	pub encoded_e9p: S15Fixed16,
	pub input_tables: u16,
	pub clut_values: Vec<u16>,
	pub output_tables: Vec<u16>,
}

// Table 45
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LutAToB {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u8,
	pub output_channels: u8,
	reserved_2: [u8; 2],
	pub offset_first_b_curve: u32,
}

// Table 46
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LutAToBClut {
	pub grid_points: [u8; 16],
	pub precision: u8,
	pub(crate) reserved_1: [u8; 3],
	pub clut_data_points: Vec<u8>, // Or Vec<u16>?
}

// Table 47
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LutBToA {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u8,
	pub output_channels: u8,
	reserved_2: [u16; 2],
	pub offset_first_b_curve: u32,
	pub offset_matrix: u32,
	pub offset_first_m_curve: u32,
	pub offset_clut: u32,
	pub offset_first_a_curve: u32,
}

// Table 49
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Measurement {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub std_observer: StandardObserver,
	pub tristimulus_values: XYZNum,
	pub measurement_geometry: MeasurementGeometry,
	pub measurement_flare: MeasurementFlare,
	pub standard_illuminant: StandardIlluminant,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MeasurementGeometry {
	Unknown,
	Deg045,
	Deg0D,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StandardObserver {
	Unknown = 0x00000000,
	Cie1931StdColorimetricObserver = 0x00000001,
	Cie1964StdColorimetricObserver = 0x00000002,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MeasurementFlare(U16Fixed16);

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StandardIlluminant {
	Zero = 0x00000000,
	OneHundred = 0x00010000,
}

// Table 54
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultiLocalizedUnicode {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub count_records: u32,
	pub record_size: u32,
	pub record_1_lang_code: u16,
	pub record_1_country_code: u16,
	pub record_1_str_length: u32,
	pub record_1_str_offset: u32,
	pub records: Vec<u16>, // Is it Vec<u16>?
}

// Table 55
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultiProcessElements {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u16,
	pub output_channels: u16,
	pub processing_elements: u32,
	pub positions_table: Vec<PositionNum>,
	// data field?
}

// Table 56
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GeneralElement {
	pub element_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u16,
	pub output_channels: u16,
	// data field?
}

// Table 57
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurveSetElement {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u16,
	pub output_channels: u16,
	pub curve_positions: Vec<PositionNum>,
	// data field?
}

// Table 58
#[derive(Debug, Clone, PartialEq)]
pub struct D1Curve {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub segments: u16,
	reserved_2: u16,
	pub break_points: Vec<f32>,
	// segments field?
}

// Table 59
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaCurveSegment {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub function_type: u16,
	pub reserved_2: u16,
	pub num_params: u16,
	pub params: Vec<f32>,
}

// Table 61
#[derive(Debug, Clone, PartialEq)]
pub struct SampledCurveSegment {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub count_entries: u32,
	pub curve_entries: Vec<f32>,
}

// Table 62
#[derive(Debug, Clone, PartialEq)]
pub struct MatrixElement {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u16,
	pub output_channels: u16,
	pub elements: Vec<f32>,
}

// Table 63
#[derive(Debug, Clone, PartialEq)]
pub struct ClutElement {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u16,
	pub output_channels: u16,
	pub grid_points: u8,
	pub data_points: Vec<f32>,
}

// Table 64
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BacsElement {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u16,
	pub output_channels: u16,
	pub signature: u32,
}

// Table 65
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EacsElement {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub input_channels: u16,
	pub output_channels: u16,
	pub signature: u32,
}

// Table 66
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamedColor2 {
	pub count_colors: u32,
	pub count_device_coords: u32,
	pub color_name_prefix: Bit7Ascii,
	pub color_name_suffix: Bit7Ascii,
	pub first_color_name_root: Bit7Ascii,
	pub first_color_name_pcs_coords: [u16; 3],
	pub first_named_color_device_coords: Vec<u16>,
}

// Table 67
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ParametricCurve {
	pub para_signature: u32,
	pub(crate) reserved_1: u32,
	pub encoded_function: u16,
	reserved_2: u32,
}

// Table 69
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ProfileSequenceDesc();

// Table 70
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ProfileDescriptionSignature();

// Table 71
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProfileSequenceIdentifier {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub count: u32,
	pub positions: Vec<PositionNum>,
}

// Table 72
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProfileIdentifier {
	pub id: u16,
	pub desc: MultiLocalizedUnicode,
}

// Table 73
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ResponseCurveSet16<const N: usize> {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub channels: u16,
	pub measurement_types: u32,
	pub offsets: [u32; N],
}

// Table 74
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CurveStructure<const N: usize, const P: usize> {
	pub measurement_unit_signature: CurveMeasurement,
	pub count_measurements: [u32; N],
	pub pcsxyz_values: [u32; N],
	pub response_arrays: [Response16; P],
}

// Table 75
impl_enum! {
	u32,
	CurveMeasurement,
	StatusA: 0x53746141: "StaA",
	StatusE: 0x53746145: "StaE",
	StatusI: 0x53746149: "StaI",
	StatusT: 0x53746154: "StaT",
	StatusM: 0x5374614D: "StaM",
	DinEPolarFilter: 0x444E2020: "Dn  ",
	DinENoPolarFilter: 0x444E2050: "Dn P",
	DinIPolarFilter: 0x444E4E20: "DNN ",
	DinINoPolarFilter: 0x444E4E50: "DNNP"
}

// Table 77
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Signature {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub signature: u32,
}

// Table 78
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub text: Vec<Bit7Ascii>,
}

// Table 84
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ViewingConditions {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub ciexyz_illuminant: XYZNum,
	pub ciexyz_surround: XYZNum,
	pub illuminant_type: Measurement,
}

// Table 85
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XYZType {
	pub(crate) type_signature: u32,
	pub(crate) reserved_1: u32,
	pub values: Vec<XYZNum>,
}
