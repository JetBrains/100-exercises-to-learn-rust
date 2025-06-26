// TODO: Define a function named `squared` that raises all `i32`s within a slice to the power of 2.
//  The slice should be modified in place.

pub fn squared(slice: &mut [i32]) {
    for i in slice.iter_mut() {
        *i *= *i;
    }
}
