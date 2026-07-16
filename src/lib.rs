//! Sensor Independent Derived Data support
//! The primary interface for general sidd reading is `read_sidd`.
//!
//! It is a future goal to have functions for each version, but for now a single
//! function call and `match` statement are used.

#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use memmap2::Mmap;
use ndarray::{Array2, ArrayView2, ArrayView4, azip, par_azip, s};
use quick_xml::DeError;
use rayon::slice::ParallelSliceMut;
use std::fs::File;
use std::path::Path;
use std::slice::from_raw_parts;
use std::str::{FromStr, Utf8Error, from_utf8};
use zerocopy::{BE, F32};

use num_complex::{Complex, Complex32};
use quick_xml::de::from_str;
use serde::Deserialize;
use thiserror::Error;

use nitf_rs::{Nitf, NitfError};

pub mod v1_0_0;

/// Construct a [Sidd] object from a file `path`.
///
/// This is specifically for cases where the version of the Sidd is not known
/// and makes use of several `enums` to parse the data.
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// use sidd_rs::SiddVersion;
///
/// let sidd_path = Path::new("../example.nitf");
/// let sidd = sidd_rs::read_sidd(sidd_path).unwrap();
/// // Then use convenience methods provided by SiddMeta enum, or match off of version
/// let meta = sidd.meta.get_v1_0_0_meta();
///
/// ```
///
pub fn read_sidd(path: &Path) -> Result<Sidd, SiddError> {
    let file = File::open(path)?;
    Sidd::from_file(file)
}

#[derive(Error, Debug)]
pub enum SiddError {
    #[error("unknown sidd version {0}")]
    VersionError(String),
    #[error("metadata for version {0} is not implemented")]
    Unimpl(String),
    #[error("file does not appear to be a SIDD")]
    NotASidd,
    // Wrappers for built in errors
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    NitfError(#[from] NitfError),
    #[error(transparent)]
    UTF8(#[from] Utf8Error),
    #[error(transparent)]
    DESER(#[from] DeError),
}

/// SIDD file structure
// TODO: Implement printing (Debug, Display?)
pub struct Sidd {
    /// Nitf file object and associated metadata
    pub nitf: Nitf,
    /// Parsed SIDD xml metadata
    pub meta: SiddMeta,
    /// SIDD Version
    pub version: SiddVersion,
    /// Image data from Nitf Image segements
    pub image_data: Vec<ImageData>,
}

#[derive(Debug)]
pub struct ImageData {
    /// SIDD uint8 raw byte array
    /// Returning owned array, so no need for lifetimes
    pub array: Array2<u8>,
}

impl ImageData {
    pub fn initialize(
        mmap: Mmap,
        nbpr: usize,
        nbpc: usize,
        nppbh: usize,
        nppbv: usize,
        n_rows: usize,
        n_cols: usize,
    ) -> Self {
        // Image is stored in blocks, each block is row contiguous.
        // Each block is nppbv x nppbh  pixels.
        // There are nbpc blocks in the vertical
        // There are nbpl blocks in the horizontal
        // Map the flat bytes to the block structure
        let byte_slice = unsafe { std::slice::from_raw_parts(mmap.as_ptr(), mmap.len()) };

        let blocks =
            ArrayView4::from_shape((nbpc, nbpr, nppbv, nppbh), byte_slice).expect("Shape mismatch");

        //Permute and collapse. First heap allocation
        let stitched = blocks
            .permuted_axes([0, 2, 1, 3])
            .as_standard_layout()
            .into_owned()
            .into_shape((nbpc * nppbv, nbpr * nppbh))
            .expect("Failed to reshape into 2D array");

        // Crop it: take a slice and turn it into an owned Array2. Second heap allocation
        let cropped = stitched.slice(s![..n_rows, ..n_cols]).to_owned();

        Self { array: cropped }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum SiddVersion {
    V1_0_0,
    V2_0_0,
}

impl FromStr for SiddVersion {
    type Err = SiddError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("urn:SIDD:").collect::<String>().as_str() {
            "1.0.0" => Ok(SiddVersion::V1_0_0),
            "2.0.0" => Ok(SiddVersion::V2_0_0),
            _ => Err(SiddError::VersionError(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum SiddMeta {
    V1_0_0(v1_0_0::SiddMeta),
    V2_0_0, //Not Implemented
}

impl SiddMeta {
    pub fn get_v1_0_0_meta(self) -> Option<v1_0_0::SiddMeta> {
        match self {
            Self::V1_0_0(meta) => Some(meta),
            _ => None,
        }
    }
    pub fn get_v2_0_0_meta(self) -> SiddError {
        SiddError::Unimpl("2.0.0".to_string())
    }
}

impl Sidd {
    pub fn from_file(mut file: File) -> Result<Self, SiddError> {
        let nitf = Nitf::from_reader(&mut file)?;
        if nitf.nitf_header.numdes.val == 0 {
            return Err(SiddError::NotASidd);
        }
        let dex_data = nitf.data_extension_segments[0].get_data_map(&mut file)?;
        let sidd_str = from_utf8(&dex_data[..])?;
        let (version, meta) = parse_sidd(sidd_str)?;

        let image_data: Vec<_> = nitf
            .image_segments
            .iter()
            .map(|seg| {
                ImageData::initialize(
                    seg.get_data_map(&mut file).unwrap(),
                    seg.header.nbpr.val as usize,
                    seg.header.nbpc.val as usize,
                    seg.header.nppbh.val as usize,
                    seg.header.nppbv.val as usize,
                    seg.header.nrows.val as usize,
                    seg.header.ncols.val as usize,
                )
            })
            .collect();

        Ok(Self {
            nitf,
            meta,
            version,
            image_data,
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VersionGetter {
    #[serde(rename = "@xmlns")]
    pub version: String,
}

fn parse_sidd(sidd_str: &str) -> Result<(SiddVersion, SiddMeta), SiddError> {
    let tmp: VersionGetter = from_str(sidd_str)?;
    let sidd_version = SiddVersion::from_str(&tmp.version)?;
    use SiddError::Unimpl;
    match sidd_version {
        SiddVersion::V1_0_0 => Ok((SiddVersion::V1_0_0, SiddMeta::V1_0_0(from_str(sidd_str)?))),
        SiddVersion::V2_0_0 => Err(Unimpl("V2_0_0".to_string())),
    }
}
