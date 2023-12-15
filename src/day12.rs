use colored::Colorize;
use itertools::Itertools;
use std::iter::once;
use std::vec;

use crate::time_it;
use crate::utils::read_lines;

fn combinations(spring: &str, possible: Vec<usize>) -> usize {
    let spring = format!(".{}", spring.trim_end_matches('.'));
    let spring: Vec<char> = spring.chars().collect();
    let length = spring.len();

    let mut dp = vec![0; length + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for count in possible {
        let mut current = vec![0; length + 1];
        let mut chunk = 0;

        for (i, &c) in spring.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != '#' {
                current[i + 1] += current[i];
            }

            if chunk >= count && spring[i - count] != '#' {
                current[i + 1] += dp[i - count];
            }
        }

        dp = current;
    }

    *dp.last().unwrap()
}

fn problem1(f: &str) -> usize {
    let lines = read_lines(f);
    let mut result: usize = 0;

    for line in lines {
        let split: Vec<_> = line.split(' ').collect();
        let spring = split[0];
        let possible: Vec<usize> = split[1]
            .split(',')
            .filter_map(|x| match x.parse::<usize>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();
        result += combinations(spring, possible);
    }
    result
}

fn problem2(f: &str) -> usize {
    let lines = read_lines(f);
    let mut result: usize = 0;

    for line in lines {
        let split: Vec<_> = line.split(' ').collect();
        let spring = once(split[0]).cycle().take(5).join("?");

        let possible: Vec<usize> = split[1]
            .split(',')
            .filter_map(|x| match x.parse::<usize>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect::<Vec<usize>>()
            .repeat(5);

        result += combinations(spring.as_str(), possible);
    }
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/12
    time_it!(12, 1, problem1("files/12.txt"));
    time_it!(12, 2, problem2("files/12.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day12::problem1("files/12_test.txt"), 21);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day12::problem2("files/12_test.txt"), 525152);
    }
}
