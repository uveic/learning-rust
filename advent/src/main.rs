use std::fs;
use advent::day01::{count_individual_measurements, count_three_window};
use advent::day02::{move_submarine, move_submarine_aim};

fn main() {
    println!("############ DAY 1 ############");
    let content = fs::read_to_string(String::from("data/measurements.txt")).unwrap();
    let input: Vec<i32> = content.split("\n")
        .map(|line| line.parse::<i32>().unwrap_or_else(|_| 0))
        .filter(|&mass| mass > 0).collect();
    count_individual_measurements(&input);
    count_three_window(&input);

    println!("############ DAY 2 ############");
    let content: String = fs::read_to_string(String::from("data/submarine.txt")).unwrap();
    let input: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();
    move_submarine(&input);
    move_submarine_aim(&input);
}
