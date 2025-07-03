pub use crate::status::{ParseStatusError, Status};

// We've seen how to declare modules in one of the earliest exercises, but
// we haven't seen how to extract them into separate files.
// Let's fix that now!
//
// In the simplest case, when the extracted module is a single file, it is enough to
// create a new file with the same name as the module and move the module content there.
// The module file should be placed in the same directory as the file that declares the module.
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.

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

        // TODO: Parse the status string into a `Status` enum.
        let status = Status::try_from(status)?;

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}
