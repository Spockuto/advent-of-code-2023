use crate::time_it;
use crate::utils::read_lines;

fn find_load_once(mirror: Vec<Vec<u8>>) -> u64 {
    let length = mirror.len();
    let mut rows: Vec<u64> = vec![0; mirror[0].len()];
    let mut last_seen: Vec<u32> = vec![0; mirror[0].len()];

    for (i, line) in mirror.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 1 {
                rows[last_seen[j] as usize] += 1;
                last_seen[j] += 1;
            }

            if *c == 2 {
                last_seen[j] = i as u32 + 1;
            }
        }
    }
    rows.iter()
        .enumerate()
        .map(|(i, x)| *x * (length - i) as u64)
        .sum::<u64>()
}

fn find_load(mirror: Vec<Vec<u8>>, count: usize) -> u64 {
    let mut grid = mirror.clone();
    let rows = mirror.len();
    let columns = mirror[0].len();

    let mut check: Vec<Vec<Vec<u8>>> = vec![];

    for _ in 0..count {
        let mut last_seen: Vec<usize> = vec![0; columns];
        let mut spin: Vec<Vec<u8>> = vec![vec![0; columns]; rows];

        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == 1 {
                    spin[last_seen[j]][j] = 1;
                    last_seen[j] += 1;
                }

                if mirror[i][j] == 2 {
                    last_seen[j] = i + 1;
                }
            }
        }

        last_seen = vec![0; rows];
        grid = spin;
        spin = vec![vec![0; columns]; rows];

        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == 1 {
                    spin[i][last_seen[i]] = 1;
                    last_seen[i] += 1;
                }

                if mirror[i][j] == 2 {
                    last_seen[i] = j + 1;
                }
            }
        }

        last_seen = vec![rows - 1; columns];
        grid = spin;
        spin = vec![vec![0; columns]; rows];

        for (i, line) in grid.iter().rev().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == 1 {
                    spin[last_seen[j]][j] = 1;
                    last_seen[j] = last_seen[j].saturating_sub(1);
                }

                if mirror[rows - 1 - i][j] == 2 {
                    last_seen[j] = rows.saturating_sub(2 + i);
                }
            }
        }

        last_seen = vec![columns - 1; rows];
        grid = spin;
        spin = vec![vec![0; columns]; rows];

        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().rev().enumerate() {
                if *c == 1 {
                    spin[i][last_seen[i]] = 1;
                    last_seen[i] = last_seen[i].saturating_sub(1);
                }

                if mirror[i][columns - 1 - j] == 2 {
                    last_seen[i] = columns.saturating_sub(j + 2);
                }
            }
        }
        grid = spin;

        if check.contains(&grid) {
            let index = check.iter().position(|r| *r == grid).unwrap();
            let range = check.len() - index;
            let to_check = (count - index) % range;
            let value = check.get(to_check + index - 1).unwrap();
            return value
                .iter()
                .enumerate()
                .map(|(i, x)| x.iter().sum::<u8>() as u64 * (rows - i) as u64)
                .sum::<u64>();
        } else {
            check.push(grid.clone());
        }
    }

    0
}

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut grid: Vec<Vec<u8>> = vec![vec![0; lines[0].len()]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'O' {
                grid[i][j] = 1;
            } else if c == '#' {
                grid[i][j] = 2;
            }
        }
    }
    find_load_once(grid)
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);

    let mut grid: Vec<Vec<u8>> = vec![vec![0; lines[0].len()]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'O' {
                grid[i][j] = 1;
            } else if c == '#' {
                grid[i][j] = 2;
            }
        }
    }

    find_load(grid, 1_000_000_000)
}

pub fn solve() {
    // https://adventofcode.com/2023/day/14
    time_it!("Time", let soln = problem1("files/14.txt"));
    println!("Solution for Day 14 problem 1 is {}", soln);
    time_it!("Time", let soln = problem2("files/14.txt"));
    println!("Solution for Day 14 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day14::problem1("files/14_test.txt"), 136);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day14::problem2("files/14_test.txt"), 64);
    }
}
