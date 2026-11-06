// TODO: переробіть метод `Ticket::new` так, щоб замість паніки він повертав `Result`.
//   Використайте `String` як тип помилки.

#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".into());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".into());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".into());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".into());
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}
