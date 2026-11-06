// TODO: реалізуйте `TryFrom<String>` і `TryFrom<&str>` для типу `TicketDescription`,
//   гарантуючи, що опис не порожній і не довший за 500 байтів.
//   Також реалізуйте трейти, потрібні для проходження тестів.

#[derive(Debug, PartialEq, Clone)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketDescriptionError {
    #[error("The description cannot be empty")]
    Empty,
    #[error("The description cannot be longer than 500 bytes")]
    TooLong,
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(value.to_string()))
    }
}

fn validate(description: &str) -> Result<(), TicketDescriptionError> {
    if description.is_empty() {
        Err(TicketDescriptionError::Empty)
    } else if description.len() > 500 {
        Err(TicketDescriptionError::TooLong)
    } else {
        Ok(())
    }
}
