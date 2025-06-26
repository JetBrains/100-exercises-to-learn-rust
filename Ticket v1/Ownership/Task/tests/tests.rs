#[cfg(test)]
mod tests {
    use task_ownership::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // If you change the signatures as requested, this should compile:
        // we can call these methods one after the other because they borrow `self`
        // rather than taking ownership of it.
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
