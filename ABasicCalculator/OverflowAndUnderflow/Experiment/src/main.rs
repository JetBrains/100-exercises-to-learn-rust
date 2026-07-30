// An experiment: run this with different Cargo profiles and watch the behaviour
// change. There's nothing to submit — just observe.
fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn main() {
    // 20! = 2,432,902,008,176,640,000 — far bigger than u32::MAX (4,294,967,295).
    println!("20! (as u32) = {}", factorial(20));
}
