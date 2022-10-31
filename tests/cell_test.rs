
#[test]
fn cell_test() {
    use std::cell::RefCell;

    let d = RefCell::new(3);
    let e = d.clone();
    *e.borrow_mut() = 4;
    println!("Value in d is {}", d.borrow());
    println!("Value in e is {}", e.borrow());
}

#[test]
fn refcell_map_test() {
    use std::cell::{RefCell, Ref};

    let c = RefCell::new((5, 'b'));
    let b1: Ref<(u32, char)> = c.borrow();
    let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
    assert_eq!(*b2, 5);
}
