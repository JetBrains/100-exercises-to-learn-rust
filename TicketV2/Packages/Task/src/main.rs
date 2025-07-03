// This is a `main.rs` file, therefore `cargo` interprets this as the root of a binary target.

// TODO: fix this broken import. Check library target in the `src` directory.
//   The library target should expose a public function named `hello_world` that takes no arguments
//   and returns nothing.
use task_packages::hello_world;

// This is the entrypoint of the binary.
fn main() {
    hello_world();
}
