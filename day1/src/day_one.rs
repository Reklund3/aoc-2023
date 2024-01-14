pub struct DayOne {}

impl DayOne {
    pub fn run(input: &str) -> i64 {
        let mut value: i64 = 0;
        input.lines().for_each(|line| {
            let mut first_number: char = '\n';
            let mut last_number: char = '\n';
            for (i, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    first_number = c;
                    break;
                } else {
                    if line.split_at(i).1.to_lowercase().starts_with("zero") {first_number = '0'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("one") {first_number = '1'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("two") {first_number = '2'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("three") {first_number = '3'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("four") {first_number = '4'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("five") {first_number = '5'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("six") {first_number = '6'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("seven") {first_number = '7'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("eight") {first_number = '8'; break;}
                    else if line.split_at(i).1.to_lowercase().starts_with("nine") {first_number = '9'; break;}
                }
            };
            let line_revers = line.chars().rev().collect::<String>();
            for (i, c) in line_revers.chars().enumerate() {
                if c.is_ascii_digit() {
                    last_number = c;
                    break;
                } else {
                    if line_revers.split_at(i).1.to_lowercase().starts_with("zero".chars().rev().collect::<String>().as_str()) {last_number = '0'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("one".chars().rev().collect::<String>().as_str()) {last_number = '1'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("two".chars().rev().collect::<String>().as_str()) {last_number = '2'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("three".chars().rev().collect::<String>().as_str()) {last_number = '3'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("four".chars().rev().collect::<String>().as_str()) {last_number = '4'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("five".chars().rev().collect::<String>().as_str()) {last_number = '5'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("six".chars().rev().collect::<String>().as_str()) {last_number = '6'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("seven".chars().rev().collect::<String>().as_str()) {last_number = '7'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("eight".chars().rev().collect::<String>().as_str()) {last_number = '8'; break;}
                    else if line_revers.split_at(i).1.to_lowercase().starts_with("nine".chars().rev().collect::<String>().as_str()) {last_number = '9'; break;}
                }
            };
            value += format!("{}{}", first_number, last_number).parse::<i64>().unwrap();
        });
        value
    }
}

#[cfg(test)]
mod test {
    use crate::day1::day_one::DayOne;

    #[test]
    fn testing() {
        let result = DayOne::run("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142)
    }

    #[test]
    fn testing_two() {
        let result = DayOne::run("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");

        assert_eq!(result, 281)
    }

    #[test]
    fn test() {
        let input = "eenin1owt";
        assert_eq!(input.split_at(1).1.starts_with("enin"), true)
    }
}