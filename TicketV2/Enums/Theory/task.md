## Enums

Згідно з логікою валідації, яку ви написали [в одному з попередніх уроків](../../../TicketV1/Validation/Theory/task.md),
тікет може мати лише кілька допустимих статусів: `To-Do`, `InProgress` і `Done`.\
Проте це зовсім не очевидно, якщо подивитися на поле `status` у struct `Ticket` або на тип параметра `status`
у методі `new`:

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

В обох випадках поле `status` ми представляємо типом `String`.
`String` — дуже загальний тип: з нього одразу не видно, що поле `status`
має обмежений набір можливих значень. Ба гірше, код, який викликає `Ticket::new`, дізнається, чи коректний
переданий статус, лише **під час виконання**.

З **enums** (переліками) можна зробити краще.

## `enum`

Enum — це тип, значення якого належать до фіксованого набору; ці значення називають **варіантами** (variants).\
У Rust enum оголошують за допомогою ключового слова `enum`:

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum`, як і `struct`, оголошує **новий тип Rust**.
