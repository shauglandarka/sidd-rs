#![allow(unused_imports)]
#![allow(non_camel_case_types)]

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
