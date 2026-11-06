## Derive macros

Реалізовувати `PartialEq` для `Ticket` було трохи нудно, чи не так?
Вам довелося вручну порівнювати кожне поле struct (struct: структура).

## Синтаксис деструктуризації

До того ж така реалізація крихка: якщо визначення struct зміниться
(наприклад, з'явиться нове поле), вам доведеться не забути оновити реалізацію `PartialEq`.

Цей ризик можна зменшити, якщо **деструктуризувати** (destructuring) struct на окремі поля:

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

Якщо визначення `Ticket` зміниться, компілятор видасть помилку й повідомить, що ваша
деструктуризація більше не вичерпна (exhaustive).\
Також можна перейменувати поля struct, щоб уникнути shadowing змінних (shadowing: нова змінна з тим самим ім'ям «затіняє» стару):

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

Деструктуризація — корисний pattern, який варто мати у своєму арсеналі, але
є ще зручніший спосіб: **derive macros**.

## Макроси

Ви вже зустрічали кілька макросів у попередніх вправах:

- `assert_eq!` і `assert!` — у тестах
- `println!` — щоб виводити текст у консоль

Макроси Rust — це **генератори коду**.\
Вони генерують новий код Rust на основі вхідних даних, які ви надаєте, а потім цей згенерований код компілюється разом
із рештою вашої програми. Деякі макроси вбудовано в стандартну бібліотеку Rust, але ви також можете
писати власні. У цьому курсі ми не створюватимемо власних макросів, але кілька корисних
посилань ви знайдете в [розділі «Додаткові матеріали»](#further-reading).

### Перегляд згенерованого коду

Деякі IDE дають змогу розгорнути макрос і переглянути згенерований код. Якщо такої можливості немає, можна скористатися
[`cargo-expand`](https://github.com/dtolnay/cargo-expand).

### Derive macros

**Derive macro** — це особливий різновид макросів Rust. Його вказують як **атрибут** над struct.

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

Derive macros використовують, щоб автоматизувати реалізацію поширених (і «очевидних») traits для власних типів.
У прикладі вище trait `PartialEq` автоматично реалізується для `Ticket`.
Якщо розгорнути макрос, ви побачите, що згенерований код функціонально еквівалентний тому, який ви написали вручну,
хоча читати його трохи важче:

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title 
            && self.description == other.description
            && self.status == other.status
    }
}
```

Коли це можливо, компілятор сам підказуватиме вам реалізувати traits через derive macros.

## Додаткові матеріали <a id="further-reading"></a>

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
