use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::linklist;

const DEFAULT_MAX_SIZE: u64 = 256;

pub struct HashMap<T: Eq + Hash, V: Clone> {
    curr_size: usize,
    arr: [Option<linklist::DoublyLinkedList<T, V>>; DEFAULT_MAX_SIZE as usize],
}


impl<T:  Hash + Eq + Clone, V: Clone> HashMap<T, V> {

    const INIT: Option<linklist::DoublyLinkedList<T, V>> = None;

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

    pub fn get(&mut self, key: T) -> Option<V> {
        let hash_val: u64 = hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        match self.arr[position as usize] {
            Some(ref mut list) => Self::check_list_for_key(key, list),
            None => None,
        }
    }

    fn insert_new_value(&mut self, key: T, val: V, pos: usize) {
        let mut list = linklist::DoublyLinkedList::new();
        list.push_back(key, val);
        self.arr[pos] = Some(list);
        self.curr_size += 1;
    }

    fn update_or_link_new_val(key: T, val: V, list: &mut linklist::DoublyLinkedList<T, V>, cur_size: &mut usize) -> Option<V> where T: Eq {
        // traverse the link list until either find value. (update),
        // or stick a new value on the end
        let key_clone = key.clone();
        match list.find(key_clone) {
            Some(node) => {
                let mut node_borrowed = node.borrow_mut();
                let old_val = node_borrowed.value.clone();
                node_borrowed.value = val;
                Some(old_val)
            },
            None => {
                list.push_back(key, val);
                *cur_size += 1;
                None
            }
        }

    }

    fn check_list_for_key(key: T, list: &mut linklist::DoublyLinkedList<T, V>) -> Option<V>
    {
        match list.find(key) {
            Some(node) => {
                let node_borrowed = node.borrow();
                Some(node_borrowed.value.clone())
            }
            None => {
                None
            }
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

    fn check_item_in_list_and_remove(key: T, list: &mut linklist::DoublyLinkedList<T, V>, curr_size: &mut usize) -> Option<V> {
        match list.find(key) {
            Some(node) => {
                let old_val = node.borrow_mut().value.clone();
                list.remove(&node);
                *curr_size -= 1;
                Some(old_val)
            },
            None => {
                None
            }
        }
    }

    pub fn clear(&mut self) {
        // overwrite the array to yeet everything
        self.curr_size = 0;
        self.arr = [Self::INIT; DEFAULT_MAX_SIZE as usize];
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
    fn test_hash_map() {
        let mut map = HashMap::new();

        assert_eq!(map.get(1), None);

        map.put(1, "one".to_string());

        assert_eq!(map.get(1), Some("one".to_string()));

        map.put(1, "uno".to_string());

        assert_eq!(map.get(1), Some("uno".to_string()));

        assert_eq!(map.remove(1), Some("uno".to_string()));

        assert_eq!(map.get(1), None);
    }
}
