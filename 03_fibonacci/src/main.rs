use std::io;

fn main() {
    let total: u64 = loop {
        println!("How many fibonacci numbers do you fancy?");

        let mut total = String::new();

        io::stdin()
            .read_line(&mut total)
            .expect("Failed to read number");

        match total.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let mut counter: u64 = 2;
    let mut previous_fibonacci: u64 = 1;
    let mut current_fibonacci: u64 = 1;
    let mut temp: u64;

    if total > 2 {
        print!("Fibonacci series: {} {} ", previous_fibonacci, current_fibonacci);
    }

    while counter < total {
        temp = previous_fibonacci;
        previous_fibonacci = current_fibonacci;
        current_fibonacci = previous_fibonacci + temp;

        print!("{} ", current_fibonacci);

        counter += 1;
    }

    println!("\nThis is the {}th fibonacci number: {}", total, current_fibonacci);
}
