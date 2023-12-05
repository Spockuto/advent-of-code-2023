use std::{collections::HashMap, vec};

use crate::time_it;
use crate::utils::read_lines;

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut data: Vec<Vec<Vec<u64>>> = vec![];
    let mut start: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() || i == lines.len() - 1 {
            data.push(
                lines[start..=i]
                    .to_vec()
                    .iter()
                    .filter_map(|s| {
                        let nums: Vec<u64> = s
                            .split_terminator(' ')
                            .filter_map(|num| match num.parse::<u64>() {
                                Ok(value) => Some(value),
                                Err(_) => None,
                            })
                            .collect();

                        if nums.is_empty() {
                            return None;
                        }
                        Some(nums)
                    })
                    .collect(),
            );
            start = i + 1;
        }
    }

    let mut seeds: Vec<u64> = data.get(0).unwrap().iter().next().unwrap().to_vec();

    data.remove(0);

    for maps in data {
        let mut visited: HashMap<usize, bool> = HashMap::new();
        for i in 0..seeds.len() {
            visited.insert(i, false);
        }
        for (i, seed) in seeds.clone().iter().enumerate() {
            for map in maps.clone() {
                if (map[1]..=(map[1] + map[2])).contains(seed) && !visited.get(&i).unwrap() {
                    seeds[i] = seed + map[0] - map[1];
                    visited.insert(i, true);
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn problem2(f: &str) -> i64 {
    let lines = read_lines(f);
    let mut data: Vec<Vec<Vec<i64>>> = vec![];
    let mut start: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() || i == lines.len() - 1 {
            data.push(
                lines[start..=i]
                    .to_vec()
                    .iter()
                    .filter_map(|s| {
                        let nums: Vec<i64> = s
                            .split_terminator(' ')
                            .filter_map(|num| match num.parse::<i64>() {
                                Ok(value) => Some(value),
                                Err(_) => None,
                            })
                            .collect();

                        if nums.is_empty() {
                            return None;
                        }
                        Some(nums)
                    })
                    .collect(),
            );
            start = i + 1;
        }
    }

    let copy = data.clone();
    let mut seeds: Vec<_> = copy
        .get(0)
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .chunks(2)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<i64>>>();

    data.remove(0);

    for maps in data {
        let mut visited: HashMap<usize, bool> = HashMap::new();
        for i in 0..seeds.len() {
            visited.insert(i, false);
        }
        loop {
            let mut new_seeds: Vec<Vec<i64>> = vec![];
            for (i, seed) in seeds.clone().iter_mut().enumerate() {
                for map in maps.clone() {
                    let start_x = map[1];
                    let end_x = map[1] + map[2];
                    let start_y = seed[0];
                    let end_y = seed[0] + seed[1] - 1;

                    if start_x <= end_y && end_x >= start_y && !visited.get(&i).unwrap() {
                        let remaining = (start_x - start_y, end_x - end_y);
                        seeds[i][0] = seed[0] + map[0] - map[1];
                        visited.insert(i, true);

                        if remaining.0 > 0 {
                            seeds[i][0] += remaining.0;
                            seeds[i][1] = seed[1] - remaining.0 ;
                            new_seeds.push(vec![start_y, remaining.0]);
                        }

                        if remaining.1 < 0 {
                            seeds[i][1] = seed[1] - (end_y - end_x);
                            new_seeds.push(vec![end_x + 1, end_y - end_x]);
                        }
                    }
                }
            }
            if new_seeds.is_empty() {
                break;
            }
            for i in seeds.len()..(new_seeds.len() + seeds.len()) {
                visited.insert(i, false);
            }
            seeds.extend(new_seeds);
        }
    }

    seeds.iter().min_by_key(|x| x[0]).unwrap()[0]
}

pub fn solve() {
    // https://adventofcode.com/2023/day/5
    time_it!("Time", let soln = problem1("files/5.txt"));
    println!("Solution for Day 5 problem 1 is {}", soln);
    time_it!("Time", let soln = problem2("files/5.txt"));
    println!("Solution for Day 5 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day5::problem1("files/5_test.txt"), 35);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day5::problem2("files/5_test.txt"), 46);
    }
}
