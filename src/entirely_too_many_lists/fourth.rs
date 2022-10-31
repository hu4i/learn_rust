//! Build a bad (not correct or useful) but safe doubly-linked deque with `RefCell` and how to take `T` out from `Rc<RefCell<T>>`. 
//! [Read more](https://rust-unofficial.github.io/too-many-lists/fourth.html)  
//! Note: 
//! * Every node of a double-linked list has two reference count, we need to implement our own drop.
//! * You can't and shouldn't get `&T` from `Rc<RefCell<T>>`, because `&T` doesn't increase the reference count of `Rc`.

use std::{rc::Rc, cell::{RefCell, Ref, RefMut}};

/// A doubly-linked deque.
/// 
/// # Example
/// 
/// ```
/// use learn_rust::entirely_too_many_lists::fourth::List;
/// 
/// let mut list = List::new();
/// 
/// list.push_front(3);
/// list.push_front(4); 
/// 
/// assert_eq!(Some(4), list.pop_front());
/// assert_eq!(Some(3), list.pop_front());
/// assert_eq!(None, list.pop_front());
/// ```
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

/// Wrapper type of iterior node.`
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {

    /// Creates a doubly-linked deque.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    /// 
    /// let mut list = List::new();
    /// 
    /// list.push_front(3);
    /// 
    /// assert_eq!(list.pop_front(), Some(3));
    /// ```
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    /// Appends an element to the head of the list.
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    /// 
    /// let mut list = List::new();
    /// 
    /// list.push_front(3);
    /// 
    /// assert_eq!(Some(4), list.pop_front());
    /// ```
    /// 
    pub fn push_front(&mut self, elem: T) {
        let new_head_node = Node::new(elem);
        
        if let Some(old_head_node) = self.head.take() {
            old_head_node.borrow_mut().prev = Some(new_head_node.clone());
            new_head_node.borrow_mut().next = Some(old_head_node);
            self.head = Some(new_head_node.clone());
        } else {
            self.tail = Some(new_head_node.clone());
            self.head = Some(new_head_node.clone());
        };

    }

    /// Pops the element at the head of the list.
    /// `None` will be returned if the list is empty.
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    /// 
    /// let mut list = List::new();
    /// 
    /// list.push_front(3);
    /// list.push_front(4); 
    /// 
    /// assert_eq!(Some(4), list.pop_front());
    /// assert_eq!(Some(3), list.pop_front());
    /// assert_eq!(None, list.pop_front());
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {

        self.head.take().map(|old_head_node| {
            match old_head_node.borrow_mut().next.take() {
                Some(new_head_node) => {
                    new_head_node.borrow_mut().prev = None;
                    self.head = Some(new_head_node);
                    
                }
                None => { self.tail.take(); }
            };
            // we need to take RefCell from Rc (by try_unwrap) and then take T from refcell (by into_inner)
            Rc::try_unwrap(old_head_node).ok().unwrap().into_inner().elem  
        })

    }

    /// Return a reference of the element at the head of the list.
    /// `None` will be returned if the list is empty.
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    ///
    /// let mut lst = List::<i32>::new();
    ///
    /// lst.push_front(3);
    ///
    /// assert_eq!(3, *lst.peek_front().unwrap());
    /// ```
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| { &node.elem })
        })
    }

    /// Appends an element to the tail of the list
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    /// 
    /// let mut lst = List::<i32>::new();
    /// 
    /// lst.push_back(3);
    /// lst.push_front(4);
    /// assert_eq!(Some(4), lst.pop_front());
    /// assert_eq!(Some(3), lst.pop_front());
    /// assert_eq!(None, lst.pop_front());
    /// ```
    pub fn push_back(&mut self, elem: T) {
        let new_tail_node = Node::new(elem);
        
        if let Some(old_tail_node) = self.tail.take() {
            old_tail_node.borrow_mut().next = Some(new_tail_node.clone());
            new_tail_node.borrow_mut().prev = Some(old_tail_node);
            self.tail = Some(new_tail_node.clone());
        } else {
            self.tail = Some(new_tail_node.clone());
            self.head = Some(new_tail_node.clone());
        };
    }


    /// Pops an element at the tail of the list
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    /// 
    /// let mut lst = List::<i32>::new();
    /// 
    /// lst.push_back(3);
    /// lst.push_front(4);
    /// assert_eq!(Some(3), lst.pop_back());
    /// assert_eq!(Some(4), lst.pop_back());
    /// assert_eq!(None, lst.pop_front());
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail.clone());
                }
                None => { self.head.take(); }
            }
            let elem = Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem;
            elem
        })
    }

    /// Return a reference of the element at the tail of the list.
    /// `None` will be returned if the list is empty.
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    ///
    /// let mut lst = List::<i32>::new();
    ///
    /// lst.push_front(3);
    ///
    /// assert_eq!(3, *lst.peek_back().unwrap());
    /// 
    /// lst.pop_back();
    /// 
    /// assert_eq!(true, lst.peek_back().is_none());
    /// ```
    pub fn peek_back(&mut self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|tail_node| {
            Ref::map(tail_node.borrow(), |node| { &node.elem }) })
    }

    /// Return a mutable reference of the element at the tail of the list.
    /// `None` will be returned if the list is empty.
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    ///
    /// let mut lst = List::<i32>::new();
    ///
    /// lst.push_front(3);
    ///
    /// let r = lst.peek_back_mut();
    /// 
    /// *r.unwrap() = 4;
    /// 
    /// assert_eq!(4, lst.pop_back().unwrap());
    /// ```
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|tail_node| {
            RefMut::map(tail_node.borrow_mut(), |node| { &mut node.elem }) })
    }    

    /// Return a mutable reference of the element at the head of the list.
    /// `None` will be returned if the list is empty.
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    ///
    /// let mut lst = List::<i32>::new();
    ///
    /// lst.push_back(3);
    ///
    /// let r = lst.peek_front_mut();
    /// 
    /// *r.unwrap() = 4;
    /// 
    /// assert_eq!(4, lst.pop_back().unwrap());
    /// ```
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|head_node| {
            RefMut::map(head_node.borrow_mut(), |node| { &mut node.elem }) })
    }    
    

}

// We have to implement drop by ourself.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.head.is_some() {
            self.pop_front();
        }
    }
}


impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

/// An iterator that moves out of a vector.
/// 
/// This `strtuct` is created by the `into_iter` method on [`List`] (provided by the [`IntoIterator`] trait).
/// 
/// # Example
/// 
/// ```
/// use learn_rust::entirely_too_many_lists::fourth::List;
/// 
/// let mut lst = List::new();
///
/// lst.push_front(3);
/// lst.push_front(4);
///  
/// let mut iter = lst.into_iter();
/// 
/// assert_eq!(Some(4), iter.next());
/// assert_eq!(Some(3), iter.next());
/// assert_eq!(true, iter.next().is_none());
/// ```
pub struct IntoIter<T>(List<T>);


impl<T> List<T>{

    /// Return an iterator that returns the onwership of values in the list from head to tail.
    /// Return `None` when nothing left.
    /// 
    /// # Example
    /// 
    /// ```
    /// use learn_rust::entirely_too_many_lists::fourth::List;
    /// 
    /// let mut lst = List::new();
    /// 
    /// lst.push_front(3);
    /// lst.push_front(4);
    /// lst.push_front(5);
    /// 
    /// let mut iter = lst.into_iter();
    /// 
    /// assert_eq!(Some(5), iter.next());
    /// assert_eq!(Some(4), iter.next());
    /// assert_eq!(Some(3), iter.next());
    /// 
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }


}


/// An iterator yieds `&T`
//pub struct Iter<T>(Option<Rc<Node<T>>>);
pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>, Option<Ref<'a,  RefCell<Node<T>> >>);

impl<T> List<T>{

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_ref().map(|head| head.borrow()), None)
    }
    // pub fn iter(& self) -> Iter<T> {
    //     Iter(self.head.as_ref().map(|head| head.clone()))
    // }

}
