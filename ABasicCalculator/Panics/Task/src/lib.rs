/// За початковою та кінцевою точками подорожі й часом, витраченим на неї,
/// обчислює середню швидкість подорожі.
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: якщо `time_elapsed` дорівнює 0, викличте паніку з повідомленням "The journey took no time at all. That's impossible!"
    //  (Рядок лишаємо англійською — саме його перевіряє тест.)

    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!");
    }

    (end - start) / time_elapsed
}
