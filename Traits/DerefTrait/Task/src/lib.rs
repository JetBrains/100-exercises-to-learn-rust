// TODO: щоразу, коли `title` і `description` повертаються через свої методи доступу, їх
//   слід нормалізувати — тобто прибрати пробільні символи на початку й у кінці.
//   У стандартній бібліотеці Rust є метод, який може в цьому допомогти, але ви не
//   знайдете його в документації до `String`.
//   Чи зможете ви з'ясувати, де його визначено і як ним користуватися?

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

    pub fn title(&self) -> &str {
        self.title.trim()
    }

    pub fn description(&self) -> &str {
        self.description.trim()
    }
}
