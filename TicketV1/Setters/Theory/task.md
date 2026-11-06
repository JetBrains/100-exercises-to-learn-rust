## Mutable references

Тепер ваші accessor methods мають виглядати так:

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

Кілька `&` тут і там — і справу зроблено!\
Тепер ми можемо звертатися до полів екземпляра `Ticket`, не поглинаючи його.
Далі подивімося, як додати до нашого struct `Ticket` **setter methods** (методи-сетери).

## Setters

Setter methods дають користувачам змогу змінювати значення приватних полів `Ticket` і водночас гарантують
дотримання його інваріантів (тобто не можна зробити заголовок `Ticket` порожнім рядком).

У Rust є два поширені способи реалізувати setters:

- приймати на вхід `self`;
- приймати на вхід `&mut self`.

### Приймаємо на вхід `self`

Перший підхід виглядає так:

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // Перевіряємо новий заголовок [...]
        self.title = new_title;
        self
    }
}
```

Метод бере ownership над `self`, змінює заголовок і повертає змінений екземпляр `Ticket`.\
Використовувати його можна так:

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```

Оскільки `set_title` бере ownership над `self` (тобто **поглинає його**), результат треба знову присвоїти змінній.
У прикладі вище ми скористалися **variable shadowing**, щоб повторно використати те саме ім'я змінної:
коли ви оголошуєте нову змінну з тим самим ім'ям, що й наявна, нова змінна **затіняє** (shadows) стару.
Це поширений pattern (типовий прийом) у коді на Rust.

`self`-setters дуже зручні, коли потрібно змінити кілька полів одразу: виклики можна об'єднати в ланцюжок!

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### Приймаємо на вхід `&mut self`

Другий підхід до setters — з `&mut self` — натомість виглядає так:

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // Перевіряємо новий заголовок [...]
        
        self.title = new_title;
    }
}
```

Цього разу метод приймає на вхід mutable reference на `self`, змінює заголовок — і все.
Нічого не повертається.

Використовувати його можна так:

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// Використовуємо змінений тікет
```

Ownership лишається за кодом, що викликає метод, тож початкова змінна `ticket` і далі дійсна. Знову присвоювати результат не потрібно.
Утім, `ticket` треба позначити як mutable, бо ми беремо на неї mutable reference.

`&mut`-setters мають недолік: їхні виклики не можна об'єднати в ланцюжок.
Оскільки вони не повертають змінений екземпляр `Ticket`, не можна викликати наступний setter для результату першого.
Кожен setter доводиться викликати окремо:

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```
