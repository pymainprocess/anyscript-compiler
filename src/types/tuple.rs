//! Tuple type for the lexer
//! 
//! This Class Provides a Tuple Type

use std::fmt::Display;
use std::default::Default;
use std::ffi::CString;

pub struct Tuple<T1: Default + Display + Clone + Send + Sync, T2: Default + Display + Clone + Send + Sync> {
    pub first: T1,
    pub second: T2,
    pub tuple: (T1, T2),
}

impl<T1: Default + Display + Clone + Send + Sync, T2: Default + Display + Clone + Send + Sync> Default for Tuple<T1, T2> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T1: Default + Display + Clone + Send + Sync, T2: Default + Display + Clone + Send + Sync> Tuple<T1, T2> {

    /// Create a new tuple with default values
    pub fn new() -> Self {
        let first = T1::default();
        let second = T2::default();
        Self {
            first: first.clone(),
            second: second.clone(),
            tuple: (first, second),
        }
    }

    /// Set the first value
    pub fn set_first(&mut self, first: T1) {
        self.first = first.clone();
        self.tuple = (first, self.second.clone());
    }

    /// Set the second value
    pub fn set_second(&mut self, second: T2) {
        self.second = second.clone();
        self.tuple = (self.first.clone(), second);
    }

    pub fn get_first(&self) -> T1 {
        self.first.clone()
    }

    pub fn get_second(&self) -> T2 {
        self.second.clone()
    }

    pub fn first_to_str(&self) -> String {
        self.first.to_string()
    }

    pub fn second_to_str(&self) -> String {
        self.second.to_string()
    }

    pub fn first_to_cstr(&self) -> CString {
        CString::new(self.first.to_string()).unwrap()
    }

    pub fn second_to_cstr(&self) -> CString {
        CString::new(self.second.to_string()).unwrap()
    }

}