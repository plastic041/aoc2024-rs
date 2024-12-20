use glam::UVec2;
use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::collections::HashMap;

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

    let cheatless_route = solve(&track, &start, &goal, &UVec2::new(0, 0)).unwrap();

    let mut map: HashMap<usize, u32> = HashMap::new();

    walls.iter().for_each(|wall| {
        let result = solve(&track, &start, &goal, wall).unwrap().len();
        let value = map.get(&result);
        match value {
            Some(n) => map.insert(result, n + 1),
            None => map.insert(result, 1),
        };
    });

    Some(
        map.iter()
            .filter(|&(pico, _)| cheatless_route.len() - pico >= 100)
            .map(|(_, count)| *count)
            .sum(),
    )
}

fn solve(track: &[Vec<Cell>], start: &UVec2, goal: &UVec2, cheat: &UVec2) -> Option<Vec<UVec2>> {
    bfs(
        start,
        |&position| {
            [
                position.wrapping_add(UVec2::X),
                position.wrapping_sub(UVec2::X),
                position.wrapping_add(UVec2::Y),
                position.wrapping_sub(UVec2::Y),
            ]
            .into_iter()
            .filter(|&position| {
                track
                    .get(position.y as usize)
                    .and_then(|row| row.get(position.x as usize))
                    .map_or(false, |c| {
                        if cheat.x == position.x && cheat.y == position.y {
                            true
                        } else {
                            matches!(*c, Cell::Empty)
                        }
                    })
            })
            .collect::<Vec<UVec2>>()
        },
        |position| position == goal,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start: Option<UVec2> = None;
    let mut goal: Option<UVec2> = None;

    let track: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Cell::Wall,
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

    let cheatless_route = solve(&track, &start, &goal, &UVec2::new(0, 0)).unwrap();

    let mut map: HashMap<usize, u32> = HashMap::new();
    cheatless_route
        .iter()
        .enumerate()
        .tuple_combinations()
        .for_each(|((from_index, from_position), (to_index, to_position))| {
            let m_distance = (to_position.as_ivec2() - from_position.as_ivec2())
                .abs()
                .element_sum() as usize;
            if m_distance <= 20 {
                let result = from_index + m_distance + cheatless_route.len() - to_index;
                let value = map.get(&result);
                match value {
                    Some(n) => map.insert(result, n + 1),
                    None => map.insert(result, 1),
                };
            }
        });

    Some(
        map.iter()
            .filter(|&(pico, _)| cheatless_route.len() - pico >= 100)
            .map(|(_, count)| *count)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
