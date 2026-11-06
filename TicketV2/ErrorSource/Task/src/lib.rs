pub use crate::status::{ParseStatusError, Status};

// В одній із найперших вправ ми бачили, як оголошувати модулі, але
// ще не бачили, як виносити їх в окремі файли.
// Виправмо це зараз!
//
// У найпростішому випадку, коли винесений модуль складається з одного файлу, достатньо
// створити новий файл з такою самою назвою, як у модуля, і перенести туди вміст модуля.
// Файл модуля слід розмістити в тій самій теці, що й файл, який оголошує модуль.
// У цьому випадку це `src/lib.rs`, тож `status.rs` слід розмістити в теці `src`.
mod status;

// TODO: додайте до `TicketNewError` новий варіант помилки для випадку, коли рядок статусу некоректний.
//   Виклик `source` для помилки цього варіанта має повертати `ParseStatusError`, а не `None`.

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
    #[error("{0}")]
    InvalidStatus(#[from] ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
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

        // TODO: розберіть рядок статусу, щоб отримати значення переліку `Status`.
        let status = Status::try_from(status)?;

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}
