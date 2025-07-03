#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use task_enums::*;

    #[test]
    fn test_partial_eq() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket::new(title.clone(), description.clone(), Status::ToDo);
        let ticket2 = Ticket::new(title.clone(), description.clone(), Status::ToDo);
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = valid_title();
        let status = Status::ToDo;
        let ticket1 = Ticket::new(title.clone(), "description".to_string(), status);
        let ticket2 = Ticket::new(title.clone(), "description2".to_string(), status);
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let description = valid_description();
        let status = Status::InProgress;
        let ticket1 = Ticket::new("title".to_string(), description.clone(), status);
        let ticket2 = Ticket::new("title2".to_string(), description.clone(), status);
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket::new(title.clone(), description.clone(), Status::InProgress);
        let ticket2 = Ticket::new(title.clone(), description.clone(), Status::Done);
        assert_ne!(ticket1, ticket2);
    }
}
