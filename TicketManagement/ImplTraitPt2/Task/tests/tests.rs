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
        // Цей код не скомпілюється, якщо `add_ticket` використовує синтаксис `impl Trait` у позиції аргументу.
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
