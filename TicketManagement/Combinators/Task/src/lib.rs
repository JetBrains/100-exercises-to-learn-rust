// TODO: Реалізуйте метод `to_dos`. Він має повертати `Vec` посилань на ті тікети
//  у `TicketStore`, статус яких — `Status::ToDo`.
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
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
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn to_dos(&self) -> Vec<&Ticket> {
        self.tickets
            .iter()
            .filter(|ticket| ticket.status == Status::ToDo)
            .collect()
    }
}
