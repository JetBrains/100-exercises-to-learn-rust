## Скасування

Що відбувається, коли незавершений future знищують (drop)?\
Середовище виконання більше не опитуватиме його, тож він уже не просуватиметься вперед.
Іншими словами, його виконання **скасовано**.

На практиці це часто трапляється під час роботи з тайм-аутами.
Наприклад:

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // Обгортаємо future у `Timeout`, який спливає через 10 мілісекунд.
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("Didn't receive a value within 10 ms");
    }
}
```

Коли тайм-аут спливе, future, який повернула `http_call`, буде скасовано.
Уявімо, що тіло `http_call` таке:

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```

Кожна точка призупинення стає **точкою скасування** (cancellation point).\
Середовище виконання не може витіснити `http_call`, тож її можна відкинути лише після того,
як вона віддала керування виконавцю через `.await`.
Це працює рекурсивно: наприклад, `stream.write_all(&request)`, найімовірніше, має у своїй
реалізації кілька точок призупинення. Цілком можливо, що `http_call` надішле
_частину_ запиту, перш ніж її буде скасовано, — тоді з'єднання буде розірвано, а тіло
запиту так і не буде передано до кінця.

## Очищення

Механізм скасування в Rust доволі потужний: він дає змогу тому, хто викликає, скасувати задачу, що вже виконується,
без жодної співпраці з боку самої задачі.\
Водночас це може бути доволі небезпечно. Іноді бажано виконати
**плавне скасування** (graceful cancellation), щоб гарантувати, що перед перериванням операції
буде виконано певні дії з очищення.

Наприклад, розгляньмо такий вигаданий API для SQL-транзакції:

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```

У разі скасування ідеально було б явно перервати незавершену транзакцію, а не
залишати її «висіти».
На жаль, Rust не надає цілком надійного механізму для такого роду
**асинхронних** операцій очищення.

Найпоширеніша стратегія — покластися на трейт `Drop`, щоб запланувати потрібну
роботу з очищення. Для цього можна:

- запустити нову задачу в середовищі виконання;
- поставити повідомлення в чергу каналу;
- породити фоновий потік.

Оптимальний вибір залежить від контексту.

## Скасування запущених задач

Коли ви запускаєте задачу за допомогою `tokio::spawn`, ви вже не можете її знищити (drop):
вона належить середовищу виконання.\
Проте за потреби її можна скасувати через відповідний `JoinHandle`:

```rust
async fn run() {
    let handle = tokio::spawn(/* some async task */);
    // Скасовуємо запущену задачу
    handle.abort();
}
```

## Додаткові матеріали

- Будьте вкрай обережні, коли використовуєте макрос `select!` з `tokio`, щоб влаштувати «перегони» (race) між двома різними futures.
  Повторювати ту саму задачу в циклі небезпечно, якщо ви не можете гарантувати **безпечність скасування** (cancellation safety).
  Докладніше читайте в [документації `select!`](https://tokio.rs/tokio/tutorial/select).\
  Якщо вам потрібно чергувати елементи двох асинхронних потоків даних (streams), наприклад сокета й каналу, натомість краще використати
  [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge).
- У деяких випадках [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) може
  бути кращим вибором, ніж `JoinHandle::abort`.
