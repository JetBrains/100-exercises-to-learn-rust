// TODO: маючи вектор цілих чисел, допустіть витік виділеної для нього пам'яті в heap.
//  Потім розділіть отриманий статичний slice на дві половини й
//  обчисліть суму кожної половини в окремому потоці.
//  Підказка: зверніть увагу на `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let v = v.leak();
    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);

    let handle1 = thread::spawn(move || v1.into_iter().sum::<i32>());
    let handle2 = thread::spawn(move || v2.into_iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}
