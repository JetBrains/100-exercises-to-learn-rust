// TODO: Змініть сигнатуру `TicketStore::add_ticket` так, щоб у ній використовувався generic type parameter
//  замість синтаксису `impl Trait`.

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

    // Якщо використати `Into<Ticket>` як type parameter для `ticket`, метод зможе приймати будь-який тип,
    // перетворення якого на `Ticket` не може завершитися невдачею.
    // Так методом зручніше користуватися, бо в місці виклику зникає синтаксичний шум `.into()`.
    // Щоправда, через це можуть стати менш зрозумілими повідомлення про помилки компіляції.
    pub fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) {
        self.tickets.push(ticket.into());
    }
}
