enum Status {
    ToDo,
    Done,
}

fn main() {
    for status in [Status::ToDo, Status::Done] {
        // Try typing `status.match` then Tab to generate this match.
        match status {
            Status::ToDo => println!("to do"),
            Status::Done => println!("done"),
        }
    }
}
