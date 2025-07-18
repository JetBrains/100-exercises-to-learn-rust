// TODO: Update `add_ticket`'s signature: it should take a `TicketDraft` as input
//  and return a `TicketId` as output.
//  Each ticket should have a unique id, generated by `TicketStore`.
//  Feel free to modify `TicketStore` fields, if needed.
//
// You also need to add a `get` method that takes as input a `TicketId`
// and returns an `Option<&Ticket>`.

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    counter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.counter;
        self.counter += 1;
        let ticket = Ticket {
            id: TicketId(id),
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
        };
        self.tickets.push(ticket);
        TicketId(id)
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|ticket| ticket.id == id)
    }
}
