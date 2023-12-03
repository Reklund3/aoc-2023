

struct Blue(i32);
struct Green(i32);
struct Red(i32);

struct Game<'a> {
    num: i32,
    rounds: Vec<&'a str>
}

pub fn run(input: &str) -> i64 {
    let red = Red(12);
    let green = Green(13);
    let blue = Blue(14);
    let mut valid_games: i64 = 0;
    input.lines().for_each(|line| {
        let (game, game_outputs) = line.rsplit_once(": ").unwrap();
        let rounds: Vec<&str> = game_outputs.split_terminator("; ").collect();
        let mut valid_game: i64 = game.split_once(" ").unwrap().1.parse::<i64>().unwrap();
        'a: for round in rounds {
            let round_results: Vec<&str> = round.split_terminator(", ").collect();
            for r in round_results {
                let (count, color) = r.split_once(" ").unwrap();
                let num = count.parse::<i32>().unwrap();
                if color == "red" && num > red.0 {
                    valid_game = 0;
                    break 'a;
                } else if color == "green" && num > green.0 {
                    valid_game = 0;
                    break 'a;
                } else if color == "blue" && num > blue.0 {
                    valid_game = 0;
                    break 'a;
                }
            }
        }
        valid_games += valid_game;
    });
    valid_games
}

pub fn run_two(input: &str) -> i32 {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let (game, game_outputs) = line.rsplit_once(": ").unwrap();
        let rounds: Vec<&str> = game_outputs.split_terminator("; ").collect();
        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;
        for round in rounds {
            let round_results: Vec<&str> = round.split_terminator(", ").collect();
            for r in round_results {
                let (count, color) = r.split_once(" ").unwrap();
                let num = count.parse::<i32>().unwrap();
                if color == "red" && num > red_min {
                    red_min = num;
                } else if color == "green" && num > green_min {
                    green_min = num;
                } else if color == "blue" && num > blue_min {
                    blue_min = num;
                }
            }
        }
        sum += red_min * blue_min * green_min;
    });
    sum
}

#[cfg(test)]
mod test {
    use crate::day2::day_two::run;
    use crate::day2::day_two::run_two;

    #[test]
    fn should_run() {
        let result = run("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result,8)
    }

    #[test]
    fn should_run_two() {
        let result = run_two("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286)
    }
}