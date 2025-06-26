pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // rather than overflowing and wrapping around
        result = result.saturating_mul(i);
    }
    result
}
