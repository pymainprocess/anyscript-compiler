//! List data type
//!
//! This is a list data type that is a collection of elements that are stored in a
//! sequence. The list is a collection of elements that are stored in a sequence.
//! The list is a collection of elements that are stored in a sequence.
use std::ffi::CString;
use std::fmt::Display;
use std::default::Default;

/// Class to Handle a List
pub struct List<T: Default + Display + Clone + Send + Sync> {
    pub items: Vec<T>,
    pub size: usize,
}

impl<T: Default + Display + Clone + Send + Sync> List<T> {

    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            size: 0,
        }
    }

    pub fn append(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
    }

    pub fn to_string2(&self) -> String {
        let mut string = String::new();
        for i in &self.items {
            string.push_str(&format!("{}", i));
        }
        string
    }

    pub fn to_cstring(&self) -> CString {
        let string = self.to_string();
        
        CString::new(string).unwrap()
    }

    pub fn extend(&mut self, items: List<T>) {
        self.items.extend(items.items);
        self.size += items.size;
    }

    pub fn to_list(&self) -> Vec<T> where T: Copy {
        self.items.to_vec()
    }

    pub fn replace(&mut self, index: usize, item: T) {
        self.items[index] = item;
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
        self.size -= 1;
    }

    pub fn get(&self, index: usize) -> T where T: Copy {
        self.items[index]
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_type(&self) -> String {
        let type_name = std::any::type_name::<T>();
        type_name.to_string()
    }

    pub fn destroy(&mut self) {
        self.items.clear();
        self.size = 0;
    }

    pub fn from(liste: Vec<T>) -> Self {
        let mut list = Self::new();
        for item in liste {
            list.append(item);
        }
        list
    }

    pub fn from_string(string: String) -> Self where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
        let mut list = Self::new();
        for item in string.split(' ') {
            list.append(item.trim().parse::<T>().unwrap());
        }
        list
    }    
    
    pub fn from_cstring(cstring: CString) -> Self where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
        let string = cstring.to_str().unwrap().to_string();
        let mut list = Self::new();
        for item in string.split(' ') {
            list.append(item.trim().parse::<T>().unwrap());
        }
        list
    }
}

// Implementierung von Default für List<T>
impl<T: Default + Display + Clone + Send + Sync> Default for List<T> {
    fn default() -> Self {
        List { items: Vec::new(), size: 0 }
    }
}

// Implementierung von Display für List<T>
impl<T: Display + Default + Clone + Send + Sync> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

// Implementierung von Clone für List<T>
impl<T: Clone + Default + Display + Send + Sync> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            items: self.items.clone(),
            size: self.size,
        }
    }
}