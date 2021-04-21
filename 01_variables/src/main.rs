fn main() {
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y + 2;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    println!("The value of y is: {}", y);

    let sum = 5 + 10;
    let another_sum: u32 = 5 + 56;

    let difference = 67 - 43;
    let product = 4 * 45;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = '~';
    let heart_eyed_cat = '😻';

    let tup = (500, 5.6, 45);
    let (a, b, c) = tup;
    println!("The value of a is: {}", a);

    let tu: (i32, f64, u8) = (500, 5.6, 1);
    let five_hundred = tu.0;
    let six_point_six = tu.1;
    let one = tu.2;

    let array = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "November", "December"];

    let arr: [i32; 5] = [3, 4, 5, 6, 7];
    let ar = [3; 5]; // Same as let ar = [3, 3, 3, 3, 3];

    let first_month = months[0];
    let first_number = array[0];
}
