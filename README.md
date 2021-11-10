# My journey learning Rust

I am going to leave here code, notes, commands and other stuff I find useful or interesting while learning Rust.  

###Useful commands:

`rustup update`: update Rust to the latest version.

`rustc --version`, `rustup --version` or `cargo --version`: display the current version.

`rustup doc`: open the local documentation in your browser.

`rustfmt`: If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called rustfmt to format your code in a particular style.

`cargo build`: creates an executable file in `target/debug/hello_cargo`.

`cargo run`: compile the code and then run the resulting executable all-in-one command.

`cargo check`: quickly checks your code to make sure it compiles but doesn’t produce an executable.

`cargo build --release`: When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in `target/release` instead of `target/debug`.

`cargo update`: Ignore the Cargo.lock file and figures out all the dependencies from Cargo.toml.

`cargo doc --open`: build documentation provided by all of your dependencies locally and open it in your browser.
