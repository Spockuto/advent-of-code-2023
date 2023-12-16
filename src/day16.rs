use std::collections::HashSet;

use crate::time_it;
use crate::utils::read_lines;
use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Operation {
    Pass(Direction, (Option<usize>, Option<usize>)),
    Turn(Direction, (Option<usize>, Option<usize>)),
    Split(
        Direction,
        (Option<usize>, Option<usize>),
        Direction,
        (Option<usize>, Option<usize>),
    ),
}

fn operation(c: char, d: Direction, x: usize, y: usize, max_x: usize, max_y: usize) -> Operation {
    let down = if x + 1 < max_x {
        (Some(x + 1), Some(y))
    } else {
        (None, Some(y))
    };

    let right = if y + 1 < max_y {
        (Some(x), Some(y + 1))
    } else {
        (Some(x), None)
    };

    let up = (x.checked_sub(1), Some(y));
    let left = (Some(x), y.checked_sub(1));
    if c == '.' {
        return match d {
            Direction::Up => Operation::Pass(Direction::Up, up),
            Direction::Down => Operation::Pass(Direction::Down, down),
            Direction::Right => Operation::Pass(Direction::Right, right),
            Direction::Left => Operation::Pass(Direction::Left, left),
        };
    }

    if c == '|' {
        return match d {
            Direction::Up => Operation::Pass(Direction::Up, up),
            Direction::Down => Operation::Pass(Direction::Down, down),
            Direction::Right | Direction::Left => {
                Operation::Split(Direction::Up, up, Direction::Down, down)
            }
        };
    }

    if c == '-' {
        return match d {
            Direction::Left => Operation::Pass(Direction::Left, left),
            Direction::Right => Operation::Pass(Direction::Right, right),
            Direction::Up | Direction::Down => {
                Operation::Split(Direction::Left, left, Direction::Right, right)
            }
        };
    }

    if c == '/' {
        return match d {
            Direction::Up => Operation::Turn(Direction::Right, right),
            Direction::Down => Operation::Turn(Direction::Left, left),
            Direction::Left => Operation::Turn(Direction::Down, down),
            Direction::Right => Operation::Turn(Direction::Up, up),
        };
    }

    // '\'
    match d {
        Direction::Up => Operation::Turn(Direction::Left, left),
        Direction::Down => Operation::Turn(Direction::Right, right),
        Direction::Left => Operation::Turn(Direction::Up, up),
        Direction::Right => Operation::Turn(Direction::Down, down),
    }
}

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let rows = lines.len();
    let columns = lines[0].len();
    let mut grid: Vec<Vec<HashSet<Direction>>> = vec![vec![HashSet::new(); columns]; rows];
    let mut result: u64 = 0;

    let mut to_process: Vec<(usize, usize)> = vec![(0, 0)];
    grid[0][0].insert(Direction::Right);

    while let Some((x, y)) = to_process.pop() {
        for dir in grid[x][y].clone().iter() {
            match operation(lines[x].chars().nth(y).unwrap(), *dir, x, y, rows, columns) {
                Operation::Pass(d, coord) => {
                    if let (Some(x), Some(y)) = coord {
                        if !grid[x][y].contains(&d) {
                            grid[x][y].insert(d);
                            to_process.push((x, y));
                        }
                    };
                }
                Operation::Turn(d, coord) => {
                    if let (Some(x), Some(y)) = coord {
                        if !grid[x][y].contains(&d) {
                            grid[x][y].insert(d);
                            to_process.push((x, y));
                        }
                    };
                }
                Operation::Split(d1, d1_coord, d2, d2_coord) => {
                    if let (Some(x), Some(y)) = d1_coord {
                        if !grid[x][y].contains(&d1) {
                            grid[x][y].insert(d1);
                            to_process.push((x, y));
                        }
                    };

                    if let (Some(x), Some(y)) = d2_coord {
                        if !grid[x][y].contains(&d2) {
                            grid[x][y].insert(d2);
                            to_process.push((x, y));
                        }
                    };
                }
            }
        }
    }

    for row in grid.iter() {
        for energy in row.iter() {
            if !energy.is_empty() {
                result += 1;
            }
        }
    }
    result
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);
    let rows = lines.len();
    let columns = lines[0].len();
    let mut result: u64 = 0;

    let mut surroundings: Vec<(Direction, usize, usize)> = vec![];

    for i in 0..rows {
        surroundings.push((Direction::Right, i, 0));
        surroundings.push((Direction::Left, i, columns - 1));
    }

    for j in 0..columns {
        surroundings.push((Direction::Down, 0, j));
        surroundings.push((Direction::Up, rows - 1, j));
    }

    for (start, i, j) in surroundings.iter() {
        let mut grid: Vec<Vec<HashSet<Direction>>> = vec![vec![HashSet::new(); columns]; rows];
        let mut to_process: Vec<(usize, usize)> = vec![(*i, *j)];
        grid[*i][*j].insert(*start);

        while let Some((x, y)) = to_process.pop() {
            for dir in grid[x][y].clone().iter() {
                match operation(lines[x].chars().nth(y).unwrap(), *dir, x, y, rows, columns) {
                    Operation::Pass(d, coord) => {
                        if let (Some(x), Some(y)) = coord {
                            if !grid[x][y].contains(&d) {
                                grid[x][y].insert(d);
                                to_process.push((x, y));
                            }
                        };
                    }
                    Operation::Turn(d, coord) => {
                        if let (Some(x), Some(y)) = coord {
                            if !grid[x][y].contains(&d) {
                                grid[x][y].insert(d);
                                to_process.push((x, y));
                            }
                        };
                    }
                    Operation::Split(d1, d1_coord, d2, d2_coord) => {
                        if let (Some(x), Some(y)) = d1_coord {
                            if !grid[x][y].contains(&d1) {
                                grid[x][y].insert(d1);
                                to_process.push((x, y));
                            }
                        };

                        if let (Some(x), Some(y)) = d2_coord {
                            if !grid[x][y].contains(&d2) {
                                grid[x][y].insert(d2);
                                to_process.push((x, y));
                            }
                        };
                    }
                }
            }
        }
        let mut count: u64 = 0;
        for row in grid.iter() {
            for energy in row.iter() {
                if !energy.is_empty() {
                    count += 1;
                }
            }
        }

        if count > result {
            result = count;
        }
    }
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/16
    time_it!(16, 1, problem1("files/16.txt"));
    time_it!(16, 2, problem2("files/16.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day16::problem1("files/16_test.txt"), 46);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day16::problem2("files/16_test.txt"), 51);
    }
}
