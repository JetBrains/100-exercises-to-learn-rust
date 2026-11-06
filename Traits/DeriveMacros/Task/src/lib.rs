// TODO: щоб ця вправа успішно скомпілювалася, бракує реалізації trait (такої, яку можна згенерувати за допомогою derive macro).
//   Виправте це!
//
// # Коротко про `Debug`
//
// `Debug` повертає представлення типу Rust, придатне для налагодження (звідси й назва: англ. debug — «налагоджувати»).
// `assert_eq!` вимагає, щоб `Ticket` реалізовував `Debug`, бо, коли assertion (перевірка) не проходить, макрос намагається
// вивести обидві сторони порівняння в термінал.
// Якщо тип, що порівнюється, не реалізує `Debug`, макрос не знає, як відобразити значення!

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
