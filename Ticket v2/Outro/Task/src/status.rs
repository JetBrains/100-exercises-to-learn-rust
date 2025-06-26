// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = ParseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(ParseStatusError {
                invalid_status: value,
            }),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_lowercase().try_into()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("`{invalid_status}` is not a valid status. Use one of: ToDo, InProgress, Done")]
pub struct ParseStatusError {
    invalid_status: String,
}
