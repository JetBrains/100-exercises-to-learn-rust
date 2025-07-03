#[cfg(test)]
mod tests {
    use task_destructors::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
