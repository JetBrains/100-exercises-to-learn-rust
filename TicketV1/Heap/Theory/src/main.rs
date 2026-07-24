fn main() {
    let mut s = String::with_capacity(5);
    s.push_str("Hey");
    // Put the caret on `String` (or `push_str`) and Go to Definition.
    println!("{s} (len {}, cap {})", s.len(), s.capacity());
}
