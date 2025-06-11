use lru_cache::LRUCache;
use std::thread;

fn main() {
    let cache = LRUCache::<i32, i32>::new(3);

    // Insert some values
    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);

    println!("Value for key 1: {:?}", cache.get(1)); // Some(10)
    println!("Value for key 2: {:?}", cache.get(2)); // Some(20)
    println!("Value for key 3: {:?}", cache.get(3)); // Some(30)

    // Insert another value, which should evict the least recently used (1)
    cache.put(4, 40);

    println!("Value for key 1: {:?}", cache.get(1)); // None (evicted)
    println!("Value for key 4: {:?}", cache.get(4)); // Some(40)

    // Demonstrate concurrent access
    let cache = std::sync::Arc::new(cache);
    let mut handles = vec![];

    for i in 0..5 {
        let cache = cache.clone();
        let handle = thread::spawn(move || {
            cache.put(i, i * 10);
            println!("Thread {} - Value for key {}: {:?}", i, i, cache.get(i));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
