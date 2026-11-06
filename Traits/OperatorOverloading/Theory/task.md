## Перевантаження операторів

Тепер, коли ми маємо базове уявлення про те, що таке traits, повернімося до **перевантаження операторів**.
Перевантаження операторів — це можливість визначати власну поведінку для таких операторів, як `+`, `-`, `*`, `/`, `==`, `!=` тощо.

## Оператори — це traits

У Rust оператори — це traits.\
Для кожного оператора існує відповідний trait, який визначає поведінку цього оператора.
Реалізувавши цей trait для свого типу, ви **відкриваєте** можливість використовувати з ним відповідні оператори.

Наприклад, [trait `PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) визначає поведінку
операторів `==` і `!=`:

```rust
// Визначення trait `PartialEq` зі стандартної бібліотеки Rust
// (поки що *трохи* спрощене)
pub trait PartialEq {
    // Required method (метод trait, який треба реалізувати)
    //
    // `Self` — це ключове слово Rust, яке означає
    // «тип, що реалізує цей trait»
    fn eq(&self, other: &Self) -> bool;

    // Provided method
    fn ne(&self, other: &Self) -> bool { ... }
}
```

Коли ви пишете `x == y`, компілятор шукає реалізацію trait `PartialEq` для типів `x` і `y`
та замінює `x == y` на `x.eq(y)`. Це синтаксичний цукор!

Ось як основні оператори відповідають traits:

| Оператор                 | Trait                                                                   |
| ------------------------ | ----------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` і `!=`              | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`, `>`, `<=` і `>=`    | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

Арифметичні оператори містяться в модулі [`std::ops`](https://doc.rust-lang.org/std/ops/index.html),
а оператори порівняння — у модулі [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html).

## Реалізації за замовчуванням

У коментарі до `PartialEq::ne` сказано, що «`ne` — provided method».\
Це означає, що `PartialEq` надає **реалізацію за замовчуванням** для `ne` у визначенні trait — це пропущений
блок `{ ... }` у наведеному вище фрагменті.\
Якщо розгорнути пропущений блок, він виглядатиме так:

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

Усе так, як ви й очікували: `ne` — це заперечення `eq`.\
Оскільки реалізацію за замовчуванням надано, під час реалізації `PartialEq` для свого типу можна не реалізовувати `ne`.
Достатньо реалізувати `eq`:

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // Реалізації `ne` тут немає
}
```

Утім, ви не зобов'язані використовувати реалізацію за замовчуванням.
Під час реалізації trait її можна перевизначити:

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // Власна реалізація
    }

    fn ne(&self, other: &MyType) -> bool {
        // Власна реалізація
    }
}
```
