pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: спираючись на вивчене в уроці «Heap», замініть `TODO`
//  правильним **розміром у stack** для відповідного типу.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // Це запитання з підступом!
        // Цього разу «інтуїтивна» відповідь виявляється правильною,
        // але загалом розміщення structs у пам'яті — складніша тема.
        // Якщо цікаво, докладніше читайте в розділі «Data layout» книги Rustonomicon:
        // https://doc.rust-lang.org/nomicon/data.html
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
