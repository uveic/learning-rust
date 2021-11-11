#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    println!("A Dime are {} cents.", value_in_cents(Coin::Dime));
    println!("A Quarter are {} cents.", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) { println!("Moving player {} spaces.", num_spaces); }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // cover any other possibility and name the variable
    }

    let another_u8_value = Some(0u8);
    if let Some(3) = another_u8_value {
        println!("three");
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Equivalent to the match above
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter form {:?}", state),
        _ => count += 1,
    }

    // Equivalent to the match above
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter form {:?}", state);
    // } else {
    //     count += 1;
    // }
}
