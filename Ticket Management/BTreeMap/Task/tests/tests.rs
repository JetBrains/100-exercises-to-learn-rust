#[cfg(test)]
mod tests {
    use task_btree_map::{Status, TicketDraft, TicketId, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let n_tickets = 5;

        for i in 0..n_tickets {
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
        }

        let ids: Vec<TicketId> = (&store).into_iter().map(|t| t.id).collect();
        let sorted_ids = {
            let mut v = ids.clone();
            v.sort();
            v
        };
        assert_eq!(ids, sorted_ids);
    }
}
