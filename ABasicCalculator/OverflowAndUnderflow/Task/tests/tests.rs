#[cfg(test)]
mod tests {
    use task_overflow_and_underflow::factorial;

    #[test]
    fn twentieth() {
        // 20! дорівнює 2432902008176640000, а це число занадто велике, щоб уміститися в u32
        // Зі стандартним профілем dev це спричинить паніку, коли ви запустите `cargo test`
        // Ми ж хочемо, щоб натомість відбувався перехід по колу (wrap around)
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // Великий числовий літерал, у якому для зручності читання використано підкреслення!
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
