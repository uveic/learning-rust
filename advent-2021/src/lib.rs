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
        let mut oxygen_content = content.clone();
        let mut scrubber_content = content.clone();

        let oxygen_content_res = content_filter(&mut oxygen_content, true, None);
        println!("\n\n");
        let scrubber_content_res = content_filter(&mut scrubber_content, false, None);

        println!("Oxygen content: {:?}", oxygen_content_res);
        println!("Scrubber content: {:?}", scrubber_content_res);

        let oxygen_rate = isize::from_str_radix(&oxygen_content_res[0], 2).unwrap();
        let scrubber_rate = isize::from_str_radix(&scrubber_content_res[0], 2).unwrap();

        println!("Oxygen rate: {}", oxygen_rate);
        println!("Scrubber rate: {}", scrubber_rate);
        println!("Support rating: {}", oxygen_rate * scrubber_rate);
    }

    fn content_filter<'a>(
        content: &'a mut Vec<&'a str>,
        min: bool,
        index: Option<usize>,
    ) -> &'a mut Vec<&'a str> {
        let len = content[0].len();
        let i: usize = index.unwrap_or(0);
        let bits_count: HashMap<(i32, char), i32> = get_bit_count_per_position(content);

        if i >= len {
            return content;
        }

        let zero_count = bits_count.get(&(i as i32, '0')).unwrap();
        let one_count = bits_count.get(&(i as i32, '1')).unwrap();

        let filter_bit: char = if zero_count > one_count { '0' } else { '1' };

        println!(
            "Index: {:2}, len: {}, content length: {:4}, '0' count: {:4}, '1' count: {:4}, filter_bit: {}",
            i,
            len,
            content.len(),
            zero_count,
            one_count,
            filter_bit,
        );

        content.retain(|x| {
            let current_char: char = x.chars().nth(i).unwrap();
            if min {
                current_char == filter_bit
            } else {
                current_char != filter_bit
            }
        });

        if content.len() == 1 {
            return content;
        }

        content_filter(content, min, Some(i + 1))
    }
}

pub mod day04 {
    use std::fs;

    #[derive(Debug, Clone)]
    pub struct Line {
        card: usize,
        numbers: Vec<usize>,
    }

    impl Line {
        fn new(card: usize, line: &str) -> Line {
            let numbers: Vec<usize> = line
                .split_whitespace()
                .map(|l| l.trim().parse::<usize>().unwrap())
                .collect();

            if numbers.len() != 5 {
                panic!("Invalid line: {}", line);
            }

            Line { card, numbers }
        }

        fn new_empty(card: usize) -> Line {
            Line {
                card,
                numbers: vec![0; 5],
            }
        }

        fn check_number(
            &self,
            bingo_number: &usize,
            matches: &mut Vec<(usize, &Vec<usize>)>,
        ) -> Option<&Vec<usize>> {
            let found: Option<&usize> = self.numbers.iter().find(|n| *n == bingo_number);

            if found.is_some() {
                println!(
                    "Found: {}, line: {:?}, matches: {:?}",
                    bingo_number,
                    self,
                    matches.get(self.card)
                );

                return match matches.iter().find(|c| c.0 == self.card) {
                    Some(&c) => {
                        matches.remove(c);
                        c.1.push(*bingo_number);

                        Some(c.1)
                    }
                    None => {
                        let mut numbers = Vec::new();
                        numbers.push(*bingo_number);
                        matches.push((self.card, &mut numbers));

                        Some(&numbers)
                    }
                };
            }

            None
        }

        fn check_winner(&self, line: &Vec<usize>, number: &usize) -> bool {
            if line.len() == 5 {
                println!("Winner: {:?}", line);

                let mut result: usize = 1;
                for n in line {
                    result *= n;
                }

                println!("Result: {}", result * number);
                return true;
            }

            false
        }
    }

    pub fn read_numbers() -> Vec<usize> {
        let numbers_content: String =
            fs::read_to_string(String::from("data/day04_bingo_numbers.txt")).unwrap();
        let numbers_vec: Vec<&str> = numbers_content
            .split("\n")
            .filter(|l| l.len() > 0)
            .collect();
        let numbers: Vec<usize> = numbers_vec[0]
            .split(",")
            .filter(|l| l.parse::<usize>().is_ok())
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        numbers
    }

    pub fn read_lines() -> Vec<Line> {
        let bingo_content: String =
            fs::read_to_string(String::from("data/day04_bingo_lines.txt")).unwrap();
        let mut lines: Vec<Line> = Vec::new();
        let mut count: usize = 0;
        for line in bingo_content.split("\n") {
            if line.len() <= 0 {
                continue;
            }

            lines.push(Line::new(count, line));

            if lines.len() % 5 == 0 {
                count += 1;
            }
        }

        lines
    }

    pub fn get_winner(lines: Vec<Line>, numbers: Vec<usize>) -> () {
        let mut matches: Vec<(usize, &mut Vec<usize>)> = Vec::new();

        for number in numbers {
            for line in &lines {
                match line.check_number(&number, &mut matches) {
                    Some(numbers) => {
                        if line.check_winner(&numbers, &number) {
                            return;
                        }
                    }
                    None => {}
                }
            }
        }
    }
}
