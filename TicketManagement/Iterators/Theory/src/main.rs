fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    // Turn on inlay hints to see the type at each step of the chain.
    let evens_squared = numbers.iter().filter(|n| *n % 2 == 0).map(|n| n * n);
    let collected: Vec<i32> = evens_squared.collect();
    println!("{collected:?}");
}
