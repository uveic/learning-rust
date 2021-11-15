use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_again() -> Result<String, io::Error> {
    // Same functionality as above
    let mut f = File::open("hello.txt")?;
    // The ? placed after a Result value is defined to work in almost the same way as the
    // match expressions we defined to handle the Result values. If the value of the Result
    // is an Ok, the value inside the Ok will get returned from this expression, and the
    // program will continue. If the value is an Err, the Err will be returned from the
    // whole function as if we had used the return keyword so the error value gets propagated
    // to the calling code.

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_one_more() -> Result<String, io::Error> {
    // Still same functionality as above
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_one_more_again() -> Result<String, io::Error> {
    // Still same functionality as above
    fs::read_to_string("hello.txt")
    // Reading a file into a string is a fairly common operation, so Rust provides the convenient
    // fs::read_to_string function that opens the file, creates a new String, reads the contents
    // of the file, puts the contents into that String, and returns it. Of course, using
    // fs::read_to_string doesn't give us the opportunity to explain all the error handling,
    // so we did it the longer way first.
}

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let _f1 = File::open("hello.txt").unwrap();
    // .unwrap() => run against a Result, it returns the contet of Ok if found or panics if not

    let _f2 = File::open("hello.txt").expect("Failed to open hello_again.txt");
    // .expect("Error message") => same as .unwrap() but with a custom error message

    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Problem reading username: {:?}", e),
    };

    match read_username_from_file_again() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Problem reading username: {:?}", e),
    };

    match read_username_from_file_one_more() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Problem reading username: {:?}", e),
    };

    match read_username_from_file_one_more_again() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Problem reading username: {:?}", e),
    };
}
