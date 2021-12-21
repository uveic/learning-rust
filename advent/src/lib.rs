pub mod day01 {
    pub fn count_individual_measurements(content: &Vec<i32>) -> () {
        let mut count = 0;
        let mut prev: Option<i32> = None;

        for n in content {
            match prev {
                Some(p) => if n > &p { count += 1 },
                None => (),
            }
            prev = Some(*n);
        }

        println!("Count window of three: {}", count);
    }

    pub fn count_three_window(content: &Vec<i32>) -> () {
        let mut count = 0;
        let mut index = 0;
        let mut prev: Option<i32> = None;
        let last = content.len() - 1;

        for _ in content {
            if index + 2 > last {
                break;
            }

            let sum: i32 = content[index] + content[index + 1] + content[index + 2];
            match prev {
                Some(p) => if sum > p { count += 1 },
                None => (),
            }

            index += 1;
            prev = Some(sum);
        }

        println!("Count individual measurements: {}", count);
    }
}

pub mod day02 {
    pub fn move_submarine(content: &Vec<&str>) -> () {
        let mut horizontal: i32 = 0;
        let mut depth: i32 = 0;

        for item in content {
            if item.starts_with("forward") {
                horizontal += item.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            } else if item.starts_with("up") {
                depth -= item.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            } else if item.starts_with("down") {
                depth += item.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            }
        }

        println!("Final position, horizontal: {}, depth: {}", horizontal, depth);
        println!("Result moving submarine: {}", horizontal * depth);
    }

    pub fn move_submarine_aim(content: &Vec<&str>) -> () {
        let mut aim: i64 = 0;
        let mut horizontal: i64 = 0;
        let mut depth: i64 = 0;

        for item in content {
            if item.starts_with("forward") {
                let unit = item.split(" ").nth(1).unwrap().parse::<i64>().unwrap();
                horizontal += unit;
                depth += aim * unit;
            } else if item.starts_with("up") {
                aim -= item.split(" ").nth(1).unwrap().parse::<i64>().unwrap();
            } else if item.starts_with("down") {
                aim += item.split(" ").nth(1).unwrap().parse::<i64>().unwrap();
            }
        }

        println!("Final position, horizontal: {}, depth: {}, aim: {}", horizontal, depth, aim);
        println!("Result moving submarine: {}", horizontal * depth);
    }
}
