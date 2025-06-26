This task has two main parts, both described in the `TODO` comments:

- Implement the `Debug`, `Display`, and `Error` traits for the `TicketNewError` `enum`. You'll use the `write!` macro for `Display`.
- Implement the `easy_ticket` function. It should panic on invalid titles (using the `TicketNewError` message) but use a default description for invalid descriptions.