## `Error::source`

Щоб завершити розгляд трейту `Error`, нам залишилося обговорити ще одну річ — метод `source`.

```rust
// Цього разу — повне визначення!
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

Метод `source` дає змогу отримати **причину помилки** (error cause), якщо вона є.\
Помилки часто утворюють ланцюжки, тобто одна помилка спричиняє іншу: є помилка високого рівня (наприклад,
не вдається під'єднатися до бази даних), спричинена помилкою нижчого рівня (наприклад, не вдається розпізнати ім'я хоста бази даних).
За допомогою методу `source` можна «пройти» весь ланцюжок помилок — це часто використовують, щоб зберегти контекст помилки в логах.

## Реалізація `source`

Трейт `Error` надає реалізацію за замовчуванням, яка завжди повертає `None` (тобто глибинної причини немає). Саме тому
в попередніх вправах вам не доводилося перейматися методом `source`.\
Цю реалізацію за замовчуванням можна перевизначити, щоб указати причину для свого типу помилки.

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

У цьому прикладі `DatabaseError` обгортає `std::io::Error` як своє джерело.
Потім ми перевизначаємо метод `source`, щоб під час виклику він повертав це джерело.

## `&(dyn Error + 'static)`

Що це за тип такий — `&(dyn Error + 'static)`?\
Розберімо його по частинах:

- `dyn Error` — це **трейт-об'єкт** (trait object). Так можна звернутися до будь-якого типу, що реалізує трейт `Error`.
- `'static` — особливий **специфікатор часу життя** (lifetime specifier).
  `'static` означає, що посилання дійсне «стільки, скільки нам потрібно», тобто протягом усього виконання програми.

Якщо скласти все докупи: `&(dyn Error + 'static)` — це посилання на трейт-об'єкт, який реалізує трейт `Error`
і залишається дійсним протягом усього виконання програми.

Поки що не надто переймайтеся цими поняттями. Докладніше ми розглянемо їх у наступних розділах.

## Реалізація `source` за допомогою `thiserror`

`thiserror` пропонує три способи автоматично реалізувати `source` для ваших типів помилок:

- Поле з назвою `source` автоматично використовується як джерело помилки.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- Поле, позначене атрибутом `#[source]`, автоматично використовується як джерело помилки.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- Поле, позначене атрибутом `#[from]`, автоматично використовується як джерело помилки, **а ще**
  `thiserror` автоматично згенерує реалізацію `From`, щоб перетворювати позначений тип на ваш тип помилки.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## Оператор `?`

Оператор `?` — це скорочений запис для поширення помилок (error propagation).\
У функції, яка повертає `Result`, він достроково завершує її виконання й повертає помилку, якщо `Result` — це `Err`.

Наприклад, такий код:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

еквівалентний цьому:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

Оператор `?` дає змогу значно скоротити код обробки помилок.\
Зокрема, оператор `?` автоматично перетворює тип помилки операції, яка може завершитися невдачею, на тип помилки
функції, якщо таке перетворення можливе (тобто якщо існує відповідна реалізація `From`).
