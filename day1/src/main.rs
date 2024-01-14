mod day_one;

fn main() {
    let input = include_str!("../resources/input.txt");
    println!("{}", day_one::DayOne::run(input));
}
