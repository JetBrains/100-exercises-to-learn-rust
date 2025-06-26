#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::any::{Any, TypeId};
    use task_string_slices::*;

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
        // Some dark magic to verify that you used the expected return types
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}
