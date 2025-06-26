// TODO: A (derivable) trait implementation is missing for this exercise to compile successfully.
//   Fix it!
//
// # `Debug` primer
//
// `Debug` returns a representation of a Rust type that's suitable for debugging (hence the name).
// `assert_eq!` requires `Ticket` to implement `Debug` because, when the assertion fails, it tries to
// print both sides of the comparison to the terminal.
// If the compared type doesn't implement `Debug`, it doesn't know how to represent them!

#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        Ticket {
            title,
            description,
            status,
        }
    }
}
