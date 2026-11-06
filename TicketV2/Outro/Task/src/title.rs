// TODO: реалізуйте `TryFrom<String>` і `TryFrom<&str>` для типу `TicketTitle`,
//   гарантуючи, що заголовок не порожній і не довший за 50 байтів.
//   Також реалізуйте traits, потрібні для проходження тестів.

#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(value.to_string()))
    }
}

fn validate(title: &str) -> Result<(), TicketTitleError> {
    if title.is_empty() {
        Err(TicketTitleError::Empty)
    } else if title.len() > 50 {
        Err(TicketTitleError::TooLong)
    } else {
        Ok(())
    }
}
