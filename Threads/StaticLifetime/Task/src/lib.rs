// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let mid = slice.len() / 2;
    let (slice1, slice2) = slice.split_at(mid);

    let handle1 = thread::spawn(move || slice1.iter().sum::<i32>());
    let handle2 = thread::spawn(move || slice2.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}
