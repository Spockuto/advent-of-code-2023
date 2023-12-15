use crate::time_it;
use crate::utils::read_lines;
use colored::Colorize;
use std::collections::BTreeSet;

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let x = a.0.abs_diff(b.0);
    let y = a.1.abs_diff(b.1);

    let max = x.max(y);
    let min = x.min(y);

    2 * min + (max - min)
}

fn problem1(f: &str, expansion_factor: usize) -> u64 {
    let lines = read_lines(f);

    let mut result: u64 = 0;
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_columns: Vec<usize> = vec![];
    let mut column_set: BTreeSet<usize> = BTreeSet::new();

    for (i, line) in lines.iter().enumerate() {
        let g_count: Vec<(usize, usize)> = line
            .match_indices('#')
            .map(|(j, _)| {
                column_set.insert(j);
                (i, j)
            })
            .collect();
        if g_count.is_empty() {
            empty_rows.push(i);
        }
        galaxies.extend(g_count);
    }

    for j in 0..lines.len() {
        if !column_set.contains(&j) {
            empty_columns.push(j);
        }
    }

    let expansion: usize = expansion_factor - 1;
    for (g_i, _) in galaxies.iter_mut() {
        let mut count = expansion;
        for i in empty_rows.as_mut_slice() {
            if g_i > i {
                count += expansion;
            }
        }
        *g_i += count;
    }

    for (_, g_j) in galaxies.iter_mut() {
        let mut count = expansion;
        for j in empty_columns.as_mut_slice() {
            if g_j > j {
                count += expansion;
            }
        }
        *g_j += count;
    }

    for (i, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            let d = distance(*a, *b);
            result += d as u64;
        }
    }

    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/11
    time_it!(11, 1, problem1("files/11.txt", 2));
    time_it!(11, 2, problem1("files/11.txt", 1_000_000));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day11::problem1("files/11_test.txt", 2), 374);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day11::problem1("files/11_test.txt", 100), 8410);
    }
}
