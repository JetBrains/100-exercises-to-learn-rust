// Перепишіть функцію factorial із використанням циклу `for`.
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}
