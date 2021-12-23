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
    pub struct Line{
        numbers: Vec<i32>
    }

    impl Line {
        fn new(line: &str) -> Line {

            println!("{}", line);

            let numbers: Vec<i32> = line.split_whitespace()
                .map(|l| l.trim().parse::<i32>().unwrap())
                .collect();

            if numbers.len() != 5 {
                panic!("Invalid line: {}", line);
            }

            Line { numbers }
        }

        fn new_empty() -> Line {
            Line { numbers: vec![0; 5] }
        }
    }

    #[derive(Debug)]
    pub struct Card {
        lines: Vec<Line>
    }

    impl Card {
        fn new(lines: Vec<Line>) -> Card {
            if lines.len() != 5 {
                panic!("Invalid card: {:?}", lines);
            }

            Card { lines }
        }

        fn new_empty() -> Card {
            Card { lines: vec![Line::new_empty(); 5] }
        }

        fn check_number(&self, number: i32, cards_check: &mut Vec<Card>) -> () {
            for (i, card) in cards_check.iter().enumerate() {
                for (j, line) in card.lines.iter().enumerate() {
                    for (k, number_line) in line.numbers.iter().enumerate() {
                        if number_line == &number {
                            cards_check[i].lines[j].numbers[k] = 1;
                        }
                    }
                }
            }
        }

        fn check_winner(&self, cards_check: &Vec<Card>) -> Option<Line> {
            for card in cards_check {
                for line in &card.lines {
                    if line.numbers.iter().sum::<i32>() == 5 {
                        return Some(line.clone());
                    }
                }
            }

            None
        }
    }

    pub fn read_numbers() -> Vec<i32> {
        let numbers_content: String =
            fs::read_to_string(String::from("data/day04_bingo_numbers.txt")).unwrap();
        let numbers_vec: Vec<&str> = numbers_content
            .split("\n")
            .filter(|l| l.len() > 0)
            .collect();
        let numbers: Vec<i32> = numbers_vec[0]
            .split(",")
            .filter(|l| l.parse::<i32>().is_ok())
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        numbers
    }

    pub fn read_bingo_cards() -> Vec<Card> {
        let bingo_content: String =
            fs::read_to_string(String::from("data/day04_bingo_cards.txt")).unwrap();
        let mut bingo_cards: Vec<Card> = Vec::new();
        let mut lines: Vec<Line> = Vec::new();
        for line in bingo_content.split("\n") {
            if line.len() <= 0 {
                continue;
            }

            lines.push(Line::new(line));

            if lines.len() == 5 {
                bingo_cards.push(Card::new(lines));
                lines = Vec::new();
            }
        }

        bingo_cards
    }

    pub fn get_winner(cards: Vec<Card>, numbers: Vec<i32>) -> () {
        let mut cards_check: Vec<Card> = cards.iter().map(|_| Card::new_empty()).collect();

        for number in numbers {
            for card in &cards {
                card.check_number(number, &mut cards_check);
                match card.check_winner(&cards_check) {
                    Some(line) => {
                        println!("Winner line: {:?}", line);
                        println!("Result: {}", line.numbers.iter().product::<i32>() * number);
                        return;
                    },
                    _ => ()
                }
            }
        }
    }
}
