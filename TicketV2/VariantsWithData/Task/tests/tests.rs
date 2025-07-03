#[cfg(test)]
mod tests {
    use ::common::{valid_description, valid_title};
    use task_variants_with_data::*;

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_todo() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        ticket.assigned_to();
    }

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_done() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::Done);
        ticket.assigned_to();
    }

    #[test]
    fn test_in_progress() {
        let ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        assert_eq!(ticket.assigned_to(), "Alice");
    }
}
