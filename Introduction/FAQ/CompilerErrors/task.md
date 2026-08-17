## Do I have to read all these compiler errors?
##

Yes, and there's a good chance you'll come to like them.

Rust's compiler produces some of the best diagnostics of any language.
It tells you what's wrong, often points at the exact spot, and frequently suggests the fix.
Think of it as your pair programming partner for the rest of this course.

Read the message in full before you start changing code. The answer is often already in there.

For a longer explanation of any error, note its code (something like `E0382`) and look it up in the
[Rust error index](https://doc.rust-lang.org/error_codes/).

> 💡 **Note**
>
> If errors only show up when you run the tests, check the external linter setting from the
> **Set up your IDE** lesson. With it on, you get the compiler's feedback as you type.
