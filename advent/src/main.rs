use std::fs;
use advent::day01::{count_individual_measurements, count_three_window};
use advent::day02::{move_submarine, move_submarine_aim};
use advent::day03::{calculate_power_consumption, calculate_support_rating};

fn main() {
    println!("############ DAY 1 ############");
    let content = fs::read_to_string(String::from("data/day01_measurements.txt")).unwrap();
    let input: Vec<i32> = content.split("\n")
        .map(|line| line.parse::<i32>().unwrap_or_else(|_| 0))
        .filter(|&mass| mass > 0).collect();
    count_individual_measurements(&input);
    count_three_window(&input);

    println!("\n############ DAY 2 ############");
    let content: String = fs::read_to_string(String::from("data/day02_submarine.txt")).unwrap();
    let input: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();
    move_submarine(&input);
    move_submarine_aim(&input);

    println!("\n############ DAY 3 ############");
    let content: String = fs::read_to_string(String::from("data/day03_report.txt")).unwrap();
    let input: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();
    calculate_power_consumption(&input);
    calculate_support_rating(&input);
}
