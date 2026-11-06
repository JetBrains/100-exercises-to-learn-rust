use crate::data::TicketDraft;
use crate::store::TicketStore;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Вмикає систему, запускаючи серверний потік.
// Повертає екземпляр `Sender`, який потім можуть використовувати
// один або кілька клієнтів для взаємодії із сервером.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: серверний потік **ніколи** не повинен зупинятися.
//  Організуйте цикл: дочекайтеся, поки в каналі з'явиться
//  команда, виконайте її, а потім знову почніть чекати
//  на наступну команду.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Insert(ticket_draft) => {
                store.add_ticket(ticket_draft);
            }
        }
    }
}
