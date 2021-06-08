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
    let mut f1 = File::open("hello.txt")?;
    let mut s1 = String::new();
    f1.read_to_string(&mut s1)?;
    Ok(s1)
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
}

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");

    let f = match f {
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

    // let f1 = File::open("hello_again.txt").unwrap();

    // let f2 = File::open("hello_again.txt").expect("Failed to open hello_again.txt");
}
