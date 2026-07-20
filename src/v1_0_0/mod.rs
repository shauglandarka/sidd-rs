use serde::Deserialize;
use ndarray::Array1;

pub mod display;
pub mod exploitation_features;
pub mod geographic_and_target;
pub mod measurement;
pub mod product_creation;
pub mod product_processing;

pub use display::Display;
pub use exploitation_features::ExploitationFeatures;
pub use geographic_and_target::GeographicAndTarget;
pub use measurement::Measurement;
pub use product_creation::ProductCreation;
pub use product_processing::ProductProcessing;

// Haven't done these yet

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct DownstreamReprocessing {}
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct Annotations {}
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct ErrorStatistics {}
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct NITF {}
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct _NITF {}
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct Radiometric {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SiddMeta {
    #[serde(rename = "ProductCreation")]
    pub product_creation: ProductCreation,
    #[serde(rename = "ExploitationFeatures")]
    pub exploitation_features: ExploitationFeatures,
    #[serde(rename = "GeographicAndTarget")]
    pub geographic_and_target: GeographicAndTarget,
    #[serde(rename = "Display")]
    pub display: Display,
    #[serde(rename = "ProductProcessing")]
    pub product_processing: ProductProcessing,
    #[serde(rename = "Measurement")]
    pub measurement: Measurement,
    #[serde(rename = "DownstreamReprocessing")]
    pub downstream_reprocessing: Option<DownstreamReprocessing>,
    #[serde(rename = "Annotations")]
    pub annotations: Option<Annotations>,
    #[serde(rename = "ErrorStatistics")]
    pub error_statistics: Option<ErrorStatistics>,
    #[serde(rename = "NITF")]
    pub nitf: Option<NITF>,
    #[serde(rename = "_NITF")]
    pub _nitf: Option<_NITF>,
    #[serde(rename = "Radiometric")]
    pub radiometric: Option<Radiometric>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Vector3 {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ArpPoly {
    #[serde(rename = "X")]
    pub x: PolyDim,
    #[serde(rename = "Y")]
    pub y: PolyDim,
    #[serde(rename = "Z")]
    pub z: PolyDim,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PolyDim {
    #[serde(rename = "@order1")]
    pub order1: String,
    #[serde(rename = "Coef")]
    pub coeffs: Vec<Coef>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TimeCoaPoly {
    #[serde(rename = "@order1")]
    pub order1: String,
    #[serde(rename = "@order2")]
    pub order2: String,
    #[serde(rename = "Coef")]
    pub coef: Coef,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef {
    #[serde(rename = "@exponent1")]
    pub exponent1: String,
    #[serde(rename = "@exponent2", default)]
    pub exponent2: Option<String>,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RowCol {
    #[serde(rename = "Row")]
    pub row: f64,
    #[serde(rename = "Col")]
    pub col: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RowColInt {
    #[serde(rename = "Row")]
    pub row: u32,
    #[serde(rename = "Col")]
    pub col: u32,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Ecef {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
}

impl Ecef {
    pub fn to_array(&self) -> Array1<f64> {
        Array1::from_vec(vec![self.x, self.y, self.z])
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AngleMagnitude {
    #[serde(rename = "Angle")]
    pub angle: f64,
    #[serde(rename = "Magnitude")]
    pub magnitude: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RangeAzimuth {
    #[serde(rename = "Range")]
    pub range: f64,
    #[serde(rename = "Azimuth")]
    pub azimuth: f64,
}
