//! # Dict
//!
//! This module contains the definition for the Dict type.
use std::ffi::CString;
use std::fmt::Display;
use std::collections::HashMap;
use std::hash::Hash;
use crate::types::prelude::*;

/// Class to Handle a Dictionary
pub struct Dict<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
    pub size: usize,
    pub dict: HashMap<K, V>,
}

impl<K: Default + Display + Eq + Hash + Clone + Send + Sync, V: Default + Display + Clone + Send + Sync> Default for Dict<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Default + Display + Eq + Hash + Clone + Send + Sync, V: Default + Display + Clone + Send + Sync> Dict<K, V> {

    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            size: 0,
            dict: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    pub fn to_string2(&self) -> String {
        let mut string = String::new();
        for (key, value) in self.keys.iter().zip(self.values.iter()) {
            string.push_str(&format!("{}: {}\n", key, value));
        }
        string
    }

    pub fn to_cstring(&self) -> CString {
        let string = self.to_string2();
        
        CString::new(string).unwrap()
    }

    pub fn to_dict(&mut self) -> HashMap<K, V> {
        for (key, value) in self.keys.iter().zip(self.values.iter()) {
            self.dict.insert(key.clone(), value.clone());
        }
        self.dict.clone()
    }

    pub fn get_type(&self) -> String {
        let k_type = std::any::type_name::<K>();
        let v_type = std::any::type_name::<V>();
        format!("{}, {}", k_type, v_type)
    }

    fn update_size(&mut self) {
        let x1 = self.keys.len();
        let x2 = self.values.len();
        self.size = x1 + x2;
    }

    pub fn get_size(&mut self) -> usize {
        self.update_size();
        self.size
    }

    pub fn replace(&mut self, key: K, new: V) {
        let index = self.keys.iter().position(|x| x == &key);
        if let Some(index) = index {
            self.values[index] = new;
        }
    }

    pub fn remove(&mut self, key: K) {
        let index = self.keys.iter().position(|x| x == &key);
        if let Some(index) = index {
            self.keys.remove(index);
            self.values.remove(index);
        }
    }

    pub fn destroy(&mut self) {
        self.keys.clear();
        self.values.clear();
        self.dict.clear();
    }

    pub fn get_key_items(&self) -> List<K> {
        let mut items = List::new();
        for k in &self.keys {
            items.append(k.clone());
        }
        items
    }

    pub fn get_value_items(&self) -> List<V> {
        let mut items = List::new();
        for v in &self.values {
            items.append(v.clone());
        }
        items
    }

    pub fn get(&self, key: K) -> V {
        self.dict[&key].clone()
    }

    pub fn to_list(&self) -> Tuple<List<K>, List<V>> {
        let mut keys = List::new();
        let mut values = List::new();
        for (key, value) in self.keys.iter().zip(self.values.iter()) {
            keys.append(key.clone());
            values.append(value.clone());
        }
        Tuple {
            first: keys.clone(),
            second: values.clone(),
            tuple: (keys, values),
        }
    }

    pub fn from(map: HashMap<K, V>) -> Self {
        let mut dict = Self::new();
        for (key, value) in map {
            dict.insert(key, value);
        }
        dict
    }
}