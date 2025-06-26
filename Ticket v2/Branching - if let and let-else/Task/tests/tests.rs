#[cfg(test)]
mod tests {
    use task_branching_if_let_and_let_else::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
        .radius();
    }
}
