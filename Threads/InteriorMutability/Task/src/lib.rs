// TODO: за допомогою `Rc` і `RefCell` реалізуйте `DropTracker<T>` — обгортку над значенням типу `T`,
//  яка збільшує спільний лічильник типу `usize` щоразу, коли обгорнуте значення знищується (drop).

use std::cell::RefCell;
use std::rc::Rc;

pub struct DropTracker<T> {
    value: T,
    counter: Rc<RefCell<usize>>,
}

impl<T> DropTracker<T> {
    pub fn new(value: T, counter: Rc<RefCell<usize>>) -> Self {
        Self { value, counter }
    }
}

impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        *self.counter.borrow_mut() += 1;
    }
}
