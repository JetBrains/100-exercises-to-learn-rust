// TODO: Implement `Debug`, `Display` and `Error` for the `TicketNewError` enum.
//  When implementing `Display`, you may want to use the `write!` macro from Rust's standard library.
//  The docs for the `std::fmt` module are a good place to start and look for examples:
//  https://doc.rust-lang.org/std/fmt/index.html#write

#[derive(Debug)]
pub enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

impl std::fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TicketNewError::TitleError(msg) => write!(f, "{}", msg),
            TicketNewError::DescriptionError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TicketNewError {}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => ticket,
        Err(err) => match err {
            TicketNewError::TitleError(_) => panic!("{err}"),
            TicketNewError::DescriptionError(_) => {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
            }
        },
    }
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
