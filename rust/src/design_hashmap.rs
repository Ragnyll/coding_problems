#[derive(Debug)]
struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
    hash_value: i32,
}

/// hashes n using the hash_value
fn hash_func(hash_val: i32, n: i32) -> usize {
    (n % hash_val) as usize
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        let hash_val = 997;
        MyHashMap {
            buckets: vec![vec![]; hash_val],
            hash_value: hash_val as i32,
        }
    }

    /// add the kvp to the hash map
    fn put(&mut self, key: i32, value: i32) {
        let hashed_key = hash_func(self.hash_value, key);
        if let Some(val) = self.contains(hashed_key, key) {
            self.buckets[hashed_key][val].1 = value;
            return;
        }

        self.buckets[hashed_key].push((key, value));
    }

    /// retrieve the value belong to key if it exists. if it does not exist return -1
    fn get(&self, key: i32) -> i32 {
        let hashed_key = hash_func(self.hash_value, key);
        if let Some(val) = self.contains(hashed_key, key) {
            return self.buckets[hashed_key][val].1;
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let hashed_key = hash_func(self.hash_value, key);
        if let Some(val) = self.contains(hashed_key, key) {
            self.buckets[hashed_key].remove(val);
        }
    }

    /// if the key is contained in the bucket returns Some(index of key)
    fn contains(&self, hash_key: usize, key: i32) -> Option<usize> {
        if self.buckets[hash_key].len() > 0 {
            for i in 0..self.buckets[hash_key].len() {
                if self.buckets[hash_key][i].0 == key {
                    return Some(i);
                }
            }
        }
        None
    }
}
