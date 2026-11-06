## Trait bounds

Досі ми бачили два способи використання traits:

- Відкриття доступу до «вбудованої» поведінки (наприклад, перевантаження операторів)
- Додавання нової поведінки до наявних типів (тобто extension traits)

Існує й третій спосіб: **generic програмування** (generics: узагальнені типи й функції).

## Проблема

Досі всі наші функції та методи працювали з **конкретними типами**.\
Код, який працює з конкретними типами, зазвичай легко писати й розуміти. Але
його можливості для повторного використання обмежені.\
Уявімо, наприклад, що ми хочемо написати функцію, яка повертає `true`, якщо ціле число парне.
Якщо працювати з конкретними типами, довелося б писати окрему функцію для кожного цілочисельного типу, який ми хочемо
підтримувати:

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// І так далі.
```

Або ж можна написати один extension trait, а потім окремі реалізації для кожного цілочисельного типу:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// І так далі.
```

Дублювання нікуди не зникає.

## Generic програмування

Це можна зробити краще за допомогою **generics**.\
Generics дають змогу писати код, який працює з **type parameter** (параметр типу), а не з конкретним типом:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` — це **generic функція**.\
Вона не прив'язана до конкретного типу вхідних даних. Натомість вона працює з будь-яким типом `T`, який:

- Реалізує trait `IsEven`.
- Реалізує trait `Debug`.

Цей контракт виражено за допомогою **trait bound**: `T: IsEven + Debug`.\
Символ `+` використовують, щоб вимагати від `T` реалізації кількох traits. `T: IsEven + Debug` означає те саме, що й
«де `T` реалізує `IsEven` **і** `Debug`».

## Trait bounds

Для чого потрібні trait bounds у `print_if_even`?\
Щоб з'ясувати це, спробуймо їх прибрати:

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

Цей код не скомпілюється:

```text
error[E0599]: no method named `is_even` found for type parameter `T` 
              in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found 
  |                    for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ 
  |   `T` cannot be formatted using `{:?}` because 
  |         it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```

Без trait bounds компілятор не знає, що **вміє робити** `T`.\
Він не знає, що `T` має метод `is_even`, і не знає, як відформатувати `T` для виведення.
З погляду компілятора «голий» `T` взагалі не має жодної поведінки.\
Trait bounds звужують множину типів, які можна використовувати: вони гарантують, що поведінка, потрібна тілу функції,
справді наявна.

## Синтаксис: inline trait bounds

В усіх наведених вище прикладах trait bounds задано за допомогою конструкції **`where` clause**:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  Це `where` clause
{
    // [...]
}
```

Якщо trait bounds прості, їх можна **вбудувати** (inline) безпосередньо поруч із type parameter:

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           Це inline trait bound
    // [...]
}
```

## Синтаксис: змістовні назви

У прикладах вище ми використовували `T` як назву type parameter. Це поширена домовленість для випадків, коли функція має
лише один type parameter.\
Утім, ніщо не заважає вам використати змістовнішу назву:

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

Насправді змістовні назви **бажано** використовувати, коли задіяно кілька type parameters або коли назва
`T` недостатньо передає роль типу у функції.
Називаючи type parameters, дбайте про максимальну ясність і читабельність — так само, як і з назвами змінних чи параметрів функцій.
Проте дотримуйтеся домовленостей Rust: [для назв type parameters використовуйте стиль UpperCamelCase](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case).

## Сигнатура функції — понад усе

Можливо, вам цікаво, навіщо взагалі потрібні trait bounds. Хіба компілятор не може вивести потрібні traits з тіла функції?\
Міг би, але не робитиме цього.\
Причина та сама, що й для [явних анотацій типів параметрів функцій](../../../ABasicCalculator/Variables/Theory/task.md#function-arguments-are-variables):
кожна сигнатура функції — це контракт між кодом, що викликає функцію, і самою функцією, і його умови мають бути зазначені явно.
Це дає кращі повідомлення про помилки, кращу документацію, менше ненавмисних порушень сумісності між версіями
та швидшу компіляцію.
