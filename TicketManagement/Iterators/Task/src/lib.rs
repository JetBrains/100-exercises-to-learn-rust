use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
//   You want to *delegate* the iteration to the `Vec<Ticket>` field in `TicketStore`.
//   Look at the standard library documentation for `Vec` to find the right type
//   to return from `into_iter`.
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

impl Ticket {
    pub fn new(title: TicketTitle, description: TicketDescription, status: Status) -> Self {
        Self {
            title,
            description,
            status,
        }
    }
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

    pub fn tickets(&self) -> &Vec<Ticket> {
        &self.tickets
    }
}

impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}
