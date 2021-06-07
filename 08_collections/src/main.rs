fn main() {
    // Vector
    let _v: Vec<i32> = Vec::new();
    let ve = vec![1, 2, 3, 4, 5];
    let mut vec: Vec<i32> = Vec::new();
    vec.push(5);
    vec.push(6);
    vec.push(7);

    let third: &i32 = &ve[2];
    println!("The third element is {}", third);

    match ve.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // This panics
    // let does_not_exist = &ve[100];
    let _does_not_exist = ve.get(100);

    for i in &vec {
        println!("{}", i);
    }

    for i in &mut vec {
        *i += 50;
    }

    for i in &vec {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    ////////////////////////////////
    ////////////////////////////////
    // String

    let mut s = String::new();

    let data = "initial contents";
    let st = data.to_string();
    let str = "initial contents".to_string();
    let stri = String::from("initial contents");

    // UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5;

    println!("s6 is {}", s6);

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let s10 = s7 + "-" + &s8 + "-" + &s9;
    println!("s10 is {}", s10);

    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");
    let s14 = format!("{}-{}-{}", s11, s12, s13);
    println!("s14 is {}", s14);

    // Not allowed
    // let s15 = String::from("hello");
    // let h = s15[0];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // HashMap

}
