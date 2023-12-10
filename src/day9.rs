use std::ops::Neg;

use crate::time_it;
use crate::utils::read_lines;
use num_integer::binomial;

fn find_latest(history: Vec<i64>) -> i64 {
    let mut count: usize = 0;
    let mut result: i64 = 0;

    while count != history.len() {
        result += history
            .iter()
            .rev()
            .take(count + 1)
            .enumerate()
            .map(|(i, value)| value * binomial(count as i64, i.try_into().unwrap()))
            .enumerate()
            .map(|(i, value)| if i % 2 == 0 { value } else { value.neg() })
            .sum::<i64>();
        count += 1;
    }
    result
}

fn find_first(history: Vec<i64>) -> i64 {
    let mut count: usize = 0;
    let mut result: i64 = 0;

    while count != history.len() {
        let sum = history
            .iter()
            .take(count + 1)
            .rev()
            .enumerate()
            .map(|(i, value)| value * binomial(count as i64, i.try_into().unwrap()))
            .enumerate()
            .map(|(i, value)| if i % 2 == 0 { value } else { value.neg() })
            .sum::<i64>();

        if count % 2 == 0 {
            result += sum;
        } else {
            result += sum.neg();
        }
        count += 1;
    }
    result
}

fn problem1(f: &str) -> i64 {
    let lines = read_lines(f);
    let mut result: i64 = 0;

    for line in lines {
        let history: Vec<i64> = line
            .split_terminator(' ')
            .filter_map(|num| match num.parse::<i64>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();
        result += find_latest(history);
    }
    result
}

fn problem2(f: &str) -> i64 {
    let lines = read_lines(f);
    let mut result: i64 = 0;

    for line in lines {
        let history: Vec<i64> = line
            .split_terminator(' ')
            .filter_map(|num| match num.parse::<i64>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();
        result += find_first(history);
    }
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/9
    time_it!("Time", let soln = problem1("files/9.txt"));
    println!("Solution for Day 9 problem 1 is {}", soln);
    time_it!("Time", let soln = problem2("files/9.txt"));
    println!("Solution for Day 9 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day9::problem1("files/9_test.txt"), 114);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day9::problem2("files/9_test.txt"), 2);
    }
}
