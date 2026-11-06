## Mutable slices

Щоразу, коли ми говорили про типи slices (зрізів), як-от `str` і `[T]`, ми використовували їхню форму immutable borrow: `&str` і `&[T]`.\
Але slices можуть бути й mutable!

Ось як створити mutable slice:

```rust
let mut numbers = vec![1, 2, 3];
let slice: &mut [i32] = &mut numbers;
```

Після цього можна змінювати елементи slice:

```rust
slice[0] = 42;
```

Так перший елемент `Vec` зміниться на `42`.

## Обмеження

Для immutable borrows рекомендація була однозначна: віддавайте перевагу references на slices, а не
references на owned type — тип, що володіє даними (наприклад, `&[T]`, а не `&Vec<T>`).\
З mutable borrows це **не** так.

Розгляньмо такий сценарій:

```rust
let mut numbers = Vec::with_capacity(2);
let mut slice: &mut [i32] = &mut numbers;
slice.push(1);
```

Цей код не скомпілюється!\
`push` — це метод `Vec`, а не slices. Це прояв загальнішого принципу: Rust не дозволяє
додавати елементи до slice чи вилучати їх із нього. Змінювати/замінювати можна лише ті елементи, які
вже там є.

У цьому сенсі `&mut Vec` чи `&mut String` строго потужніші за `&mut [T]` чи `&mut str`.\
Обирайте тип, який найкраще підходить для операцій, що їх вам потрібно виконати.
