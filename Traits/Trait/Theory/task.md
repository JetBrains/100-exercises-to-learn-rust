## Traits

Знову погляньмо на наш тип `Ticket`:

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

Досі всі наші тести робили assertions (перевірки) за допомогою полів `Ticket`.

```rust
assert_eq!(ticket.title(), "A new title");
```

А що, як ми захочемо порівняти два екземпляри `Ticket` безпосередньо?

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

Компілятор нас зупинить:

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` — це новий тип. За замовчуванням до нього **не прив'язано жодної поведінки**.\
Rust не вгадує магічним чином, як порівнювати два екземпляри `Ticket`, лише тому, що вони містять значення типу `String`.

Утім, компілятор Rust підштовхує нас у правильному напрямку: він підказує, що нам, можливо, бракує реалізації
`PartialEq`. `PartialEq` — це **trait**!

## Що таке traits?

Traits — це спосіб, у який Rust визначає **інтерфейси**.\
Trait визначає набір методів, які тип має реалізувати, щоб виконати контракт цього trait.

### Визначення trait

Синтаксис визначення trait виглядає так:

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

Наприклад, можна визначити trait `MaybeZero`, який вимагає від типів, що його реалізують, визначити метод `is_zero`:

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

### Реалізація trait

Щоб реалізувати trait для типу, ми використовуємо ключове слово `impl`, так само як і для звичайних методів (метод, визначений безпосередньо для типу без використання trait, також називають **inherent method**),
але синтаксис дещо інший:

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // Тіло методу
    }
}
```

Наприклад, ось як реалізувати trait `MaybeZero` для користувацького числового типу `WrappingU32`:

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

### Виклик методу trait

Щоб викликати метод trait, ми використовуємо оператор `.`, так само як і для звичайних методів:

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

Щоб викликати метод trait, мають виконуватися дві умови:

- Тип має реалізовувати цей trait.
- Trait має бути у scope (scope: ділянка коду, де змінна доступна).

Щоб виконати другу умову, може знадобитися додати для trait інструкцію `use`:

```rust
use crate::MaybeZero;
```

Цього робити не потрібно, якщо:

- Trait визначено в тому самому модулі, де відбувається виклик.
- Trait визначено в **prelude** стандартної бібліотеки.
  Prelude — це набір traits і типів, які автоматично імпортуються в кожну програму на Rust.
  Усе відбувається так, ніби на початок кожного модуля Rust додано `use std::prelude::*;`.

Список traits і типів, що входять до prelude, можна знайти в
[документації Rust](https://doc.rust-lang.org/std/prelude/index.html).
