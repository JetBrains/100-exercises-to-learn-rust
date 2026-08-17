## Do I need to install Rust?
##

You need a Rust toolchain, but you probably don't need to install it by hand.

If you followed the **Set up your IDE** lesson you've checked this already.
This is the longer version, for when the automatic setup doesn't do what you want, or when something stops working later
on.

RustRover can fetch a toolchain for you: use the **Set up toolchain** banner above the editor, or open
**Settings / Preferences | Rust** and click **Install Rustup**, which installs both the toolchain and the standard
library. The **Toolchain version** and **Standard library** fields then fill in by themselves.

If you already have a toolchain and RustRover hasn't picked it up, you can enter the paths yourself on that same settings
page. RustRover needs Rust 1.70 or newer.

A missing or misconfigured toolchain is the usual explanation when nothing compiles at all.
Full details: [Rust toolchain](https://www.jetbrains.com/help/rust/rust-toolchain.html) in the RustRover documentation.
