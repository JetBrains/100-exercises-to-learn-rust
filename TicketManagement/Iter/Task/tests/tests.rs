#[cfg(test)]
mod tests {
    use task_iter::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket::new(ticket_title(), ticket_description(), Status::ToDo);
        store.add_ticket(ticket);

        let ticket = Ticket::new(ticket_title(), ticket_description(), Status::InProgress);
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
