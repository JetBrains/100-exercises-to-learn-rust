// TODO: маючи вектор цілих чисел, розділіть його на дві половини
//  та обчисліть суму кожної половини в окремому потоці.
//  Не виділяйте жодної пам'яті в купі. Не допускайте жодних витоків пам'яті.

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    std::thread::scope(|s| {
        let left = s.spawn(|| left.iter().sum::<i32>());
        let right = s.spawn(|| right.iter().sum::<i32>());
        left.join().unwrap() + right.join().unwrap()
    })
}
