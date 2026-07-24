fn main() {
    let a = String::from("hello");
    let b = a; // ownership moves from `a` to `b`
    // Set a breakpoint on the next line and inspect `b` in the debugger.
    println!("{b}");
}
