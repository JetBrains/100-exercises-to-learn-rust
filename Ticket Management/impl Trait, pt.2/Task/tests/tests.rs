#[cfg(test)]
mod tests {
    use task_impl_trait_pt2::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};
    use ticket_fields::{TicketDescription, TicketTitle};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // This won't compile if `add_ticket` uses `impl Trait` syntax in argument position.
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
