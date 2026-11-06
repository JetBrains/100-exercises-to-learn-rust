pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: реалізуйте функцію `new`.
    //  Мають виконуватися такі вимоги:
    //   - Дозволено лише статуси `To-Do`, `In Progress` і `Done`.
    //   - Поля `title` і `description` не можуть бути порожніми.
    //   - `title` має бути завдовжки не більше 50 байтів.
    //   - `description` має бути завдовжки не більше 500 байтів.
    //  Метод має панікувати, якщо будь-яку з вимог не виконано.
    //  Повідомлення паніки (саме такі, англійською — їх перевіряють тести): "Only `To-Do`, `In Progress`, and `Done` statuses are allowed",
    //  "Title cannot be empty", "Description cannot be empty", "Title cannot be longer than 50 bytes",
    //  "Description cannot be longer than 500 bytes"
    //
    // Вам знадобиться те, що ви вивчили в попередніх вправах,
    // а також деякі методи `String`. Скористайтеся документацією стандартної бібліотеки Rust,
    // щоб знайти найвідповідніші варіанти -> https://doc.rust-lang.org/std/string/struct.String.html
    pub fn new(title: String, description: String, status: String) -> Self {
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

        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Self {
            title,
            description,
            status,
        }
    }
}
