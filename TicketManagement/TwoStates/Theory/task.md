## Ідентифікатори тікетів

Знову поміркуймо над нашою системою керування тікетами.\
Зараз модель тікета має такий вигляд:

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

Тут дечого бракує: **ідентифікатора**, за яким можна однозначно визначити тікет.\
Цей ідентифікатор має бути унікальним для кожного тікета. Це можна гарантувати, якщо генерувати його автоматично
під час створення нового тікета.

## Уточнюємо модель

Де зберігати id?\
Можна додати нове поле до struct (структури) `Ticket`:

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

Але до створення тікета ми не знаємо його id. Тож від самого початку цього значення там бути не може.\
Довелося б зробити поле необов'язковим:

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

Це теж не ідеально — нам довелося б обробляти випадок `None` щоразу, коли ми отримуємо тікет зі сховища,
хоча ми знаємо, що після створення тікета id має бути завжди.

Найкраще рішення — мати два різні **стани** тікета, представлені двома окремими типами:
`TicketDraft` і `Ticket`:

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

`TicketDraft` — це тікет, який ще не створено. У нього немає ні id, ні статусу.\
`Ticket` — це вже створений тікет. У нього є і id, і статус.\
Оскільки кожне поле в `TicketDraft` і `Ticket` містить у собі власні обмеження, нам не потрібно дублювати логіку
в цих двох типах.
