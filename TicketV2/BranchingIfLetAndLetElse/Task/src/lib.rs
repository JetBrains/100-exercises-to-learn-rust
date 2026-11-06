pub enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    // TODO: реалізуйте метод `radius` за допомогою
    //  `if let` або `let/else`.
    pub fn radius(&self) -> f64 {
        if let Shape::Circle { radius } = self {
            *radius
        } else {
            panic!("Not a circle");
        }
    }
}
