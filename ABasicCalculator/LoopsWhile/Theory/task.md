## Цикли, частина 1: `while`

Вашу реалізацію `factorial` довелося писати з використанням рекурсії.\
Вам це може здаватися природним, особливо якщо ви маєте досвід функціонального програмування.
А може — дивним, якщо ви звикли до імперативніших мов, як-от C чи Python.

Подивімося, як натомість реалізувати ту саму функціональність за допомогою **циклу**.

## Цикл `while`

Цикл `while` дає змогу виконувати блок коду, доки **умова** істинна.\
Ось загальний синтаксис:

```rust
while <condition> {
    // код, який потрібно виконати
}
```

Наприклад, нам може знадобитися підсумувати числа від 1 до 5:

```rust
let sum = 0;
let i = 1;
// «поки i менше або дорівнює 5»
while i <= 5 {
    // `+=` — це скорочений запис `sum = sum + i`
    sum += i;
    i += 1;
}
```

Цей код раз у раз додаватиме 1 до `i` та `i` до `sum`, доки `i` не стане більшим за 5.

## Ключове слово `mut`

Приклад вище в такому вигляді не скомпілюється. Ви отримаєте приблизно таку помилку:

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: consider making this binding mutable: `mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         first assignment to `i`
  |         help: consider making this binding mutable: `mut i`
...
8 |         i += 1;
  |         ^^^^^^ cannot assign twice to immutable variable
```

Так стається тому, що змінні в Rust за замовчуванням **незмінювані** (immutable).\
Після того як значення присвоєно, змінити його не можна.

Якщо ви хочете дозволити зміни, потрібно оголосити змінну **змінюваною** (mutable) за допомогою ключового слова `mut`:

```rust
// Тепер `sum` та `i` змінювані!
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```

Тепер код скомпілюється й виконається без помилок.

## Додаткові матеріали

- [Документація циклу `while`](https://doc.rust-lang.org/std/keyword.while.html)
