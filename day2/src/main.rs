use std::fs;

mod day_two;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("{}", day_two::run_two(input));
}
