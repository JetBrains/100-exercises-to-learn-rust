struct Guard {
    name: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}

fn main() {
    let _g = Guard { name: String::from("g1") };
    // Set a breakpoint inside `drop` and watch when it runs as `main` ends.
    println!("end of main");
}
