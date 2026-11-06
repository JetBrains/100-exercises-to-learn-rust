## Scoped-потоки

Усі проблеми з lifetimes (lifetime: час життя посилання), які ми досі обговорювали, мають спільне джерело:
запущений потік може пережити свій батьківський потік.\
Цю проблему можна обійти за допомогою **scoped-потоків** — потоків, прив'язаних до певного scope (ділянка коду, де змінна доступна).

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
        println!("Here's the first half of v: {first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
    });
});

println!("Here's v: {v:?}");
```

Розберімо, що тут відбувається.

## `scope`

Функція `std::thread::scope` створює новий **scope**.\
`std::thread::scope` приймає на вхід closure (замикання) з єдиним аргументом — екземпляром `Scope`.

## Spawn потоків у scope

`Scope` надає метод `spawn`.\
На відміну від `std::thread::spawn`, усі потоки, запущені через `Scope`,
будуть **автоматично приєднані** (join), коли scope закінчиться: вихід зі scope відбудеться лише після їхнього завершення.

Якби ми «переклали» попередній приклад на `std::thread::spawn`,
він мав би такий вигляд:

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("Here's the first half of v: {first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("Here's the second half of v: {second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("Here's v: {v:?}");
```

## Borrowing (запозичення) з оточення

Однак «перекладений» приклад не скомпілювався б: компілятор поскаржився б,
що `&v` не можна використовувати в наших запущених потоках, оскільки його lifetime не
`'static`.

Із `std::thread::scope` такої проблеми немає — ви можете **безпечно позичати з оточення**.

У нашому прикладі `v` створюється до місць, де запускаються потоки.
Його буде знищено лише _після_ того, як `scope` поверне керування. Водночас
усі потоки, запущені всередині `scope`, гарантовано завершаться _до_ того, як `scope` поверне керування,
тож ризику отримати dangling references (висячі посилання) немає.

Компілятор не скаржитиметься!
