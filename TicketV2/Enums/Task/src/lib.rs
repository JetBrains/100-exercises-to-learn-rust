// TODO: використайте `Status` як тип для `Ticket::status`
//   За потреби змініть сигнатуру й реалізацію всіх інших методів.

#[derive(Debug, PartialEq)]
// `derive` працює рекурсивно: він може згенерувати `PartialEq`, лише якщо всі поля теж реалізують `PartialEq`.
// Те саме стосується `Debug`. Зробіть із `Status` усе потрібне, щоб це запрацювало.
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}
