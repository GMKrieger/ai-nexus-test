use std::collections::HashMap;
use std::sync::Mutex;

pub struct LRUCache<K, V> {
    capacity: usize,
    cache: Mutex<HashMap<K, V>>,
}

impl<K, V>
where
    K: Eq + std::hash::Hash + Copy,
    V: Copy,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn put(&self, key: K, value: V) {
        let mut cache = self.cache.lock().unwrap();
        if cache.len() >= self.capacity {
            // Remove the least recently used item
            if let Some(oldest_key) = cache.keys().next().copied() {
                cache.remove(&oldest_key);
            }
        }
        cache.insert(key, value);
    }

    pub fn get(&self, key: K) -> Option<V> {
        let cache = self.cache.lock().unwrap();
        cache.get(&key).copied()
    }
}
