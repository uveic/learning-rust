fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    // This is call 'shadowing': a variable can be declared twice or more with the same name
    // Rust ignores the previous declarations
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "    ";
    // When shadowing the type of the variable can be changed
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // Integer
    // Length	Signed	Unsigned
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize

    // Decimal	98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is {}, the value of y is {}", x, y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    let remainder = 43 % 5;

    println!(
        "Sum: {}, difference: {}, product: {}, quotient: {}, floored: {}, remainder: {}",
        sum,
        difference,
        product,
        quotient,
        floored,
        remainder
    );

    // let t = true;
    // let f: bool = false;

    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';

    let tup: (u32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {}, x: {}, z: {}", y, x, z);
    println!("The value of y is {}, x: {}, z: {}", tup.1, tup.0, tup.2);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _three_threes = [3; 3];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("The first month is: {}, the fifth month is: {}", months[0], months[4]);
}
