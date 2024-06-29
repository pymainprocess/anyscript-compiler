use std::ffi::CString;
use crate::alias::*;
use crate::types::prelude::*;
use std::hash::Hash;
use std::fmt::Display;
use std::any::TypeId;

pub struct Int<T1> {
    pub integral: IntAlias,
    pub size: IntAlias,
    pub original: T1,
    pub type_id: String,
}

impl<T1: 'static + Default + Display + Eq + Hash + Clone + Send + Sync> Default for Int<T1> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T1: 'static + Default + Display + Eq + Hash + Clone + Send + Sync> Int<T1> {
    ///
    /// # Create a new instance of the class
    ///
    /// ```rust
    /// pub fn new() -> Self {
    ///     let n = T1::default();
    ///     Int {
    ///         integral: 0,
    ///         size: 0,
    ///         original: n,
    ///         type_id: String::new(),
    ///     }
    /// }
    /// ```
    /// ---
    pub fn new() -> Self {
        let n = T1::default();
        Int {
            integral: 0,
            size: 0,
            original: n,
            type_id: String::new(),
        }
    }
    ///
    /// # Reset the class
    ///
    /// ```rust
    /// pub fn reset(&mut self) {
    ///     self.integral = 0;
    ///     self.size = 0;
    ///     self.type_id = String::new();
    ///     self.original = T1::default();
    /// }
    /// ```
    /// ---
    pub fn reset(&mut self) {
        self.integral = 0;
        self.size = 0;
        self.original = T1::default();
        self.type_id = String::new();
    }
    ///
    /// # Check Valid
    ///
    /// ```rust
    ///
    /// ```
    /// ---
    pub fn validate(&self) -> (bool, String) {
        if TypeId::of::<T1>() == TypeId::of::<IntAlias>() {
            return (true, "int".to_string());
        } else if TypeId::of::<T1>() == TypeId::of::<FloatAlias>() {
            return (true, "float".to_string());
        } else if TypeId::of::<T1>() == TypeId::of::<UIntAlias>() {
            return (true, "uint".to_string());
        } else if TypeId::of::<T1>() == TypeId::of::<&str>() || TypeId::of::<T1>() == TypeId::of::<String>() {
            let value = self.original.to_string();
            if value.parse::<IntAlias>().is_ok() {
                return (true, "int".to_string());
            } else if value.parse::<FloatAlias>().is_ok() {
                return (true, "float".to_string());
            } else if value.parse::<UIntAlias>().is_ok() {
                return (true, "uint".to_string());
            } else {
                return (false, "invalid string".to_string());
            }
        } else if TypeId::of::<T1>() == TypeId::of::<CString>() {
            let c_str = self.original.to_string();
            if c_str.parse::<IntAlias>().is_ok() {
                return (true, "int".to_string());
            } else if c_str.parse::<FloatAlias>().is_ok() {
                return (true, "float".to_string());
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
}