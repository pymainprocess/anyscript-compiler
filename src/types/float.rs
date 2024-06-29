use crate::types::prelude::*;
use crate::alias::*;
use std::ffi::CString;

pub struct Float<T1> {
    pub number: FloatAlias,
    pub original: T1,
    pub size: IntAlias,
    pub type_id: String,
}