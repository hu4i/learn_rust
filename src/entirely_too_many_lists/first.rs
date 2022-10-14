// impl List {

//     pub fn new() -> Self {
//         List { head: Link::Empty }
//     }

//     pub fn push(&mut self, elem:i32) {
//         let new_node = Box::new(Node {
//             elem: elem,
//             next: mem::replace(&mut self.head, Link::Empty),
//         });

//         self.head = Link::More(new_node);
//     }

//     pub fn pop(&mut self) -> Option<i32> {
//         match mem::replace(&mut self.head, Link::Empty) {
//             Link::Empty => None,
//             Link::More(node) => {
//                 self.head = node.next;
//                 Some(node.elem)
//             },
//         }
//     }
// }

// impl Drop for List {
    
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, Link::Empty);
//         while let Link::More(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
//         }
//     }
// }



use std::mem;

pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Nil,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}


impl List {
    
    pub fn new() -> Self {
        List { head: Link::Nil }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new( Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Nil),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::More(boxed_node) => {
                self.head = boxed_node.next;
                Some(boxed_node.elem)
            }
        }
    }

}

impl Drop for List {
    // fn drop(&mut self) {
    //     let mut cur_link = mem::replace(&mut self.head, Link::Nil);
    //     while let Link::More(boxed_node) = cur_link { // why "mut" is needed here? Doesn't boxed_node have the onwnership of cur_link's node?
    //         cur_link = mem::replace(&mut boxed_node.next, Link::Nil);
    //     }
    // }

    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Nil);
        while let Link::More(boxed_node) = cur_link {
            cur_link = boxed_node.next; // Boxed_node have the onwnership of cur_link's node.
        }

    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
       
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

