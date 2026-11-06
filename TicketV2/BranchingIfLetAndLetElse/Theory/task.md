## Лаконічне розгалуження

Ваш розв'язок попередньої вправи, імовірно, має приблизно такий вигляд:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```

Вас цікавить лише варіант `Status::InProgress`.
Чи справді потрібно зіставляти значення з усіма іншими варіантами?

На допомогу приходять нові конструкції!

## `if let`

Конструкція `if let` дає змогу зіставити значення з одним варіантом enum,
не обробляючи всіх інших варіантів.

Ось як за допомогою `if let` можна спростити метод `assigned_to`:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```

## `let/else`

Якщо гілка `else` має достроково завершити функцію (паніка теж вважається достроковим виходом!),
можна використати конструкцію `let/else`:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```

Вона дає змогу оголосити деструктуризовану змінну без жодного «зсуву праворуч» (right drift),
тобто змінна з'являється на тому самому рівні відступу,
що й код перед нею.

## Стиль

І `if let`, і `let/else` — ідіоматичні конструкції Rust.\
Користуйтеся ними на свій розсуд, щоб зробити код зрозумілішим,
але не перестарайтеся: `match` завжди до ваших послуг, коли він потрібен.
