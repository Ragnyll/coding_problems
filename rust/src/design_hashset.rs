/// a hash set implemented as a vector of vectors using chaining
#[derive(Debug)]
struct MyHashSet {
    buckets: Vec<Vec<i32>>,
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
impl MyHashSet {
    /// Initializes an empty HashSet
    fn new() -> Self {
        let large_prime: i32 = 997;
        MyHashSet {
            buckets: vec![vec![]; large_prime as usize],
            hash_value: large_prime,
        }
    }


    /// add a number to the hash table
    fn add(&mut self, key: i32) {
        if !self.contains(key) {
            self.buckets[hash_func(self.hash_value, key)].push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let hashed_val = hash_func(self.hash_value, key);
        let index_of_key = self.contains_index(hashed_val, key);
        if index_of_key.is_some() {
            self.buckets[hash_func(self.hash_value, key)].remove(index_of_key.unwrap());
        }
    }

    /// check if the vec at the given bucket contains the key
    fn contains(&self, key: i32) -> bool {
        self.buckets[hash_func(self.hash_value, key)].contains(&key)
    }

    /// checks if the vector contains a value and returns the index it first occurs at if it does
    fn contains_index(&self, hashed_val: usize, key: i32) -> Option<usize>{
        let bucket = &self.buckets[hashed_val];
        for i in 0..bucket.len() {
            if bucket[i] == key {
                return Some(i)
            }
        }

        None
    }
}
