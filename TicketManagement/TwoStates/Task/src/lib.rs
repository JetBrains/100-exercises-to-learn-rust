// TODO: Оновіть сигнатуру `add_ticket`: метод має приймати на вхід `TicketDraft`
//  і повертати `TicketId`.
//  Кожен тікет повинен мати унікальний id, який генерує `TicketStore`.
//  За потреби можете змінювати поля `TicketStore`.
//
// Також потрібно додати метод `get`, який приймає на вхід `TicketId`
// і повертає `Option<&Ticket>`.

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
