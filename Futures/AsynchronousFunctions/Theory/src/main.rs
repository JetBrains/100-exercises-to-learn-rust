async fn ticket_count() -> u32 {
    42
}

fn main() {
    // Creating a future doesn't run it. Turn on inlay hints to see its type here.
    let fut = ticket_count();
    let _ = fut;
}
