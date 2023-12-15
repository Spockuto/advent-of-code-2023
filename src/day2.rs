use crate::time_it;
use crate::utils::read_lines;
use colored::Colorize;

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut sum: u64 = 0;
    const RED: i64 = 12;
    const GREEN: i64 = 13;
    const BLUE: i64 = 14;

    // each game
    for (game, line) in lines.iter().enumerate() {
        let mut fail: bool = false;
        let split: Vec<_> = line.split_terminator(':').collect();
        for set in split[1].split(';') {
            for draw in set
                .split_terminator(&[',', ' '])
                .filter(|&str| !str.is_empty())
                .collect::<Vec<&str>>()
                .chunks(2)
            {
                let color = draw[1];
                let value: i64 = draw[0].parse().unwrap();

                if (color == "red" && value > RED)
                    || (color == "green" && value > GREEN)
                    || (color == "blue" && value > BLUE)
                {
                    fail = true;
                }
            }
        }
        if !fail {
            sum = sum + game as u64 + 1;
        }
    }
    sum
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut sum: u64 = 0;

    // each game
    for (_, line) in lines.iter().enumerate() {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;

        let split: Vec<_> = line.split_terminator(':').collect();
        for set in split[1].split(';') {
            for draw in set
                .split_terminator(&[',', ' '])
                .filter(|&str| !str.is_empty())
                .collect::<Vec<&str>>()
                .chunks(2)
            {
                let color = draw[1];
                let value: u64 = draw[0].parse::<u64>().unwrap();

                if color == "red" && value > red {
                    red = value;
                }

                if color == "green" && value > green {
                    green = value;
                }

                if color == "blue" && value > blue {
                    blue = value;
                }
            }
        }
        sum += red * green * blue;
    }
    sum
}

pub fn solve() {
    // https://adventofcode.com/2023/day/2
    time_it!(2, 1, problem1("files/2.txt"));
    time_it!(2, 2, problem2("files/2.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day2::problem1("files/2_test.txt"), 8);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day2::problem2("files/2_test.txt"), 2286);
    }
}
