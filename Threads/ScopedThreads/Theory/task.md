## Scoped-потоки

Усі проблеми з часами життя, які ми досі обговорювали, мають спільне джерело:
породжений потік може пережити свій батьківський потік.\
Цю проблему можна обійти за допомогою **scoped-потоків** (scoped threads) — потоків, прив'язаних до певної області (scope).

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

Функція `std::thread::scope` створює нову **область** (scope).\
`std::thread::scope` приймає на вхід замикання з єдиним аргументом — екземпляром `Scope`.

## Породження потоків в області

`Scope` надає метод `spawn`.\
На відміну від `std::thread::spawn`, усі потоки, породжені через `Scope`,
будуть **автоматично приєднані** (join), коли область закінчиться: вихід з області відбудеться лише після їхнього завершення.

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

## Запозичення з оточення

Однак «перекладений» приклад не скомпілювався б: компілятор поскаржився б,
що `&v` не можна використовувати в наших породжених потоках, оскільки його час життя не
`'static`.

Із `std::thread::scope` такої проблеми немає — ви можете **безпечно позичати з оточення**.

У нашому прикладі `v` створюється до місць, де породжуються потоки.
Його буде знищено лише _після_ того, як `scope` поверне керування. Водночас
усі потоки, породжені всередині `scope`, гарантовано завершаться _до_ того, як `scope` поверне керування,
тож ризику отримати висячі посилання (dangling references) немає.

Компілятор не скаржитиметься!
