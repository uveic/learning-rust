use std::fs;

pub fn day01() -> () {
    let content = fs::read_to_string(String::from("data/day01_elves.txt")).unwrap();
    let elves: Vec<&str> = content.split("\n\n").map(|elf| elf).collect();

    let mut total_by_elf: Vec<i32> = Vec::new();

    for elf in elves {
        let total: i32 = elf
            .split("\n")
            .map(|line| line.parse::<i32>().unwrap_or_else(|_| 0))
            .sum();

        total_by_elf.push(total);
    }

    total_by_elf.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let top_three: i32 = total_by_elf.split_at(3).0.iter().sum();
    let max: i32 = total_by_elf[0];

    println!("############ DAY 1 ############");
    println!("Part 1, result: {:?}", max);
    println!("Part 2, result: {:#?}", top_three);
}
