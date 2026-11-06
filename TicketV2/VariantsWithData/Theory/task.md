## Варіанти можуть містити дані

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Наш enum `Status` — це те, що зазвичай називають **C-style enum**.\
Кожен варіант — це просто мітка, щось на кшталт іменованої константи. Такі enums є в багатьох мовах
програмування: C, C++, Java, C#, Python тощо.

Проте enums у Rust здатні на більше. До кожного варіанта можна **прикріпити дані**.

## Варіанти

Припустімо, ми хочемо зберігати ім'я людини, яка зараз працює над тікетом.\
Ця інформація є лише тоді, коли тікет у роботі. Для тікета, який ще чекає на виконання або вже виконаний,
її не буде.
Це можна змоделювати, додавши до варіанта `InProgress` поле типу `String`:

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

Тепер `InProgress` — це **struct-like variant** (структуроподібний варіант).\
Його синтаксис, по суті, повторює той, яким ми оголошували struct, — просто тут struct «вбудовано» в enum як варіант.

## Доступ до даних варіанта

Якщо спробувати звернутися до `assigned_to` в екземплярі `Status`,

```rust
let status: Status = /* */;

// Це не скомпілюється
println!("Assigned to: {}", status.assigned_to);
```

компілятор нас зупинить:

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

Поле `assigned_to` **належить конкретному варіанту**: воно є не в усіх екземплярах `Status`.\
Щоб дістатися до `assigned_to`, потрібен **pattern matching** (зіставлення зі зразком):

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## Bindings

У pattern `Status::InProgress { assigned_to }` ідентифікатор `assigned_to` — це **binding** (прив'язка).\
Ми **деструктуризуємо** (destructuring) варіант `Status::InProgress` і прив'язуємо поле `assigned_to` до
нової змінної, яка теж називається `assigned_to`.\
За бажання поле можна прив'язати до змінної з іншою назвою:

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```
