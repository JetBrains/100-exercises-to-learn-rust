## Trait `Future`

### Проблема локального `Rc`

Повернімося до сигнатури `tokio::spawn`:

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```

Що _насправді_ означає, що `F` має бути `Send`?\
Як ми бачили в попередньому уроці, це означає, що будь-яке значення, яке future захоплює (capture) з
оточення, де його запускають (spawn), має бути `Send`. Але цим справа не обмежується.

Будь-яке значення, яке _утримується під час очікування в точці .await_, має бути `Send`.\
Розгляньмо приклад:

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // Значення, яке не є `Send`,
    // створене _всередині_ асинхронної функції
    let non_send = Rc::new(1);
    
    // Точка `.await`, яка нічого не робить
    yield_now().await;

    // Локальне значення, що не є `Send`, досі потрібне
    // після `.await`
    println!("{}", non_send);
}
```

Компілятор відхилить цей код:

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ 
    | future returned by `example` is not `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- has type `Rc<i32>` which is not `Send`
12  |     // A `.await` point
13  |     yield_now().await;
    |                 ^^^^^ 
    |   await occurs here, with `non_send` maybe used later
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```

Щоб зрозуміти, чому так, нам потрібно уточнити своє розуміння
асинхронної моделі Rust.

## Trait `Future`

Ми вже казали, що `async`-функції повертають **futures** — типи, які реалізують
trait `Future` (trait: набір методів, які тип може реалізувати; аналог інтерфейсу). Future можна уявляти собі як **скінченний автомат** (state machine).
Він перебуває в одному з двох станів:

- **pending** (в очікуванні): обчислення ще не завершилося.
- **ready** (готово): обчислення завершилося, ось результат.

Це закодовано у визначенні trait:

```rust
trait Future {
    type Output;
    
    // Поки що не зважайте на `Pin` і `Context`
    fn poll(
      self: Pin<&mut Self>, 
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```

### `poll`

Метод `poll` — серце trait `Future`.\
Сам по собі future нічого не робить. Щоб він просувався вперед, його потрібно **опитувати** (poll).\
Викликаючи `poll`, ви просите future виконати певну роботу.
`poll` намагається просунутися вперед, а тоді повертає один із таких варіантів:

- `Poll::Pending`: future ще не готовий. Потрібно викликати `poll` знову пізніше.
- `Poll::Ready(value)`: future завершився. `value` — результат обчислення,
  що має тип `Self::Output`.

Щойно `Future::poll` повернув `Poll::Ready`, future більше не слід опитувати: він
завершився, робити більше нічого.

### Роль runtime

Вам рідко (якщо взагалі колись) доведеться викликати poll безпосередньо.\
Це робота вашого async runtime: він має всю потрібну інформацію (`Context`
у сигнатурі `poll`), щоб ваші futures просувалися вперед щоразу, коли це можливо.

## `async fn` і futures

Ми вже працювали з високорівневим інтерфейсом — асинхронними функціями.\
А тепер розглянули низькорівневий примітив — `Future trait`.

Як вони пов'язані?

Щоразу, коли ви позначаєте функцію як асинхронну, вона повертатиме future.
Компілятор перетворить тіло вашої асинхронної функції на **скінченний автомат**:
по одному стану на кожну точку `.await`.

Повернімося до нашого прикладу з `Rc`:

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```

Компілятор перетворив би його на enum (перелік), який виглядає приблизно так:

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>),
    Terminated,
}
```

Коли викликають `example`, вона повертає `ExampleFuture::NotStarted`. Future ще жодного разу
не опитували, тож нічого не відбулося.\
Коли runtime вперше опитає його, `ExampleFuture` просунеться до наступної
точки `.await`: він зупиниться на стані `ExampleFuture::YieldNow(Rc<i32>)` скінченного
автомата й поверне `Poll::Pending`.\
Коли його опитають знову, він виконає решту коду (`println!`) і
поверне `Poll::Ready(())`.

Якщо подивитися на його представлення у вигляді скінченного автомата (`ExampleFuture`),
стає зрозуміло, чому `example` не є `Send`: автомат утримує `Rc`, а отже,
не може бути `Send`.

## Yield points

Як ви щойно побачили на прикладі `example`, кожна точка `.await` створює новий проміжний
стан у життєвому циклі future.\
Тому точки `.await` також називають **yield points** (точками призупинення): ваш future _віддає керування_ (yield)
назад до runtime, який його опитував, і той може призупинити його та (за потреби)
запланувати до виконання інший task, просуваючись таким чином на кількох фронтах конкурентно.

До того, чому так важливо віддавати керування, ми повернемося в одному з наступних уроків.
