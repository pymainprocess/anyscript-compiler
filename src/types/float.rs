use std::ffi::CString;
use crate::alias::*;
use crate::types::prelude::*;
use std::hash::Hash;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::any::TypeId;
use std::str::FromStr;

#[derive(Clone)]
pub struct Float<T1> {
    pub floating: FloatAlias,
    pub size: FloatAlias,
    pub original: T1,
    pub type_id: String,
}

impl<T1: 'static + Default + Display + Eq + Hash + Clone + Send + Sync> Default for Float<T1> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T1: 'static + Default + Display + Eq + Hash + Clone + Send + Sync> Float<T1> {
    ///
    /// # Create a new instance of the class
    ///
    /// ```rust
    /// pub fn new() -> Self {
    ///     let n = T1::default();
    ///     Float {
    ///         floating: 0.0,
    ///         size: 0.0,
    ///         original: n,
    ///         type_id: String::new(),
    ///     }
    /// }
    /// ```
    /// ---
    pub fn new() -> Self {
        let n = T1::default();
        Float {
            floating: 0.0,
            size: 0.0,
            original: n,
            type_id: String::new(),
        }
    }

    ///
    /// # Reset the class
    ///
    /// ```rust
    /// pub fn reset(&mut self) {
    ///     self.floating = 0.0;
    ///     self.size = 0.0;
    ///     self.type_id = String::new();
    ///     self.original = T1::default();
    /// }
    /// ```
    /// ---
    pub fn reset(&mut self) {
        self.floating = 0.0;
        self.size = 0.0;
        self.original = T1::default();
        self.type_id = String::new();
    }

    ///
    /// # Check Valid
    ///
    /// ```rust
    /// pub fn validate(&self) -> (bool, String) {
    ///     if TypeId::of::<T1>() == TypeId::of::<FloatAlias>() {
    ///         return (true, "float".to_string());
    ///     } else if TypeId::of::<T1>() == TypeId::of::<IntAlias>() {
    ///         return (true, "int".to_string());
    ///     } else if TypeId::of::<T1>() == TypeId::of::<UIntAlias>() {
    ///         return (true, "uint".to_string());
    ///     } else if TypeId::of::<T1>() == TypeId::of::<&str>() || TypeId::of::<T1>() == TypeId::of::<String>() {
    ///         let value = self.original.to_string();
    ///         if value.parse::<FloatAlias>().is_ok() {
    ///             return (true, "float".to_string());
    ///         } else if value.parse::<IntAlias>().is_ok() {
    ///             return (true, "int".to_string());
    ///         } else if value.parse::<UIntAlias>().is_ok() {
    ///             return (true, "uint".to_string());
    ///         } else {
    ///             return (false, "invalid string".to_string());
    ///         }
    ///     } else if TypeId::of::<T1>() == TypeId::of::<CString>() {
    ///         let c_str = self.original.to_string();
    ///         if c_str.parse::<FloatAlias>().is_ok() {
    ///             return (true, "float".to_string());
    ///         } else if c_str.parse::<IntAlias>().is_ok() {
    ///             return (true, "int".to_string());
    ///         } else if c_str.parse::<UIntAlias>().is_ok() {
    ///             return (true, "uint".to_string());
    ///         } else {
    ///             return (false, "invalid CString".to_string());
    ///         }
    ///     } else if TypeId::of::<T1>() == TypeId::of::<List<T1>>() {
    ///         let _raw: List<T1> = List::new(); // Explicit type annotation
    ///         let _type = _raw.get_type();
    ///         return (true, _type);
    ///     }
    ///     (false, "unsupported type".to_string())
    /// }
    /// ```
    /// ---
    pub fn validate(&self) -> (bool, String) {
        if TypeId::of::<T1>() == TypeId::of::<FloatAlias>() {
            return (true, "float".to_string());
        } else if TypeId::of::<T1>() == TypeId::of::<IntAlias>() {
            return (true, "int".to_string());
        } else if TypeId::of::<T1>() == TypeId::of::<UIntAlias>() {
            return (true, "uint".to_string());
        } else if TypeId::of::<T1>() == TypeId::of::<&str>() || TypeId::of::<T1>() == TypeId::of::<String>() {
            let value = self.original.to_string();
            if value.parse::<FloatAlias>().is_ok() {
                return (true, "float".to_string());
            } else if value.parse::<IntAlias>().is_ok() {
                return (true, "int".to_string());
            } else if value.parse::<UIntAlias>().is_ok() {
                return (true, "uint".to_string());
            } else {
                return (false, "invalid string".to_string());
            }
        } else if TypeId::of::<T1>() == TypeId::of::<CString>() {
            let c_str = self.original.to_string();
            if c_str.parse::<FloatAlias>().is_ok() {
                return (true, "float".to_string());
            } else if c_str.parse::<IntAlias>().is_ok() {
                return (true, "int".to_string());
            } else if c_str.parse::<UIntAlias>().is_ok() {
                return (true, "uint".to_string());
            } else {
                return (false, "invalid CString".to_string());
            }
        } else if TypeId::of::<T1>() == TypeId::of::<List<T1>>() {
            let _raw: List<T1> = List::new(); // Explicit type annotation
            let _type = _raw.get_type();
            return (true, _type);
        }
        (false, "unsupported type".to_string())
    }

    /// # Convert Floating to Int, with Optional set to 32 bit or 64 bit
    /// 
    /// ```rust
    /// pub fn to_int(&mut self, bit32: Option<bool>) {
    ///     let if_32 = bit32.unwrap_or(true); // Default is true
    ///     if if_32 {
    ///         self.floating = self.floating as f32 as FloatAlias;
    ///     } else {
    ///         self.floating = self.floating as f64 as FloatAlias;
    ///     }
    /// }
    /// ```
    pub fn to_int(&mut self, bit32: Option<bool>) {
        let if_32 = bit32.unwrap_or(true); // Default is true
        if if_32 {
            self.floating = self.floating as f32 as FloatAlias;
        } else {
            self.floating = self.floating as f64 as FloatAlias;
        }
    }

    /// # Convert to String
    /// 
    /// ```rust
    /// pub fn to_str(&self) -> String {
    ///     self.floating.to_string()
    /// }
    /// ```
    pub fn to_str(&self) -> String {
        self.floating.to_string()
    }

    /// # Convert to an CString
    /// 
    /// ```rust
    /// pub fn to_cstring(&self) -> CString {
    ///     let _nstring = self.to_str();
    ///     let _cstr = CString::new(_nstring).unwrap();
    ///     _cstr
    /// }
    /// ```
    pub fn to_cstring(&self) -> CString {
        let _nstring = self.to_str();
        let _cstr = CString::new(_nstring).unwrap();
        _cstr
    }

    /// # Convert from String
    /// 
    /// ```rust
    /// pub fn from_string(s: &str) -> Result<Self, <FloatAlias as FromStr>::Err> {
    ///     let floating = s.parse::<FloatAlias>()?;
    ///     Ok(Float {
    ///         floating,
    ///         size: 0.0,
    ///         original: T1::default(),
    ///         type_id: String::new(),
    ///     })
    /// }
    /// ```
    pub fn from_string(s: &str) -> Result<Self, <FloatAlias as FromStr>::Err> {
        let floating = s.parse::<FloatAlias>()?;
        Ok(Float {
            floating,
            size: 0.0,
            original: T1::default(),
            type_id: String::new(),
        })
    }
}

impl<T1: 'static + Default + Display + Eq + Hash + Clone + Send + Sync> Display for Float<T1> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.floating)
    }
}