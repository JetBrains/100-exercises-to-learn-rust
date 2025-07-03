This task requires you to refactor error handling for `Ticket` creation. Your primary goals are to:

- Define the `TicketNewError` `enum` with specific variants for title and description issues.
- Update `Ticket::new` to return a `Result` using this new `enum` for errors.
- Implement `easy_ticket` to use `TicketNewError` for panicking on title errors, but default descriptions for description errors.
 
All detailed instructions are in the `TODO` comments in the code.