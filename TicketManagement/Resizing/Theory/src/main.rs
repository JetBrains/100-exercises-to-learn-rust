fn main() {
    let mut v = Vec::new();
    for i in 0..10 {
        v.push(i);
        // Set a breakpoint here and watch len and capacity grow.
        println!("len {}, cap {}", v.len(), v.capacity());
    }
}
