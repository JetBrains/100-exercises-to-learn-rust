pub enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

impl Shape {
    // TODO: Implement the `n_sides` method using a `match`.
    pub fn n_sides(&self) -> u8 {
        match self {
            Shape::Circle => 0,
            Shape::Square | Shape::Rectangle => 4,
            Shape::Triangle => 3,
            Shape::Pentagon => 5,
        }
    }
}
