use std::collections::HashMap;
use std::io;

fn mean_median_mode() {
    println!("===================================");
    println!("Mean, median, mode");
    println!("===================================");

    let mut numbers: Vec<usize> = vec![3, 45, 54, 43, 23, 54, 3, 25, 3, 1, 45, 67, 3, 4, 32, 68, 21, 5, 54];

    println!("Numbers: {:?}", numbers);

    let mut total: usize = 0;
    let length = numbers.len();
    let mut map_count = HashMap::new();

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("Sorted: {:?}", numbers);

    for num in &numbers {
        total += num;

        let count = map_count.entry(num).or_insert(0);
        *count += 1;
    }

    let mut most_repeated: usize = 0;
    let mut max = 0;
    for (k, v) in map_count {
        if v > max {
            max = v;
            most_repeated = *k;
        }
    }

    println!("Total: {}", total);
    println!("Length: {}", length);
    println!("Mean: {}", total / length);
    println!("Median: {}", numbers[length / 2]);
    println!("Mode: {}", most_repeated);
}

fn pig_latin() {
    println!("===================================");
    println!("Pig latin");
    println!("===================================");

    let text = "this is a text to be converted to pig latin y una opción";
    let mut latin_text = String::from("");

    for word in text.split_whitespace() {
        if word.starts_with("a") || word.starts_with("e") || word.starts_with("i") || word.starts_with("o") || word.starts_with("u") {
            let mut w = String::from(word);
            w.push_str("-hay ");
            latin_text.push_str(&w);
        } else {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap();

            let mut new_word = String::from("");
            for c in chars {
                new_word.push(c);
            }

            new_word.push_str("-");
            new_word.push(first_char);
            new_word.push_str("ay ");
            latin_text.push_str(&new_word);
        }
    }

    println!("Original text: {}", text);
    println!("Latin text: {}", latin_text.trim());
}

fn print_menu() -> u8 {
    loop {
        println!("=================================");
        println!("============= MENU ==============");
        println!("=================================");
        println!("1. Add employee");
        println!("2. List employees in department");
        println!("0. Exit");
        println!("");
        println!("Select an option: ");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read option");

        match option.trim().parse() {
            Ok(num) => {
                if num > 2 {
                    continue
                } else {
                    break num
                }
            },
            Err(_) => continue,
        };
    }
}

fn add_employee(map: &mut HashMap<String, Vec<String>>) {
    let name: String = loop {
        println!("What is the name of the employee?");

        let mut employee_name = String::new();

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Failed to read name");

        match employee_name.trim().parse() {
            Ok(n) => break n,
            Err(_) => continue,
        };
    };

    let department: String = loop {
        println!("What is the department for {}?", name);

        let mut department_name = String::new();

        io::stdin()
            .read_line(&mut department_name)
            .expect("Failed to read department");

        match department_name.trim().parse() {
            Ok(n) => break n,
            Err(_) => continue,
        };
    };

    println!("{} was added to {}", &name, department);

    let vec = map.entry(department).or_insert(Vec::new());
    vec.push(name);
}

fn list_employees(map: &HashMap<String, Vec<String>>) {
    let department: String = loop {
        println!("Which department do you want to list employees for?");

        let mut department_name = String::new();

        io::stdin()
            .read_line(&mut department_name)
            .expect("Failed to read department");

        match department_name.trim().parse() {
            Ok(n) => break n,
            Err(_) => continue,
        };
    };

    match map.get(&department) {
        Some(v) => println!("{:?}", v),
        None => println!("Department not found"),
    };
}

fn employees() {
    println!("===================================");
    println!("Employees");
    println!("===================================");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        match print_menu() {
            0 => std::process::exit(0),
            1 => add_employee(&mut map),
            2 => list_employees(&map),
            _ => continue,
        };
    };
}

fn main() {
    mean_median_mode();
    pig_latin();
    employees();
}
