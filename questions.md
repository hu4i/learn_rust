# Qusestions

## Ownership

* In file [first.rs](./src/entirely_too_many_lists/first.rs):

    ```rust

        fn drop(&mut self) {
            let mut cur_link = mem::replace(&mut self.head, Link::Nil);
            while let Link::More(boxed_node) = cur_link { // why "mut" is needed here? Doesn't boxed_node have the onwnership of cur_link's node?
                cur_link = mem::replace(&mut boxed_node.next, Link::Nil);
            }
        }

        fn drop(&mut self) {
            let mut cur_link = mem::replace(&mut self.head, Link::Nil);
            while let Link::More(boxed_node) = cur_link {
                cur_link = boxed_node.next; // Boxed_node have the onwnership of cur_link's node.
            }

        }
    ```
