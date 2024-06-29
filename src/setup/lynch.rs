use crate::alias::*;
use crate::types::prelude::*;

pub struct LynchCoder {
    pub base64_byte: Vec<u8>,
    pub base64_string: String,
    pub lynch_byte: Vec<Float<FloatAlias>>,
    pub lynch_string: String,
    pub lynch_code: List<FloatAlias>,
}

impl LynchCoder {
    /// # Create a new Instance
    /// 
    /// For example
    /// 
    /// ```
    /// let lynch = LynchCoder::new(
    ///     Vec::new(),
    ///     String::new(),
    ///     Vec::new(),
    ///     String::new(),
    ///     List::new(),
    /// );
    /// ```
    pub fn new(
        base64_byte: Vec<u8>,
        base64_string: String,
        lynch_byte: Vec<Float<FloatAlias>>,
        lynch_string: String,
        lynch_code: List<FloatAlias>,
    ) -> Self {
        Self {
            base64_byte,
            base64_string,
            lynch_byte,
            lynch_string,
            lynch_code,
        }
    }

    pub fn from_existing(coder: &LynchCoder) -> Self {
        Self {
            base64_byte: coder.base64_byte.clone(),
            base64_string: coder.base64_string.clone(),
            lynch_byte: coder.lynch_byte.clone(),
            lynch_string: coder.lynch_string.clone(),
            lynch_code: coder.lynch_code.clone(),
        }
    }
}