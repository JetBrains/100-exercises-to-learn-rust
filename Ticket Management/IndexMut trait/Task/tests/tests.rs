#[cfg(test)]
mod tests {
    use task_index_mut_trait::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id = store.add_ticket(draft.clone());
        let ticket = &store[id];
        assert_eq!(draft.title, ticket.title);
        assert_eq!(draft.description, ticket.description);
        assert_eq!(ticket.status, Status::ToDo);

        let ticket = &mut store[id];
        ticket.status = Status::InProgress;

        let ticket = &store[id];
        assert_eq!(ticket.status, Status::InProgress);

        let ticket = &mut store[&id];
        ticket.status = Status::Done;

        let ticket = &store[id];
        assert_eq!(ticket.status, Status::Done);
    }
}
