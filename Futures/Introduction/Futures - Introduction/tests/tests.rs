mod tests {
    use task_futures_introduction::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about futures!");
    }
}
