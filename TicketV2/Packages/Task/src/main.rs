// Це файл `main.rs`, тому `cargo` вважає його коренем binary target.

// TODO: виправте цей зламаний імпорт. Перегляньте library target у теці `src`.
//   Library target має надавати публічну функцію з назвою `hello_world`, яка не приймає аргументів
//   і нічого не повертає.
use task_packages::hello_world;

// Це точка входу binary crate.
fn main() {
    hello_world();
}
