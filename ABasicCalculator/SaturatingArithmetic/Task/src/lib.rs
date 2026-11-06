pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // Використайте saturating multiplication, щоб результат зупинявся на максимальному значенні u32,
        // а не переповнювався й не переходив по колу
        result = result.saturating_mul(i);
    }
    result
}
