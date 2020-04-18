use std::collections::BTreeMap;
use crate::value::Value;

pub struct Map<K, V> {
    map: BTreeMap<K, V>
}

impl Map<String, Value> {

    pub fn new() -> Self {
        Map { map: BTreeMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn length(&self) -> usize {
        self.map.len()
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.map.get(key)?.as_bool()
    }
}