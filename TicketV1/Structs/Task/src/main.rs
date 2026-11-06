// Визначте структуру з назвою `Order` і такими полями:
// - `price` — беззнакове ціле число
// - `quantity` — беззнакове ціле число
//
// Вона також повинна мати метод `is_available`, який повертає `true`, якщо кількість (`quantity`)
// більша за 0, і `false` в іншому разі.

struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}

fn main() {
    let order = Order {
        price: 100,
        quantity: 10,
    };
    assert!(order.is_available());

    let order = Order {
        price: 100,
        quantity: 0,
    };
    assert!(!order.is_available());
}
