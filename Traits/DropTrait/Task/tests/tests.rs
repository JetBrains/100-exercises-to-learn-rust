#[cfg(test)]
mod tests {
    use task_drop_trait::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // Бомба має панікувати під час drop
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // Бомба не має панікувати під час drop,
        // оскільки її знешкоджено
    }
}
