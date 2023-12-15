use crate::time_it;
use crate::utils::read_lines;
use colored::Colorize;

type PartPosition = Vec<(u64, (usize, usize), (usize, usize))>;
fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);

    let mut pos: Vec<(usize, usize)> = Vec::new();
    let mut symbols: Vec<char> = vec!['.'];

    let max_x = lines[0].len();
    let max_y = lines.len();

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                symbols.push(char);
                pos.push((i, j));
            }
        }
    }

    let mut num_pos: PartPosition = vec![];
    for (i, line) in lines.iter().enumerate() {
        let split: Vec<_> = line.split_terminator(symbols.as_slice()).collect();
        let mut loc: usize = 0;
        for each in split {
            let num_str = each.parse::<u64>();
            match num_str {
                Ok(value) => {
                    num_pos.push((value, (i, loc), (i, loc + each.len() - 1)));
                    loc = loc + each.len() + 1;
                }
                Err(_) => {
                    loc += 1;
                }
            }
        }
    }

    let mut result: u64 = 0;
    for (number, (start_x, start_y), (end_x, end_y)) in num_pos {
        let mut valid: bool = false;
        for (x, y) in pos.iter() {
            if x >= &start_x.saturating_sub(1)
                && x <= &((end_x + 1).min(max_y))
                && y >= &start_y.saturating_sub(1)
                && y <= &((end_y + 1).min(max_x))
            {
                valid = true;
            }
        }
        if valid {
            result += number;
        }
    }
    result
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);

    let mut pos: Vec<(usize, usize)> = Vec::new();
    let mut symbols: Vec<char> = vec!['.'];

    let max_x = lines[0].len();
    let max_y = lines.len();

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                symbols.push(char);
            }
            if char == '*' {
                pos.push((i, j));
            }
        }
    }

    let mut num_pos: PartPosition = vec![];
    for (i, line) in lines.iter().enumerate() {
        let split: Vec<_> = line.split_terminator(symbols.as_slice()).collect();
        let mut loc: usize = 0;
        for each in split {
            let num_str = each.parse::<u64>();
            match num_str {
                Ok(value) => {
                    num_pos.push((value, (i, loc), (i, loc + each.len() - 1)));
                    loc = loc + each.len() + 1;
                }
                Err(_) => {
                    loc += 1;
                }
            }
        }
    }

    let mut result: u64 = 0;
    for (x, y) in pos.iter() {
        let mut gears = vec![];
        for (number, (start_x, start_y), (end_x, end_y)) in num_pos.iter() {
            if x >= &start_x.saturating_sub(1)
                && x <= &((end_x + 1).min(max_y))
                && y >= &start_y.saturating_sub(1)
                && y <= &((end_y + 1).min(max_x))
            {
                gears.push(number);
            }
        }
        if gears.len() == 2 {
            result += gears[0] * gears[1];
        }
    }
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/3
    time_it!(3, 1, problem1("files/3.txt"));
    time_it!(3, 2, problem2("files/3.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day3::problem1("files/3_test.txt"), 4361);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day3::problem2("files/3_test.txt"), 467835);
    }
}
