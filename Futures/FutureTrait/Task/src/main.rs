//! TODO: змусьте код скомпілюватися, **змінивши порядок** інструкцій
//!  у функції `example`. Не можна змінювати ні функцію
//!  `spawner`, ні те, що робить кожен рядок у `example`.
//!   За потреби можна обгортати наявні інструкції в блоки `{}`.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }
    yield_now().await;
}

fn main() {}
