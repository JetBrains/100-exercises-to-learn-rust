use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Почнімо вибудовувати каркас нашого сховища тікетів!
//  Перше завдання: реалізуйте `IntoIterator` для `TicketStore`, щоб усі тікети, які в ньому є,
//  можна було перебирати за допомогою циклу `for`.
//
// Підказка: у цьому випадку вам не потрібно реалізовувати трейт `Iterator`.
//   Вам потрібно *делегувати* ітерацію полю `Vec<Ticket>` у `TicketStore`.
//   Зазирніть у документацію стандартної бібліотеки для `Vec`, щоб знайти правильний тип,
//   який слід повертати з `into_iter`.
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
