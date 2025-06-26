/// Given the start and end points of a journey, and the time it took to complete the journey,
/// calculate the average speed of the journey.
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: Panic with a custom message: "The journey took no time at all. That's impossible!"
    //  if `time_elapsed` is 0

    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!");
    }

    (end - start) / time_elapsed
}
