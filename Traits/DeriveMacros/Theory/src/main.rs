#[derive(Debug, Clone)]
struct Ticket {
    title: String,
}

fn main() {
    let t = Ticket { title: String::from("Learn traits") };
    let t2 = t.clone(); // `Clone` was derived for us
    // Put the caret on `#[derive(...)]` and Expand macro to see the generated impls.
    println!("{t:?}");
    println!("still have the original: {}", t2.title);
}
