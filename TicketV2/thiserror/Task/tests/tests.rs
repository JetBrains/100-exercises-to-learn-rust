#[cfg(test)]
mod tests {
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};
    use task_thiserror::*;

    #[test]
    fn title_cannot_be_empty() {
        let err = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let err = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let err = Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description_cannot_be_too_long() {
        let err = Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Description cannot be longer than 500 bytes"
        );
    }
}
