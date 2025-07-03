// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    temperatures: [Option<i32>; 7],
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures {
            temperatures: [None; 7],
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        let index = weekday2index(&day);
        self.temperatures[index]
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        let index = weekday2index(&day);
        self.temperatures[index] = Some(temperature);
    }
}

fn weekday2index(day: &Weekday) -> usize {
    match day {
        Weekday::Monday => 0,
        Weekday::Tuesday => 1,
        Weekday::Wednesday => 2,
        Weekday::Thursday => 3,
        Weekday::Friday => 4,
        Weekday::Saturday => 5,
        Weekday::Sunday => 6,
    }
}
