use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut count = 1;

    loop {
        println!("==========================");
        println!("Attempt: {}", count);
        println!("Please, input your guess: ");
        count += 1;

        let mut guess_str = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: Guess = match guess_str.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        // Not needed anymore because this check is make in Guess::new()
        // It panics instead of giving the user a new chance though
        //
        // if guess.value() < 1 || guess.value() > 100 {
        //     println!("The secret number will be between 1 and 100");
        //     continue;
        // }

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
