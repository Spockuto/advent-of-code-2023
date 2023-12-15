use colored::Colorize;
use geo::{point, Polygon};
use geo::{Contains, LineString};
use std::collections::VecDeque;

use std::vec;

use crate::time_it;
use crate::utils::read_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pipe {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
    block: bool,
}

const V: Pipe = Pipe {
    up: 1,
    down: 1,
    left: 0,
    right: 0,
    block: false,
};
const H: Pipe = Pipe {
    up: 0,
    down: 0,
    left: 1,
    right: 1,
    block: false,
};
const L: Pipe = Pipe {
    up: 1,
    down: 0,
    left: 0,
    right: 1,
    block: false,
};
const J: Pipe = Pipe {
    up: 1,
    down: 0,
    left: 1,
    right: 0,
    block: false,
};
const W: Pipe = Pipe {
    up: 0,
    down: 1,
    left: 1,
    right: 0,
    block: false,
}; // 7
const F: Pipe = Pipe {
    up: 0,
    down: 1,
    left: 0,
    right: 1,
    block: false,
};
const B: Pipe = Pipe {
    up: 0,
    down: 0,
    left: 0,
    right: 0,
    block: true,
};
const S: Pipe = Pipe {
    up: 1,
    down: 1,
    left: 1,
    right: 1,
    block: false,
};

fn max(a: i32, b: i32, c: i32, d: i32) -> i32 {
    *[a, b, c, d].as_slice().iter().max().unwrap()
}

fn enclosed(start_i: usize, start_j: usize, grid: &Vec<Vec<Pipe>>) -> i32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut polygon_points: Vec<(f32, f32)> = vec![];

    queue.push_front((start_i, start_j));
    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        if !visited[i][j] {
            if grid[i][j] != V && grid[i][j] != H {
                polygon_points.push((i as f32, j as f32));
            }
            let pipe = grid[i][j];
            if !pipe.block {
                if pipe.up == 1 && grid[i.saturating_sub(1)][j].down == 1 {
                    queue.push_front((i.saturating_sub(1), j));
                }

                if pipe.down == 1 && grid[(i + 1).min(grid.len() - 1)][j].up == 1 {
                    queue.push_front(((i + 1).min(grid.len() - 1), j));
                }

                if pipe.left == 1 && grid[i][j.saturating_sub(1)].right == 1 {
                    queue.push_front((i, j.saturating_sub(1)));
                }

                if pipe.right == 1 && grid[i][(j + 1).min(grid[0].len() - 1)].left == 1 {
                    queue.push_front((i, (j + 1).min(grid[0].len() - 1)));
                }
            }
            visited[i][j] = true;
        }
    }
    let polygon = Polygon::new(LineString::from(polygon_points), vec![]);
    let mut result: i32 = 0;
    for (i, row) in visited.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let pair: (f32, f32) = ((i as f32), (j as f32));
            if !*val && polygon.contains(&point!(pair)) {
                result += 1;
            }
        }
    }
    result
}

fn max_path(start_i: usize, start_j: usize, grid: &Vec<Vec<Pipe>>) -> i32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut dp: Vec<Vec<i32>> = vec![vec![0; grid[0].len() + 2]; grid.len() + 2];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back((start_i, start_j));

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        if !visited[i][j] {
            let pipe = grid[i][j];
            if !pipe.block {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(max(
                    pipe.up + dp[i][j + 1],
                    pipe.down + dp[i + 2][j + 1],
                    pipe.left + dp[i + 1][j],
                    pipe.right + dp[i + 1][j + 2],
                ));

                if pipe.up == 1 && grid[i.saturating_sub(1)][j].down == 1 {
                    queue.push_back((i.saturating_sub(1), j));
                }

                if pipe.down == 1 && grid[(i + 1).min(grid.len() - 1)][j].up == 1 {
                    queue.push_back(((i + 1).min(grid.len() - 1), j));
                }

                if pipe.left == 1 && grid[i][j.saturating_sub(1)].right == 1 {
                    queue.push_back((i, j.saturating_sub(1)));
                }

                if pipe.right == 1 && grid[i][(j + 1).min(grid[0].len() - 1)].left == 1 {
                    queue.push_back((i, (j + 1).min(grid[0].len() - 1)));
                }
            }
            visited[i][j] = true;
        }
    }

    dp.iter().flatten().max().unwrap() - 1
}

fn problem1(f: &str) -> i32 {
    let lines = read_lines(f);

    let mut grid: Vec<Vec<Pipe>> = vec![];
    let mut start_i = 0;
    let mut start_j = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut row: Vec<Pipe> = vec![];
        for (j, c) in line.chars().enumerate() {
            let pipe = match c {
                '|' => V,
                '-' => H,
                'L' => L,
                'J' => J,
                '7' => W,
                'F' => F,
                'S' => {
                    start_i = i;
                    start_j = j;
                    S
                }
                _ => B,
            };
            row.push(pipe);
        }
        grid.push(row);
    }
    max_path(start_i, start_j, &grid)
}

fn problem2(f: &str) -> i32 {
    let lines = read_lines(f);

    let mut grid: Vec<Vec<Pipe>> = vec![];
    let mut start_i = 0;
    let mut start_j = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut row: Vec<Pipe> = vec![];
        for (j, c) in line.chars().enumerate() {
            let pipe = match c {
                '|' => V,
                '-' => H,
                'L' => L,
                'J' => J,
                '7' => W,
                'F' => F,
                'S' => {
                    start_i = i;
                    start_j = j;
                    S
                }
                _ => B,
            };
            row.push(pipe);
        }
        grid.push(row);
    }
    enclosed(start_i, start_j, &grid)
}

pub fn solve() {
    // https://adventofcode.com/2023/day/10
    time_it!(10, 1, problem1("files/10.txt"));
    time_it!(10, 2, problem2("files/10.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day10::problem1("files/10_test.txt"), 8);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day10::problem2("files/10_test_2.txt"), 10);
    }
}
