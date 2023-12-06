use std::iter::zip;
use std::ops::Mul;

use crate::time_it;
use crate::utils::read_lines;

fn roots(t: i64, d: i64) -> (i64, i64) {
    let t_f = t.mul(-1) as f64;
    let d_f = d as f64;
    let det = (t_f * t_f - 4.0 * d_f).sqrt();
    let soln1 = (t_f * -1.0 + det) / 2.0;
    let soln2 = (t_f * -1.0 - det) / 2.0;

    if soln1 > soln2 {
        return (soln1.floor() as i64, soln2.ceil() as i64);
    }
    (soln2.floor() as i64, soln1.ceil() as i64)
}

fn maxima(t: i64) -> (i64, i64) {
    if t % 2 == 0 {
        return (t / 2, t / 2);
    }
    ((t - 1) / 2, (t + 1) / 2)
}

fn range(t: i64, d: i64) -> i64 {
    let mut count = 0;
    let root = roots(t, d);
    let max_t = maxima(t);

    if (root.0 * root.1 - d) == 0 {
        count += -2;
    }

    if max_t.0 == max_t.1 {
        count += root.0 - root.1 + 1;
    } else {
        count += root.0 - max_t.1 + max_t.0 - root.1 + 2;
    }

    count
}

fn problem1(f: &str) -> i64 {
    let lines = read_lines(f);
    let mut result: i64 = 1;

    let time: Vec<i64> = lines[0]
        .split_terminator(' ')
        .filter_map(|num| match num.parse::<i64>() {
            Ok(value) => Some(value),
            Err(_) => None,
        })
        .collect();
    let distance: Vec<i64> = lines[1]
        .split_terminator(' ')
        .filter_map(|num| match num.parse::<i64>() {
            Ok(value) => Some(value),
            Err(_) => None,
        })
        .collect();

    for (total, max) in zip(time, distance) {
        result *= range(total, max)
    }

    result
}

fn problem2(f: &str) -> i64 {
    let lines = read_lines(f);
    let mut result: i64 = 0;

    let time: i64 = lines[0]
        .split_terminator(' ')
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    let distance: i64 = lines[1]
        .split_terminator(' ')
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    for t in 0..=time {
        if t * (time - t) > distance {
            result += 1;
        }
    }

    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/3
    time_it!("Time", let soln = problem1("files/6.txt"));
    println!("Solution for Day 6 problem 1 is {}", soln);
    time_it!("Time", let soln = problem2("files/6.txt"));
    println!("Solution for Day 6 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day6::problem1("files/6_test.txt"), 288);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day6::problem2("files/6_test.txt"), 71503);
    }
}
