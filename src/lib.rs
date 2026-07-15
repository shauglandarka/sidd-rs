//! Sensor Independent Derived Data support
//! The primary interface for general sidd reading is `read_sidd`.
//!
//! It is a future goal to have functions for each version, but for now a single
//! function call and `match` statement are used.

#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use rayon::slice::ParallelSliceMut;
use memmap2::Mmap;
use ndarray::{azip, par_azip, Array2, ArrayView2};
use quick_xml::DeError;
use std::fs::File;
use std::path::Path;
use std::slice::from_raw_parts;
use std::str::{from_utf8, FromStr, Utf8Error};
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
pub fn read_sidd(path: &Path) -> Result<Sidd<'_>, SiddError> {
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
pub struct Sidd<'a> {
    /// Nitf file object and associated metadata
    pub nitf: Nitf,
    /// Parsed SIDD xml metadata
    pub meta: SiddMeta,
    /// SIDD Version
    pub version: SiddVersion,
    /// Image data from Nitf Image segements
    pub image_data: Vec<ImageData<'a>>,
}

#[derive(Debug)]
pub struct ImageData<'a> {
    /// SIDD uint8 raw byte array
    pub array: ArrayView2<'a, u8>,
    /// Need to hold onto this to access data
    _mmap: Mmap,
}

impl<'a> ImageData<'a> {
    fn initialize(mmap: Mmap, n_rows: usize, n_cols: usize) -> Self {
        let byte_slice = unsafe { from_raw_parts(mmap.as_ptr(), mmap.len()) };

        let array = ArrayView2::from_shape((n_rows, n_cols), byte_slice).unwrap();

        Self { array, _mmap: mmap }
    }
}

pub trait ToNative {
    /// Performs allocation of a native-aligned u8 array
    fn to_native(&self) -> Array2<u8>;
    /// Performs allocation of a native-aligned u8 array using `rayon`
    fn par_to_native(&self) -> Array2<u8>;
}

impl<'a> ToNative for ArrayView2<'a, u8> {
    fn to_native(&self) -> Array2<u8> {
        self.to_owned()
    }

    fn par_to_native(&self) -> Array2<u8> {
        self.to_owned()
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

impl<'a> Sidd<'a> {
    pub fn from_file(mut file: File) -> Result<Self, SiddError> {
        let nitf = Nitf::from_reader(&mut file)?;
        if nitf.nitf_header.numdes.val == 0 {
            return Err(SiddError::NotASidd)
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
    // This feels bad
    let tmp: VersionGetter = from_str(sidd_str)?;
    let sidd_version = SiddVersion::from_str(&tmp.version)?;
    use SiddError::Unimpl;
    match sidd_version {
        SiddVersion::V1_0_0 => Ok((SiddVersion::V1_0_0, SiddMeta::V1_0_0(from_str(sidd_str)?))),
        SiddVersion::V2_0_0 => Err(Unimpl("V2_0_0".to_string())),
    }
}


