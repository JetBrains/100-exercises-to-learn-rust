#[cfg(test)]
mod tests {
    use task_panics::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 За допомогою анотації `#[should_panic]` можна вказати, що ми очікуємо паніки
    //    від тестованого коду. Також можна перевірити повідомлення паніки за допомогою `expected`.
    //    Усе це — частина вбудованого фреймворку тестування Rust!
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
