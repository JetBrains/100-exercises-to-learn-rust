#[cfg(test)]
mod tests {
    use task_sync_trait::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
