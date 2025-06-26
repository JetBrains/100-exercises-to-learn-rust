// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    std::thread::scope(|s| {
        let left = s.spawn(|| left.iter().sum::<i32>());
        let right = s.spawn(|| right.iter().sum::<i32>());
        left.join().unwrap() + right.join().unwrap()
    })
}
