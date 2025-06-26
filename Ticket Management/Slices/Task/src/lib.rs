// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
//  elements in the slice.

pub fn sum(slice: &[u32]) -> u32 {
    slice.iter().sum()
}
