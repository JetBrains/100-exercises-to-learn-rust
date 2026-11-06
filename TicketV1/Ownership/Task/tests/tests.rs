#[cfg(test)]
mod tests {
    use task_ownership::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // Якщо ви змінили сигнатури, як вимагається, цей код має скомпілюватися:
        // ми можемо викликати ці методи один за одним, бо вони позичають `self`,
        // а не беруть його у володіння.
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
