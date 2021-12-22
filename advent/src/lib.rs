pub mod day01 {
    pub fn count_individual_measurements(content: &Vec<i32>) -> () {
        let mut count = 0;
        let mut prev: Option<i32> = None;

        for n in content {
            match prev {
                Some(p) => {
                    if n > &p {
                        count += 1
                    }
                }
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
                Some(p) => {
                    if sum > p {
                        count += 1
                    }
                }
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

        println!(
            "Final position, horizontal: {}, depth: {}",
            horizontal, depth
        );
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

        println!(
            "Final position, horizontal: {}, depth: {}, aim: {}",
            horizontal, depth, aim
        );
        println!("Result moving submarine: {}", horizontal * depth);
    }
}

pub mod day03 {
    use std::collections::HashMap;

    fn get_bit_count_per_position(content: &Vec<&str>) -> HashMap<(i32, char), i32> {
        let mut bits_count: HashMap<(i32, char), i32> = HashMap::new();

        for item in content {
            for (i, c) in item.chars().enumerate() {
                let entry: (i32, char) = (i as i32, c);
                let count = bits_count.entry(entry).or_insert(0);
                *count += 1;
            }
        }

        bits_count
    }

    pub fn calculate_power_consumption(content: &Vec<&str>) -> () {
        let bits_count = get_bit_count_per_position(content);

        let mut max: HashMap<i32, char> = HashMap::new();
        let mut min: HashMap<i32, char> = HashMap::new();
        let end: i32 = content[0].len() as i32;
        for i in 0..end {
            let zero = bits_count.get(&(i, '0')).unwrap();
            let one = bits_count.get(&(i, '1')).unwrap();

            max.insert(i, if zero > one { '0' } else { '1' });
            min.insert(i, if zero < one { '0' } else { '1' });
        }

        let mut max_vec: Vec<(&i32, &char)> = max.iter().collect();
        max_vec.sort_by(|a, b| a.0.cmp(&b.0));

        let gamma_rate: String = max_vec.iter().map(|x| x.1.to_string()).collect();
        let gamma_rate_decimal = isize::from_str_radix(&gamma_rate.to_string(), 2).unwrap();

        println!("Gamma rate: {}, {}", gamma_rate, gamma_rate_decimal);

        let mut min_vec: Vec<(&i32, &char)> = min.iter().collect();
        min_vec.sort_by(|a, b| a.0.cmp(&b.0));

        let epsilon_rate: String = min_vec.iter().map(|x| x.1.to_string()).collect();
        let epsilon_rate_decimal = isize::from_str_radix(&epsilon_rate.to_string(), 2).unwrap();

        println!("Epsilon rate: {}, {}", epsilon_rate, epsilon_rate_decimal);
        println!(
            "Power consumption: {}",
            gamma_rate_decimal * epsilon_rate_decimal
        );
    }

    pub fn calculate_support_rating(content: &Vec<&str>) -> () {
        let bits_count = get_bit_count_per_position(content);

        let end: i32 = content[0].len() as i32;
        let mut oxygen_content = content.clone();
        let mut scrubber_content = content.clone();

        for i in 0..end {
            let index: usize = i as usize;
            let zero_count = bits_count.get(&(i, '0')).unwrap();
            let one_count = bits_count.get(&(i, '1')).unwrap();

            let oxygen_bit: char = if zero_count > one_count { '0' } else { '1' };
            let scrubber_bit: char = if zero_count < one_count { '0' } else { '1' };

            println!("Oxygen len: {}", oxygen_content.len());

            for j in 0..oxygen_content.len() {
                if oxygen_content.len() == 1 {
                    continue;
                }

                let current_char = oxygen_content[j].chars().nth(index).unwrap();
                if current_char != oxygen_bit {
                    oxygen_content.remove(j);
                }
            }

            for k in 0..scrubber_content.len() {
                if scrubber_content.len() == 1 {
                    continue;
                }

                let current_char = scrubber_content[k].chars().nth(index).unwrap();
                if current_char != scrubber_bit {
                    scrubber_content.remove(k);
                }
            }
        }

        println!("Oxygen content: {:?}", oxygen_content);
        println!("Scrubber content: {:?}", scrubber_content);
    }

    fn content_filer(content: &mut Vec<&str>, filter_bit: char, index: usize) -> &mut Vec<&str> {
        content.retain(|x| {
            let current_char = x.chars().nth(index).unwrap();
            current_char != filter_bit
        });

        content
    }
}
