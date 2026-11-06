// Визначте трейт з назвою `IsEven`, що має метод `is_even`, який повертає `true`, якщо `self` —
// парне число, і `false` в іншому разі.
//
// Потім реалізуйте цей трейт для `u32` та `i32`.

pub trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
