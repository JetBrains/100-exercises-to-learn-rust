// TODO: реалізуйте trait `Error` для `TicketNewError` за допомогою `thiserror`.
//   Ми зробили варіанти enum конкретнішими, тож тепер не потрібно зберігати
//   поле `String` у кожному варіанті.
//   Також доведеться додати `thiserror` як залежність у файлі `Cargo.toml`.

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}
