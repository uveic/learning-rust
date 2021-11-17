use functional;

mod iterator;

fn main() {
    functional::generate_workout(33, 4);

    let x = 4;
    // Closure can capture their environment and access variables from the scope in which
    // they’re defined
    let equal_to_x = |z| z == x;

    assert!(equal_to_x(4));

    // The x value is moved into the closure when the closure is defined, because we added
    // the move keyword. The closure then has ownership of x, and main isn’t allowed to use x
    // anymore in the println! statement.
    let y = vec![1, 2, 3];
    let equal_to_y = move |z| z == y;

    // println!("can't use y here: {:?}", y);

    let a = vec![1, 2, 3];

    assert!(equal_to_y(a));

    let counter = iterator::Counter::new();
    let res: Vec<u32> = counter.map(|x| x + 1).collect();
    println!("Counter: {:?}", res);
}
