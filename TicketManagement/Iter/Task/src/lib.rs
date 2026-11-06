use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Додайте метод `iter`, який повертає ітератор по елементах `&Ticket`.
//
// Підказка: як і в попередній вправі, потрібно делегувати ітерацію
//   полю `Vec<Ticket>` у `TicketStore`. Зазирніть у документацію стандартної бібліотеки
//   для `Vec`, щоб знайти правильний тип, який слід повертати з `iter`.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
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

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}
