use std::collections::HashMap;

use glam::UVec2;
use itertools::{enumerate, Itertools};
use pathfinding::prelude::bfs;

advent_of_code::solution!(20);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: Option<UVec2> = None;
    let mut goal: Option<UVec2> = None;

    let mut walls: Vec<UVec2> = vec![];
    let track: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => {
                        walls.push(UVec2::new(x as u32, y as u32));
                        Cell::Wall
                    }
                    'S' => {
                        start = Some(UVec2::new(x as u32, y as u32));
                        Cell::Empty
                    }
                    'E' => {
                        goal = Some(UVec2::new(x as u32, y as u32));
                        Cell::Empty
                    }
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    let start = start.unwrap();
    let goal = goal.unwrap();

    let without_cheat = solve(&track, &start, &goal, &UVec2::new(0, 0)).unwrap();

    let mut map: HashMap<usize, u32> = HashMap::new();

    walls.iter().for_each(|wall| {
        let result = solve(&track, &start, &goal, wall).unwrap();
        let value = map.get(&result);
        match value {
            Some(n) => map.insert(result, n + 1),
            None => map.insert(result, 1),
        };
    });

    Some(
        map.iter()
            .filter(|&(pico, _)| without_cheat - pico >= 100)
            .map(|(_, count)| *count)
            .sum(),
    )
    // let  counts = map.iter().collect::<Vec<_>>();

    // counts.sort_unstable_by(|a, b| b.0.cmp(a.0));
    // counts.iter().for_each(|&(pico, count)| {
    //     println!("count: {}, pico: {}", count, without_cheat - pico);
    // });

    // None
}

fn solve(track: &[Vec<Cell>], start: &UVec2, goal: &UVec2, cheat: &UVec2) -> Option<usize> {
    let solution = bfs(
        &(start.x, start.y),
        |&(x, y)| {
            [
                (x + 1, y),
                (x.wrapping_sub(1), y),
                (x, y + 1),
                (x, y.wrapping_sub(1)),
            ]
            .iter()
            .filter(|&(x, y)| {
                track
                    .get(*y as usize)
                    .and_then(|row| row.get(*x as usize))
                    .map_or(false, |c| {
                        if cheat.x == *x && cheat.y == *y {
                            true
                        } else {
                            matches!(*c, Cell::Empty)
                        }
                    })
            })
            .map(|&(x, y)| (x, y))
            .collect::<Vec<_>>()
        },
        |&p| p.0 == goal.x && p.1 == goal.y,
    );

    solution.map(|s| s.len() - 1) // minus start
}

fn is_neighbors(a: &UVec2, b: &UVec2) -> bool {
    if a == b {
        false
    } else {
        let diffx = a.x.abs_diff(b.x);
        let diffy = a.y.abs_diff(b.y);
        diffx + diffy == 1
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let a = UVec2::new(1, 0);
        let b = UVec2::new(0, 1);

        assert!(!is_neighbors(&a, &b));

        let c = UVec2::new(1, 0);
        let d = UVec2::new(0, 0);

        assert!(is_neighbors(&c, &d));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
