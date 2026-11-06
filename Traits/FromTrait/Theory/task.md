## `From` і `Into`

Повернімося туди, звідки почалася наша мандрівка рядками:

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```

Тепер ми знаємо достатньо, щоб розібратися, що тут робить `.into()`.

## Проблема

Ось сигнатура методу `new`:

```rust
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

Ми також бачили, що рядкові літерали (як-от `"A title"`) мають тип `&str`.\
Тут маємо невідповідність типів: очікується `String`, а в нас `&str`.
Цього разу жодна магічна coercion (неявне перетворення типу) нас не врятує — потрібно **виконати перетворення**.

## `From` і `Into`

Стандартна бібліотека Rust визначає два traits для **перетворень, які не можуть завершитися невдачею** (infallible conversions):
`From` та `Into`. Вони містяться в модулі `std::convert`.

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

Ці визначення traits демонструють кілька понять, яких ми ще не бачили: **supertraits**
та **неявні trait bounds** (trait bound: вимога, щоб тип реалізовував певний trait).
Спершу розберімося з ними.

### Supertrait / Subtrait <a id="supertrait--subtrait"></a>

Синтаксис `From: Sized` означає, що `From` — **subtrait** `Sized`: будь-який тип, що
реалізує `From`, мусить також реалізовувати `Sized`.
Інакше кажучи, `Sized` — **supertrait** `From`.

### Неявні trait bounds

Щоразу, коли у вас є generic type parameter, компілятор неявно припускає, що він `Sized`.

Наприклад:

```rust
pub struct Foo<T> {
    inner: T,
}
```

насправді еквівалентно такому:

```rust
pub struct Foo<T: Sized> 
{
    inner: T,
}
```

У випадку `From<T>` визначення trait еквівалентне такому:

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

Іншими словами, `Sized` мають бути _обидва_: і `T`, і тип, що реалізує `From<T>`, хоча
перший із цих trait bounds неявний.

### Негативні trait bounds

Відмовитися від неявного trait bound `Sized` можна за допомогою **негативного trait bound**:

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            Це негативний trait bound
    inner: T,
}
```

Цей синтаксис читається як «`T` може бути `Sized`, а може й не бути», і він дає змогу
підставити замість `T` DST (тип динамічного розміру; наприклад, `Foo<str>`). Утім, це особливий випадок: негативні trait bounds
існують лише для `Sized`, з іншими traits їх використовувати не можна.

## Із `&str` у `String`

У [документації `std`](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)
можна побачити, які типи з `std` реалізують trait `From`.\
Ви знайдете там, що `String` реалізує `From<&str> for String`. Отже, можна написати:

```rust
let title = String::from("A title");
```

Утім, досі ми здебільшого використовували `.into()`.\
Якщо переглянути [реалізації `Into`](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors),
ви не знайдете там `Into<String> for &str`. У чому ж річ?

`From` і `Into` — **двоїсті traits**.\
Зокрема, `Into` реалізовано для будь-якого типу, що реалізує `From`, за допомогою **blanket implementation**
(реалізація trait одразу для всіх типів, що задовольняють умову):

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

Якщо тип `U` реалізує `From<T>`, то `Into<U> for T` реалізується автоматично. Саме тому
можна написати `let title = "A title".into();`.

## `.into()`

Щоразу, коли ви бачите `.into()`, відбувається перетворення між типами.\
Але який цільовий тип?

У більшості випадків цільовий тип або:

- задано сигнатурою функції чи методу (наприклад, `Ticket::new` у нашому прикладі вище);
- вказано в оголошенні змінної за допомогою анотації типу (наприклад, `let title: String = "A title".into();`).

`.into()` працюватиме без додаткових зусиль, якщо компілятор може однозначно вивести цільовий тип із контексту.
