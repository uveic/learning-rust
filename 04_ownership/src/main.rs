fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s1);
    // ^^^^^^^^^^^^^^^^^^^ This fails because s1 was moved to s2,
    // in other words, s1 is no longer available

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // ^ But this code seems to contradict what we just learned: we don’t have a call
    // to clone, but x is still valid and wasn’t moved into y.
    // The reason is that types such as integers that have a known size at compile time
    // are stored entirely on the stack, so copies of the actual values are quick to make.

    let s3 = String::from("hello");
    takes_ownership(s3);
    // println!("s3 value in main is: {}", s3);

    // ^ This throws an error at compile time because the value of s3 was moved to
    // some_string in "takes_ownership()", or in other words, "takes_ownership()" took ownership
    // of s3. It was a pointer that was moved from s3 to some_string.
    // The variable and its content was used inside "takes_ownership()" and it was freed
    // once it terminated, or in other words, Rust freed the allocated memory to store its
    // value right after the last } in "takes_ownership()" as it's no longer needed.

    let x = 5;
    makes_copy(x);
    println!("X in main value is: {}", x);
    // This isn't a problem because x was copied when it was passed to "makes_copy()"
    // In other words, the value of x is stored in two variables, there is no pointer involved

    let _s10 = gives_ownership();
    let s11 = String::from("hi");
    let s13 = takes_and_gives_ownership(s11);
    println!("s13 value is: {}", s13);

    let s20 = String::from("how are you doing?");
    let (s21, len) = calculate_length_taking_ownership(s20);
    println!("The length of '{}' is {}.", s21, len);

    let s30 = String::from("another string");
    let len = calculate_length(s30);
    println!("The length of '{}' is {}.", len.0, len.1);
    let s31 = String::from("here we are again");
    let len = calculate_length_again(&s31);
    println!("The length of '{}' is {}.", s31, len);

    let mut s40 = String::from("some text");
    let len = append_suffix_and_calculate_length(&mut s40);
    println!("The length of '{}' is {}.", s40, len);

    let s50 = String::from("this is another string");
    println!("The first word in '{}' is '{}'", s50, fist_word(&s50[..]));

    let string_literal = "a string literal";
    println!("The first word in '{}' is '{}'", string_literal, fist_word(&string_literal[..]));
    println!("The first word in '{}' is '{}'", string_literal, fist_word(string_literal));

    // But mutable references have one big restriction: you can have only one mutable reference
    // to a particular piece of data at a time.
    let mut s60 = String::from("hello");
    {
        let _r1 = &mut s60;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s60;

    let mut s61 = String::from("hello");
    let r1 = &s61; // no problem
    let r2 = &s61; // no problem
    println!("{} and {}", r1, r2);
    // let r3 = &mut s61; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_ownership(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length_taking_ownership(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length(s: String) -> (String, usize) {
    // s.push_str("some text");
    // This throws an error because 's' is a reference hence it can't be modified
    // References are immutable

    let length = s.len();

    (s, length)
}

fn calculate_length_again(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn append_suffix_and_calculate_length(s: &mut String) -> usize {
    s.push_str(" [APPENDED]");
    s.len()
}

fn fist_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
