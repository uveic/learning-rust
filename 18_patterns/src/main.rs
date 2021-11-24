fn main() {

    // Conditional if let Expressions

    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_colour {
        println!("Using your favourite colour, {}, as the background", colour);
    } else if is_tuesday {
        println!("Tuesday is a green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background colour");
        } else {
            println!("Using orange as the background colour");
        }
    } else {
        println!("Using blue as the background colour");
    }

    // while let Conditional Loops

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for Loops

    let v = vec!['a', 'b', 'c'];

    // We use the enumerate method to adapt an iterator to produce a value and that value’s
    // index in the iterator, placed into a tuple. The first value produced is the tuple (0, 'a').
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let Statements

    // x is a pattern that means “bind what matches here to the variable x.”
    // Because the name x is the whole pattern, this pattern effectively means
    // “bind everything to the variable x, whatever the value is.”
    let _x = 5;

    // Function Parameters
    let point = (3, 5);
    print_coordinates(&point);

    // Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // Multiple Patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring to Break Apart Values
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructuring Enums
    let msg = Message::ChangeColour(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColour(r, g, b) => {
            println!("Change the colour to red {}, green {}, and blue {}", r, g, b)
        },
    }

    // Destructuring Nested Structs and Enums
    let msg = AnotherMessage::ChangeColour(Colour::Hsv(0, 160, 255));

    match msg {
        AnotherMessage::ChangeColour(Colour::Rgb(r, g, b)) => {
            println!("Change the colour to red {}, green {}, and blue {}", r, g, b)
        },
        AnotherMessage::ChangeColour(Colour::Hsv(h, s, v)) => {
            println!("Change the colour to hue {}, saturation {}, and value {}", h, s, v)
        },
        _ => (),
    }

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{} feet, {} inches, x: {}, y: {}", feet, inches, x, y);

    // Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => { setting_value = new_setting_value; },
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        },
    }

    let s = Some(String::from("Hello"));
    if let Some(_) = s {
        println!("Found a string");
    }
    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with ..
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    // Extra Conditionals with Match Guards
    // A match guard is an additional if condition specified after the pattern in a match
    // arm that must also match, along with the pattern matching, for that arm to be chosen.

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    // the preference in a matching guard behaves like this:
    // (4 | 5 | 6) if y => ...
    let bool = true;
    match y {
        4 | 5 | 6 if bool => println!("yes"),
        _ => println!("no"),
    }

    // @ Bindings
    // The at operator (@) lets us create a variable that holds a value at the same
    // time we’re testing that value to see whether it matches a pattern.
    let msg = OneMoreMessage::Hello { id: 5 };
    match msg {
        OneMoreMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("id is {}", id_variable),
        OneMoreMessage::Hello { id: 10..=12 } => println!("id is 10..=12"),
        OneMoreMessage::Hello { id } => println!("id is {}", id),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

enum Colour {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum AnotherMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(Colour),
}

enum OneMoreMessage {
    Hello { id: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
