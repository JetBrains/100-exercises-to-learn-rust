// TODO: коли вважатимете, що виконали цю вправу, встановіть `move_forward` у `true` у тесті `ready`.
//  Можете також попросити викладача перевірити ваш розв'язок!
use std::time::Duration;
use task_channels::data::TicketDraft;
use task_channels::{launch, Command};
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // Якщо потік уже не працює, тут станеться паніка,
        // бо канал буде закрито.
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // У цій вправі ми мало що можемо перевірити автоматично,
    // адже наш сервер не надає жодних операцій **читання**.
    // Ми ніяк не можемо дізнатися, чи вставки справді відбуваються і чи
    // відбуваються вони правильно.
    let move_forward = true;

    assert!(move_forward);
}
