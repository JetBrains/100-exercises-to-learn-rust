pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        result = result.wrapping_mul(i);
    }
    result
}
