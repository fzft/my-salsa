use std::cell::RefCell;
use std::iter::Iterator;
use std::net::IpAddr::V4;
use std::rc::{Rc, Weak};

type Link<K, T> = Option<Rc<RefCell<Node<K, T>>>>;
type WeakLink<K, T> = Option<Weak<RefCell<Node<K, T>>>>;

#[derive(Debug)]
pub struct Node<K, T> {
    pub(crate) value: T,
    pub(crate) key: K,
    pub(crate) prev: WeakLink<K, T>,
    pub(crate) next: Link<K, T>,
}

impl<K, T> Node<K, T> {
    fn new(key: K, value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            key,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<K: Eq, T> {
    pub(crate) head: Link<K, T>,
    pub(crate) tail: WeakLink<K, T>,
}

impl<K: Eq, T> DoublyLinkedList<K, T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, key: K, value: T) {
        let new_node = Node::new(key, value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.upgrade().unwrap().borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_node.clone());
            }
        };
        self.tail = Some(Rc::downgrade(&new_node));
    }

    pub fn remove(&mut self, target: &Rc<RefCell<Node<K, T>>>) {
        let target_borrowed = target.borrow_mut();
        if let Some(prev) = target_borrowed.prev.as_ref().and_then(|w| w.upgrade()) {
            prev.borrow_mut().next = target_borrowed.next.clone();
        } else {
            self.head = target_borrowed.next.clone();
        }

        if let Some(next) = target_borrowed.next.as_ref() {
            next.borrow_mut().prev = target_borrowed.prev.clone();
        } else {
            self.tail = target_borrowed.prev.clone();
        }
    }

    pub fn find(&self, key: K) -> Option<Rc<RefCell<Node<K, T>>>> {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().key == key {
                return Some(node.clone());
            }
            current = node.borrow().next.clone();
        }
        None
    }

    pub fn iter(&self) -> DoublyLinkedListRefIterator<K, T> {
        DoublyLinkedListRefIterator {
            current: self.head.clone(),
        }
    }
}

pub struct DoublyLinkedListIterator<K: Eq, T> {
    current: Link<K, T>,
}

impl<K: Eq, T> Iterator for DoublyLinkedListIterator<K, T> {
    type Item = Rc<RefCell<Node<K, T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.clone() {
            self.current = current.borrow().next.clone();
            Some(current)
        } else {
            None
        }
    }
}

impl<K: Eq, T> IntoIterator for DoublyLinkedList<K, T> {
    type Item = Rc<RefCell<Node<K, T>>>;
    type IntoIter = DoublyLinkedListIterator<K, T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListIterator {
            current: self.head,
        }
    }
}


pub struct DoublyLinkedListRefIterator<K: Eq, T> {
    current: Link<K, T>,
}

impl<K: Eq, T> Iterator for DoublyLinkedListRefIterator<K, T> {
    type Item = Rc<RefCell<Node<K, T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.clone() {
            self.current = current.borrow().next.clone();
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_find() {
        let mut list: DoublyLinkedList<String, i32> = DoublyLinkedList::new();

        list.push_back("Apple".to_string(), 100);
        list.push_back("Banana".to_string(), 200);
        list.push_back("Cherry".to_string(), 300);

        let apple = list.find("Apple".to_string());
        assert!(apple.is_some());
        assert_eq!(apple.unwrap().borrow().value, 100);

        let banana = list.find("Banana".to_string());
        assert!(banana.is_some());
        assert_eq!(banana.unwrap().borrow().value, 200);

        let cherry = list.find("Cherry".to_string());
        assert!(cherry.is_some());
        assert_eq!(cherry.unwrap().borrow().value, 300);
    }

    #[test]
    fn test_remove() {
        let mut list: DoublyLinkedList<String, i32> = DoublyLinkedList::new();

        list.push_back("Apple".to_string(), 100);
        list.push_back("Banana".to_string(), 200);
        list.push_back("Cherry".to_string(), 300);

        let banana = list.find("Banana".to_string());
        assert!(banana.is_some());
        list.remove(&banana.unwrap());

        assert!(list.find("Banana".to_string()).is_none());
    }

    #[test]
    fn test_iter() {
        let mut list: DoublyLinkedList<String, i32> = DoublyLinkedList::new();

        list.push_back("Apple".to_string(), 100);
        list.push_back("Banana".to_string(), 200);
        list.push_back("Cherry".to_string(), 300);

        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().borrow().key, "Apple");
        assert_eq!(iter.next().unwrap().borrow().key, "Banana");
        assert_eq!(iter.next().unwrap().borrow().key, "Cherry");
        assert!(iter.next().is_none());
    }
}
