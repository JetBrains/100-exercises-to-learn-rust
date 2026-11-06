// TODO: реалізуйте так звану «Drop-бомбу» (Drop bomb): тип, який панікує під час drop,
//  якщо над ним не виконали певної операції.
//  Очікуваний API можна побачити в тестах
pub struct DropBomb {
    defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defused: false }
    }

    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("Boom!");
        }
    }
}
