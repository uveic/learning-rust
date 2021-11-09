fn main() {
    println!("Hello, world!");
    another_function(5, 45);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(87);
    println!("The value of x is: {}", x); // This is a comment

    // This is a line comment
    // Multiple line comments can be done like this

    print_label_measurement(5, 'h');
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
