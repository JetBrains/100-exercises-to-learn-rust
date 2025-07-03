// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

use std::cmp::max;

/// Return the minimum of two values.
pub fn min<T: Ord>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}

fn main() {
    assert_eq!(min(17, 15), 15);
}
