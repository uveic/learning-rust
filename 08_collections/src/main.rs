use std::collections::HashMap;

fn main() {

    ////////////////////////////////
    ////////////////////////////////
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

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    ////////////////////////////////
    ////////////////////////////////
    // String

    let mut s = String::new();

    let data = "initial contents";
    let _st = data.to_string();
    let _str = "initial contents".to_string();
    let _stri = String::from("initial contents");

    // UTF-8 encoded
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}, s1 is {}", s2, s1);

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

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    ////////////////////////////////
    ////////////////////////////////
    // HashMap

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Hello"), 36);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores1: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_value, field_name);
    // field_name and field_value are invalid at this point,
    // they were moved and the hash map is the owner

    let team_name = String::from("Blue");
    println!("Score for Blue team: {:?}", scores.get(&team_name));

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    // The value will be replaced
    scores2.insert(String::from("Blue"), 25);
    println!("{:?}", scores2);

    // Only insert a new value if the key doesn't exist yet
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores2);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
