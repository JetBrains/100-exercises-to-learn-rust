use std::cell::RefCell;

fn main() {
    let counter = RefCell::new(0);
    *counter.borrow_mut() += 1;
    // Put the caret on `RefCell` (or `borrow_mut`) and Go to Definition.
    println!("{}", counter.borrow());
}
