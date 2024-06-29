use crate::types::prelude::*;
use crate::alias::*;
use base64;
use std::ffi::CString;

pub struct LynchCoder {
    pub instance: String,
}

impl LynchCoder {

    /// Create a New Instance, this is a instance without content, more later
    pub fn new() -> Self {
        Self {
            instance: String::new(),
        }
    }

    /// Create a New Instance from a CString
    pub fn from_cstring(cstring: CString) -> Self {
        Self {
            instance: cstring.to_str().unwrap().to_string(),
        }
    }

    /// Create a New Instance from a String
    pub fn from_string(string: String) -> Self {
        Self {
            instance: string,
        }
    }

    /// Create a New Instance from a Lynch String
    pub fn from_lynch_str(string: String) -> Self {
        Self {
            instance: string,
        }
    }

    /// Create a New Instance from a Lynch CString
    pub fn from_lynch_cstr(cstring: CString) -> Self {
        Self {
            instance: cstring.to_str().unwrap().to_string(),
        }
    }
}