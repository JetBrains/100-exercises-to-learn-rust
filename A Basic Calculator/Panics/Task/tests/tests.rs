#[cfg(test)]
mod tests {
    use task_panics::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 With the `#[should_panic]` annotation we can assert that we expect the code
    //    under test to panic. We can also check the panic message by using `expected`.
    //    This is all part of Rust's built-in test framework!
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
