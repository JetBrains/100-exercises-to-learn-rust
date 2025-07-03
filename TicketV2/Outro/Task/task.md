This task requires you to refactor the `Ticket` struct's fields into distinct types: `TicketTitle`, `TicketDescription`, and `Status`.

You'll find `TODO` comments in each of the corresponding modules (`title.rs`, `description.rs`, `status.rs`, and `lib.rs`). Your main goal is to:

- Implement `TryFrom<String>` and `TryFrom<&str>` for `TicketTitle`, `TicketDescription`, and `Status`, ensuring all specified validation rules (e.g., length, non-empty, case-insensitivity for `Status`) are enforced within their respective implementations.
- Implement any additional traits needed to make the associated tests pass for each of these new types.
- Observe how the `Ticket` struct's fields are now public, leveraging the encapsulation provided by the new types.

This task emphasizes using Rust's type system to enforce invariants and improve code organization across multiple files.