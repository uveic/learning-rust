fn main() {
    let num = 10;
    println!("{} plus 1 is {}", &num, add_one::add_one(&num));
    println!("{} plus 2 is {}", &num, add_two::add_two(&num));
}
