// TODO: Визначте функцію з назвою `sum`, яка приймає reference на slice `u32` і повертає суму всіх
//  елементів slice.

pub fn sum(slice: &[u32]) -> u32 {
    slice.iter().sum()
}
