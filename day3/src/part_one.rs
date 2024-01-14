
struct DeadSymbol(char);

pub fn part_one(input: &str) -> u64 {
    let dead_symbol = DeadSymbol('.');
    for i in 0..input.lines().count() {
        let line = input.lines().nth(i).unwrap();
        let (l, r) = get_left_right(line);
    }
    0
}

fn get_left_right(line: &str) -> (&str, &str) {
    let dead_symbol = DeadSymbol('.');
    println!("The line we are working with is: {}", line);
    let x: Vec<usize> = line.char_indices().filter(|(i, c)| !c.is_ascii_digit() && *c != dead_symbol.0).map(|(i, _)| i).collect();
    println!("{:?}", x);
    let mut left_of = "";
    let mut right_of = "";
    if x.len() > 0 {
        for i in 0..x.len() {
            println!("working on index {}", i);
            let (l, r) = line.split_at(x[i]);
            // println!("Left: {}, Right: {}", l, r);
            left_of = l.split_at(l.rfind(|c: char| !c.is_ascii_digit() || c == dead_symbol.0).unwrap() +1).1;
            right_of = r.split_at(1).1.split_once(|c: char| !c.is_ascii_digit() || c == dead_symbol.0).unwrap().0;
        }
    }
    (left_of, right_of)
}


#[test]
fn part_one_test() {
    let result = part_one("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");

    assert_eq!(result, 4361)
}

#[test]
fn get_left_right_test() {
    part_one("..45*617..");
    assert_eq!(0,1)
}