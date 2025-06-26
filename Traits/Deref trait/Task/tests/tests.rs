#[cfg(test)]
mod tests {
    use task_deref_trait::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket::new(
            "   A title ".to_string(),
            " A description   ".to_string(),
            "To-Do".to_string(),
        );
        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
