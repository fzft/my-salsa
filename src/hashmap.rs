use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::linklist;

const DEFAULT_MAX_SIZE: u64 = 256;

pub struct HashMap<T, V> {
    curr_size: usize,
    arr: [Option<linklist::List<T, V>>; DEFAULT_MAX_SIZE as usize],
}


impl<T: Clone + Hash + Eq + PartialEq, V> HashMap<T, V> {

    const INIT: Option<linklist::List<T, V>> = None;

    pub fn new() -> Self {
        Self {
            curr_size: 0,
            arr: [Self::INIT; DEFAULT_MAX_SIZE as usize]
        }
    }

    pub fn put(&mut self, key: T, val: V) -> Option<V> {
        let hash_val = hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        match self.arr[position as usize] {
            Some(ref mut list) => {
                // hash collisist
                Self::update_or_link_new_val(key, val, list, &mut self.curr_size)
            }
            None => {
                self.insert_new_value(key, val, position as usize);
                None
            }
        }
    }

    fn insert_new_value(&mut self, key: T, val: V, pos: usize) {
        let mut list = linklist::List::new();
        list.push(key, val);
        self.arr[pos] = Some(list);
        self.curr_size += 1;
    }

    fn update_or_link_new_val(key: T, val: V, list: &mut linklist::List<T, V>, cur_size: &mut usize) -> Option<V> where T: Eq {
        // traverse the link list until either find value. (update),
        // or stick a new value on the end
        let key_clone = key.clone();
        match list.find(key) {
            Some((_, v)) => {
                let old_value = std::mem::replace(v, val);
                Some(old_value)
            }
            None => {
                list.push(key_clone, val);
                *cur_size += 1;
                None
            }
        }
    }

    pub fn get(&mut self, key: T) -> Option<&V> {
        let hash_val = hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        match self.arr[position as usize] {
            Some(ref mut list) => Self::check_list_for_key(key, list),
            None => None,
        }
    }



    fn check_list_for_key(key: T, list: &mut linklist::List<T, V>) -> Option<&V> {
        match list.find(key) {
            Some((_, v)) => {
                Some(v)
            }
            None => None
        }
    }

    pub fn remove(&mut self, key: T) -> Option<V> {
        let hash_val: u64 = hash_key(key.clone());
        let position: u64 = hash_val % DEFAULT_MAX_SIZE;

        match self.arr[position as usize] {
            Some(ref mut list) => Self::check_item_in_list_and_remove(key, list, &mut self.curr_size),
            None => None,
        }
    }

    fn check_item_in_list_and_remove(key: T, list: &mut linklist::List<T, V>, curr_size: &mut usize) -> Option<V> {

    }

    pub fn clear(&mut self) {
        todo!()
    }
}

fn hash_key<T: Hash>(key: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_val = hasher.finish();
    hash_val
}


fn simple_hash(string: &str) -> u32 {
    let mut total = 0;
    for c in string.chars() {
        total += c as u32;
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_get_item() {
        let key = "hello".to_string();
        let value: i32 = 1;

        let mut my_hash = HashMap::new();
        my_hash.put(key.clone(), value);

        let result = my_hash.get(key).unwrap();

        assert_eq!(result, &value);

    }
}