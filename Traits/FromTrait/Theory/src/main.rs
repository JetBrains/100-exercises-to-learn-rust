fn main() {
    let s: String = String::from("hi"); // From<&str>
    let n: u64 = u64::from(10u8);        // From<u8>
    // Put the caret on `from` (or `From`) and Go to Definition.
    println!("{s} {n}");
}
