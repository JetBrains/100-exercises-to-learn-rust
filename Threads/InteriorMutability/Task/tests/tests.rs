#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use task_interior_mutability::*;

    #[test]
    fn it_works() {
        let counter = Rc::new(RefCell::new(0));
        let _ = DropTracker::new((), Rc::clone(&counter));
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn multiple() {
        let counter = Rc::new(RefCell::new(0));

        {
            let a = DropTracker::new(5, Rc::clone(&counter));
            let b = DropTracker::new(6, Rc::clone(&counter));
        }

        assert_eq!(*counter.borrow(), 2);
    }
}
