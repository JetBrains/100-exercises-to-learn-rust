## Реалізація traits

Коли тип визначено в іншому crate (crate: одиниця компіляції Rust), як-от `u32` зі стандартної бібліотеки Rust, ви
не можете безпосередньо визначати для нього нові методи. Якщо спробуєте:

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

компілятор поскаржиться:

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

## Extension trait

**Extension trait** — це trait (trait: набір методів, які тип може реалізувати; аналог інтерфейсу), основне призначення якого — додавати нові методи
до зовнішніх типів, таких як `u32`.
Саме цей pattern (типовий прийом) ви застосували в попередній вправі, коли визначили
trait `IsEven`, а потім реалізували його для `i32` і `u32`. Після цього ви
можете вільно викликати `is_even` для цих типів, доки `IsEven` перебуває у scope.

```rust
// Вводимо trait у scope
use my_library::IsEven;

fn main() {
    // Викликаємо його метод для типу, який його реалізує
    if 4.is_even() {
        // [...]
    }
}
```

## Одна реалізація

Реалізації traits, які ви можете писати, мають певні обмеження.\
Найпростіше й найочевидніше з них: не можна реалізувати той самий trait двічі
в одному crate для одного й того самого типу.

Наприклад:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

Компілятор відхилить такий код:

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

Коли `IsEven::is_even` викликається для значення типу `u32`, не може бути жодної неоднозначності
щодо того, яку реалізацію trait використати, тому реалізація може бути лише одна.

## Orphan rule

Коли залучено кілька crates, з'являються нюанси.
Зокрема, має виконуватися принаймні одна з таких умов:

- Trait визначено в поточному crate
- Тип, що реалізує trait, визначено в поточному crate

У Rust це називають **orphan rule**. Його мета — зробити однозначним процес вибору методу,
який буде викликано (method resolution).

Уявіть таку ситуацію:

- Crate `A` визначає trait `IsEven`
- Crate `B` реалізує `IsEven` для `u32`
- Crate `C` надає (іншу) реалізацію trait `IsEven` для `u32`
- Crate `D` залежить і від `B`, і від `C` та викликає `1.is_even()`

Яку реалізацію слід використати? Ту, що визначено в `B`? Чи ту, що визначено в `C`?\
Доброї відповіді немає, тому й запровадили orphan rule — щоб запобігти такому сценарію.
Завдяки orphan rule не скомпілюється ні crate `B`, ні crate `C`.

## Додаткові матеріали

- Orphan rule у тому вигляді, як його сформульовано вище, має деякі застереження й винятки.
  Зазирніть у [довідник мови (The Rust Reference)](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence),
  якщо хочете ознайомитися з його нюансами.
