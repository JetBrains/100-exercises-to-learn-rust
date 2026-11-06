// TODO: використайте два варіанти: один для помилки заголовка, інший — для помилки опису.
//   Кожен варіант має містити рядок із поясненням, що саме пішло не так.
//   Реалізацію `Ticket::new` теж доведеться оновити.
#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

// TODO: якщо заголовок некоректний, `easy_ticket` має панікувати з повідомленням про помилку,
//   що зберігається у відповідному варіанті переліку `TicketNewError`.
//   Якщо ж некоректний опис, функція має використати опис за замовчуванням:
//   "Description not provided".
pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => ticket,
        Err(TicketNewError::DescriptionError(_)) => {
            Ticket::new(title, "Description not provided".to_string(), status).unwrap()
        }
        Err(TicketNewError::TitleError(error)) => panic!("{error}"),
    }
}

#[derive(Debug, PartialEq)]
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
    pub(crate) fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError(
                "Title cannot be longer than 50 bytes".to_string(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be longer than 500 bytes".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}
