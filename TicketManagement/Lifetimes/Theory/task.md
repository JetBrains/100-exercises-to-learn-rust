## Lifetimes

Спробуймо доповнити попередню вправу реалізацією `IntoIterator` для `&TicketStore`, щоб цикли `for` були
максимально зручними.

Почнімо із заповнення найбільш «очевидних» частин реалізації:

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = // Що тут має бути?

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

Яким має бути `type IntoIter`?\
Інтуїтивно це має бути тип, який повертає `self.tickets.iter()`, тобто тип, який повертає `Vec::iter()`.\
Зазирнувши в документацію стандартної бібліотеки, ви побачите, що `Vec::iter()` повертає `std::slice::Iter`.
Ось визначення `Iter`:

```rust
pub struct Iter<'a, T> { /* fields omitted */ }
```

`'a` — це **lifetime parameter** (параметр часу життя).

## Lifetime parameters

Lifetimes — це **мітки**, за якими компілятор Rust відстежує, як довго reference (посилання на значення) —
mutable (змінюваний) чи immutable (незмінюваний) — залишається дійсним.\
Lifetime будь-якого reference обмежений scope (ділянкою коду, де змінна доступна) того значення, на яке він
посилається. Rust завжди ще під час компіляції перевіряє, що references не використовуються після того, як значення,
на яке вони посилаються, було знищено (drop), — так він запобігає dangling pointers (висячим вказівникам) і помилкам
use-after-free (використання пам'яті після її звільнення).

Це вам має бути знайомо: ми вже бачили ці концепції в дії, коли обговорювали ownership (володіння) і borrowing (запозичення).
Lifetimes — це просто спосіб **назвати** проміжок, протягом якого певний reference дійсний.

Назви стають важливими, коли у вас є кілька references і потрібно пояснити, як вони **пов'язані між собою**.
Погляньмо на сигнатуру `Vec::iter()`:

```rust
impl <T> Vec<T> {
    // Трохи спрощено
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // [...]
    }
}
```

`Vec::iter()` — generic (узагальнений) за lifetime parameter з назвою `'a`.\
`'a` використовується, щоб **пов'язати** lifetime `Vec` із lifetime `Iter`, який повертає `iter()`.
Простими словами: `Iter`, який повертає `iter()`, не може жити довше, ніж reference на `Vec` (`&self`), з якого
його створено.

Це важливо, бо, як ми вже обговорювали, `Vec::iter` повертає ітератор по **references** на елементи `Vec`.
Якщо `Vec` знищити, references, які повертає ітератор, стануть недійсними. Rust має гарантувати, що цього не
станеться, і lifetimes — це інструмент, за допомогою якого він забезпечує дотримання цього правила.

## Lifetime elision

У Rust є набір правил — **lifetime elision rules** (правила пропуску часів життя), — які в багатьох випадках
дають змогу не писати явні lifetime annotations.
Наприклад, у вихідному коді `std` визначення `Vec::iter` має такий вигляд:

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // [...]
    }
}
```

У сигнатурі `Vec::iter()` немає явного lifetime parameter.
З правил lifetime elision випливає, що lifetime `Iter`, який повертає `iter()`, прив'язаний до lifetime reference `&self`.
`'_` можна вважати **placeholder** (позначкою-«замінником») для lifetime reference `&self`.

Офіційну документацію щодо lifetime elision ви знайдете в розділі [Додаткові матеріали](#references).\
У більшості випадків можна покластися на компілятор: він сам підкаже, коли потрібно додати явні lifetime annotations.

## Додаткові матеріали <a id="references"></a>

- [std::vec::Vec::iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
- [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [Правила lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)
