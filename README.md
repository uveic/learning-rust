# My journey learning Rust

Code, notes, commands and other stuff I find useful or interesting while learning Rust. Mainly stuff I find I might want to come back to it in the future. In no specific order. It might change over time.

### Useful commands:

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

### Code Snippets
`.iter()`: returns each element in a collection. Example below.

`.enumerate()`: wraps the result of `.iter()` and returns each element as part of a tuple instead: the first element is the index, and the second element is a reference to the element. Example:
```rust
let bytes = String::from("hello").as_bytes();
for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
        return i;
    }
}
```

String slice:
```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

`println!("{:?}", some_struct);`: the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`.

`println!("{:#?}", some_struct);`: we get a fancier output using `:#?`.

`dbg!(some_expresion);`: prints the file and line number of where that `dbg!` macro call occurs in your code along with the resulting value of that expression.

### [The Stack and the Heap](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)

* **The stack** stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.
* All data stored on the stack must have a known, fixed size.
* The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
* Pushing to the stack is faster than allocating on the heap.
* Accessing data in the heap is slower than accessing data on the stack.
* When your code calls a function, the values passed into the function and the function’s local variables get pushed onto the stack.

### [Ownership](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)
* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.
* Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
* When a variable goes out of scope, Rust calls a special function for us: `drop`.
* Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
* But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time.
* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.

### [Variable Scope](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variable-scope)
```rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                              // this scope is now over, and s is no
                                   // longer valid
```

### [Collections](https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html)
* Vectors (`Vec<T>`) allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart. [Standard Library Documentation](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html).
* A string is a collection of characters. Strings are complicated. Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
* Hash Maps (`HashMap<K, V>`): The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`. It does this via a hashing function, which determines how it places these keys and values into memory. [Standard Library Documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
