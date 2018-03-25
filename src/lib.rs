#[cfg(test)]
mod tests;

use std::collections::HashMap;

#[derive(Debug)]
pub struct KV {
    hash_map: HashMap<String, String>,
}

impl KV {
    pub fn new() -> KV {
        KV {hash_map: HashMap::new()}
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.hash_map.get(key).map(String::as_str)
    }

    pub fn put(&mut self, key:String, value: String) {
        self.hash_map.insert(key, value);
    }
}
