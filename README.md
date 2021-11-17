# My journey learning Rust

Code, notes, commands and other stuff I find useful or interesting while learning Rust. Mainly stuff I might want to come back to it in the future. In no specific order. It might change over time.

### Commands:

`rustup update`: update Rust to the latest version.

`rustc --version`, `rustup --version` or `cargo --version`: display the current version.

`rustup doc`: open the local documentation in your browser.

`rustfmt`: format your code in a particular style.

`cargo build`: create an executable file in `target/debug/hello_cargo`.

`cargo run`: compile the code and then run the resulting executable all-in-one command.

`cargo check`: quickly check your code to make sure it compiles. It doesn't produce an executable.

`cargo build --release`: compile your project with optimizations when it is finally ready for release. An executable will be created in `target/release` instead of `target/debug`.

`cargo update`: ignore the `Cargo.lock` file and figure out all the dependencies from `Cargo.toml`.

`cargo doc --open`: build documentation provided by all of your dependencies locally and open it in your browser.

`cargo test`: run tests.
* `cargo test -- --help`: show help.
* `cargo test -- --test-threads=1`: control over the number of threads used.
* `cargo test -- --show-output`: show the output (of `println!()`) of successful tests.
* `cargo test name_of_test_you_want_to_run`: run a specific test.
* `cargo test part_of_a_test_name`: run any test whose name matches this value.
* `cargo test -- --ignore`: run only ignored tests. All ignore tests include the line `#[ignore]` before the test.
* `cargo test --test integration_test`: run a particular integration test function.

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

### Rust Terminology

`Monomorphization`: the process of turning generic code into specific code by filling in the concrete types that are used when compiled. Rust implements generics in such a way that your code doesn't run any slower using generic types than it would with concrete types.

`Trait`: Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose. A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Traits are similar to a feature often called interfaces in other languages, although with some differences. [↗️](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)

`coherence` and the `orphan rule`: One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate. But we can’t implement external traits on external types. For example, we can’t implement the `Display` trait on `Vec<T>` within our crate, because `Display` and `Vec<T>` are defined in the standard library and aren't local to our crate. This restriction is part of a property of programs called `coherence`, and more specifically the `orphan rule`, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.

`Closure`: anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context.

### Programming Terminology

`Syntactic sugar`: syntax within a programming language that is designed to make things easier to read or to express. It makes the language "sweeter" for human use: things can be expressed more clearly, more concisely, or in an alternative style that some may prefer.  [↗️](https://en.wikipedia.org/wiki/Syntactic_sugar)

`Statically typed language`: A language is statically typed if the type of a variable is known at compile time. For some languages this means that you as the programmer must specify what type each variable is; other languages offer some form of type inference, the capability of the type system to deduce the type of a variable. The main advantage here is that all kinds of checking can be done by the compiler, and therefore a lot of trivial bugs are caught at a very early stage.

`Dynamically typed language`: A language is dynamically typed if the type is associated with run-time values, and not named variables/fields/etc. This means that you as a programmer can write a little quicker because you do not have to specify types every time (unless using a statically-typed language with type inference).

`Strongly typed language`: Strongly typed is a concept used to refer to a programming language that enforces strict restrictions on intermixing of values with differing data types. Generally, a strongly typed language has stricter typing rules at compile time, which implies that errors and exceptions are more likely to happen during compilation. Most of these rules affect variable assignment, function return values, procedure arguments and function calling. Dynamically typed languages (where type checking happens at run time) can also be strongly typed. Note that in dynamically typed languages, values have types, not variables. The opposite of a strongly typed language is a "weakly (or loosely) typed language". [↗️](https://en.wikipedia.org/wiki/Strong_and_weak_typing)

`Test Driven Development (TDD)`: is a software development process relying on software requirements being converted to test cases before software is fully developed, and tracking all software development by repeatedly testing the software against all test cases. This is as opposed to software being developed first and test cases created later.

`Polymorphism`: the provision of a single interface to entities of different types or the use of a single symbol to represent multiple different types.The concept is borrowed from a principle in biology where an organism or species can have many different forms or stages.

`Parametric polymorphism`: Using parametric polymorphism, a function or a data type can be written generically so that it can handle values identically without depending on their type. Such functions and data types are called generic functions and generic datatypes respectively and form the basis of generic programming.

`Idiomatic Code`: following the conventions of the language. You want to find the easiest and most common ways of accomplishing a task rather than porting your knowledge from a different language. Pertaining or conforming to the natural mode of expression of a language.

`memoization or lazy evaluation pattern`: We can create a struct that will hold the closure and the resulting value of calling the closure. The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn't have to be responsible for saving and reusing the result.

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
* Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time.
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
