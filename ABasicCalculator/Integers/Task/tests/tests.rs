#[cfg(test)]
mod tests {

    use task_integers::compute;

    #[test]
    fn case_1() {
        assert_eq!(compute(1, 2), 9);
    }

    #[test]
    fn case_2() {
        assert_eq!(compute(1, 3), 13);
    }
}
