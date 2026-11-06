## `TryFrom` і `TryInto`

У попередньому розділі ми розглянули [traits `From` і `Into`](../../../Traits/FromTrait/Theory/task.md) —
ідіоматичні інтерфейси Rust для перетворень типів, які **не можуть завершитися невдачею** (infallible).\
Але що, як успіх перетворення не гарантовано?

Тепер ми знаємо про помилки достатньо, щоб обговорити аналоги `From` і `Into` для перетворень, які **можуть завершитися невдачею** (fallible):
`TryFrom` і `TryInto`.

### `TryFrom` і `TryInto`

І `TryFrom`, і `TryInto` оголошено в модулі `std::convert`, так само як `From` і `Into`.

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

Головна відмінність між `From`/`Into` і `TryFrom`/`TryInto` полягає в тому, що останні повертають тип `Result`.\
Завдяки цьому перетворення може завершитися невдачею й повернути помилку замість паніки.

## `Self::Error`

І `TryFrom`, і `TryInto` мають асоційований тип `Error`.
Це дає змогу кожній реалізації задати власний тип помилки — в ідеалі той, що найкраще підходить для конкретного
перетворення.

`Self::Error` — це спосіб звернутися до асоційованого типу `Error`, оголошеного в самому trait.

## Двоїстість

Як і `From` та `Into`, `TryFrom` і `TryInto` — двоїсті traits.\
Якщо ви реалізуєте `TryFrom` для типу, `TryInto` отримуєте безкоштовно.
