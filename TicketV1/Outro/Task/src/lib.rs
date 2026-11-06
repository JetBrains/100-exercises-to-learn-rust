// TODO: Визначте новий тип `Order`.
//   Він має зберігати три значення: `product_name`, `quantity` і `unit_price`.
//   Назва товару (product name) не може бути порожньою чи довшою за 300 байтів.
//   Кількість (quantity) має бути строго більшою за нуль.
//   Ціна за одиницю (unit price) вказується в центах і має бути строго більшою за нуль.
//   Order повинен мати метод `total`, який повертає загальну вартість замовлення.
//   Order повинен надавати сетери й гетери для кожного поля.
//
// Тести розташовано в теці `tests`.
// Тека `tests` — особливе місце для `cargo`: саме там він шукає **інтеграційні тести**.
// «Інтеграційні» тут має цілком конкретне значення: такі тести перевіряють **публічний API** вашого проєкту.
// Вам доведеться стежити за видимістю своїх типів і методів: інтеграційні
// тести не мають доступу до приватних елементів чи елементів з `pub(crate)`.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        validate_product_name(&product_name);
        validate_quantity(&quantity);
        validate_unit_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        validate_product_name(&product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        validate_quantity(&quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        validate_unit_price(&unit_price);
        self.unit_price = unit_price;
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}

fn validate_product_name(product_name: &String) {
    if product_name.is_empty() {
        panic!("Product name cannot be empty");
    }
    if product_name.len() > 300 {
        panic!("Product name cannot be longer than 300 bytes");
    }
}

fn validate_quantity(quantity: &u32) {
    if quantity == &0 {
        panic!("Quantity must be greater than zero");
    }
}

fn validate_unit_price(unit_price: &u32) {
    if unit_price == &0 {
        panic!("Unit price must be greater than zero");
    }
}
