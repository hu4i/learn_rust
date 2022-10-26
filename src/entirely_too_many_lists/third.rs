//! About a persistent immutable singly-linked list with `Rc` and relations between `Rc` and `Arc`. 
//! [Read more](https://rust-unofficial.github.io/too-many-lists/third.html)

use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node{
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List <T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref()}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| { self.next = node.next.as_deref() ; &node.elem })
    }
}

// Drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            match Rc::try_unwrap(node) {
                Ok(mut real_node) => { head = real_node.next.take() }
                _ => { return; }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}