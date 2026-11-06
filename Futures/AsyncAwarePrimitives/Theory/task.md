## Примітиви з підтримкою async

Переглядаючи документацію `tokio`, ви помітите, що він надає багато типів,
які «віддзеркалюють» типи зі стандартної бібліотеки, але в асинхронному варіанті:
locks (блокування), канали (channels), таймери тощо.

В асинхронному контексті варто надавати перевагу цим асинхронним альтернативам,
а не їхнім синхронним відповідникам.

Щоб зрозуміти чому, погляньмо на `Mutex` — lock із взаємним виключенням, який ми
розглядали в попередньому розділі.

## Розбір випадку: `Mutex`

Розгляньмо простий приклад:

```rust
use std::sync::{Arc, Mutex};

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().unwrap();
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // Тут `guard` знищується (drop)
}

/// Використовує `v` як тіло HTTP-запиту.
async fn http_call(v: &[u64]) {
  // [...]
}
```

### `std::sync::MutexGuard` і yield points

Цей код скомпілюється, але він небезпечний.

Ми намагаємося захопити lock `Mutex` із `std` в асинхронному контексті.
Потім ми утримуємо отриманий `MutexGuard` у yield point — `.await` для
`http_call`.

Уявімо, що два tasks конкурентно виконують `run` в однопотоковому
runtime. Ми спостерігаємо таку послідовність подій планування:

```text
     Task A          Task B
        | 
  Acquire lock
Yields to runtime
        | 
        +--------------+
                       |
             Tries to acquire lock
```

Маємо deadlock. Task B ніколи не зможе захопити lock, бо зараз його
утримує task A, який віддав керування (yield) runtime, не звільнивши
lock, і його вже не буде заплановано знову, бо runtime не може витіснити task B.

### `tokio::sync::Mutex`

Цю проблему можна розв'язати, перейшовши на `tokio::sync::Mutex`:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().await;
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // Тут `guard` знищується
}
```

Тепер захоплення lock — асинхронна операція, яка віддає керування runtime,
якщо не може просунутися далі.\
Якщо повернутися до попереднього сценарію, відбуватиметься таке:

```text
       Task A          Task B
          | 
  Acquires the lock
  Starts `http_call`
  Yields to runtime
          | 
          +--------------+
                         |
             Tries to acquire the lock
              Cannot acquire the lock
                 Yields to runtime
                         |
          +--------------+
          |
`http_call` completes      
  Releases the lock
   Yield to runtime
          |
          +--------------+
                         |
                 Acquires the lock
                       [...]
```

Усе гаразд!

### Багатопотоковість вас не врятує

У попередньому прикладі контекстом виконання був однопотоковий runtime,
але той самий ризик зберігається навіть у multi-threaded runtime (багатопотоковому середовищі
виконання).\
Різниця лише в кількості конкурентних tasks, потрібних для виникнення deadlock:
в однопотоковому runtime достатньо двох, а в multi-threaded runtime
знадобиться `N+1` tasks, де `N` — кількість потоків runtime (runtime threads).

### Недоліки

`Mutex` з підтримкою async має свою ціну з погляду продуктивності.\
Якщо ви впевнені, що за lock немає значної конкуренції,
_і_ уважно стежите, щоб ніколи не утримувати його в yield point, то
все ж можете використовувати `std::sync::Mutex` в асинхронному контексті.

Але зіставте виграш у продуктивності з ризиком для liveness (живості),
на який ви при цьому йдете.

## Інші примітиви

Ми взяли за приклад `Mutex`, але те саме стосується `RwLock`, семафорів тощо.\
В асинхронному контексті надавайте перевагу версіям із підтримкою async, щоб мінімізувати
ризик проблем.
