


pub struct DayOne {}

impl DayOne {
    pub fn run(input: &str) -> i64 {
        let mut value: i64 = 0;
        input.lines().for_each(|line| {
            let mut first_number: char = '\n';
            let mut last_number: char = '\n';
            for (_, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    first_number = c;
                    break;
                }
            };
            for (_, c) in line.chars().rev().enumerate() {
                if c.is_ascii_digit() {
                    last_number = c;
                    break;
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
}