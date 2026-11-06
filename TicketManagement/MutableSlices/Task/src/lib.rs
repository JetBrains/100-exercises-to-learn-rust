// TODO: Визначте функцію з назвою `squared`, яка підносить до квадрата (до степеня 2) усі значення `i32` у slice.
//  Slice має змінюватися на місці.

pub fn squared(slice: &mut [i32]) {
    for i in slice.iter_mut() {
        *i *= *i;
    }
}
