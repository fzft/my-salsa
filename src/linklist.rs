pub struct List<T, V> {
    head: Option<Box<Node<T, V>>>,
}


impl<T, V> List<T, V> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn push(&mut self, key: T, val: V) {
        let old_head = self.head.take();
        let new_head = Box::new(Node::new(key, val, old_head));
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<(T, V)> {
        self.head.take().map(|n| {
            self.head = n.next;
            (n.key, n.value)
        })
    }

    pub fn find(&mut self, key: T) -> Option<(&T, &mut V)> where T: Eq {
        let mut current_head = &mut self.head;
        while let Some(current_n) = current_head {
            if current_n.key == key {
                return Some((&current_n.key, &mut current_n.value));
            }
            current_head = &mut current_n.next;
        }
        None
    }

    pub fn remove(&mut self, key: T) -> Option<>
}

impl<T, V> Drop for List<T, V> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut n) = cur {
            cur = n.next.take()
        }
    }
}


impl<T: Copy, V: Copy> Into<Vec<(T, V)>> for List<T, V> {
    fn into(self) -> Vec<(T, V)> {
        let mut current_head = &self.head;
        let mut items = Vec::new();
        while let Some(current_n) = current_head {
            items.push((current_n.key, current_n.value));
            current_head = &current_n.next;
        }
        items
    }
}

pub struct Node<T, V> {
    value: V,
    key: T,
    next: Option<Box<Node<T, V>>>,
}

impl<T, V> Node<T, V> {
    pub fn new(key: T, value: V, next: Option<Box<Node<T, V>>>) -> Self {
        Self {
            key,
            value,
            next,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_build_list() {
        let mut list = List::new();
        list.push(5, "foo");
        list.push(4, "bar");
        list.push(3, "buzz");
        list.push(2, "fizz");
        list.push(1, "bazz");

        assert_eq!(list.head.is_some(), true);
        assert_eq!(list.pop(), Some((1, "bazz")));
        assert_eq!(list.pop(), Some((2, "fizz")));
        assert_eq!(list.pop(), Some((3, "buzz")));
        assert_eq!(list.pop(), Some((4,"bar")));
        assert_eq!(list.pop(), Some((5,"foo")));
        assert_eq!(list.pop(), None)
    }

    #[test]
    fn works_build_list_vec() {
        let mut list = List::new();
        list.push(5, "foo");
        list.push(4, "bar");
        list.push(3, "buzz");
        list.push(2, "fizz");
        list.push(1, "bazz");


        let items: Vec<(i32, &str)> = list.into();
    }

    #[test]
    fn works_build_list_find() {
        let mut list: List<i32, &str> = List::new();
        list.push(5, "foo");
        list.push(4, "bar");
        list.push(3, "buzz");
        list.push(2, "fizz");
        list.push(1, "bazz");

        if let Some((_, value)) = list.find(2) {
            let _ = std::mem::replace(value, "fizz2");
        }

        assert_eq!(list.pop(), Some((1, "bazz")));
        assert_eq!(list.pop(), Some((2, "fizz2")));
    }
}