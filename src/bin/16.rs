use glam::IVec2;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

advent_of_code::solution!(16);

#[derive(Debug)]
enum Cell {
    Empty,
    Wall,
}

fn parse_grid(input: &str) -> (Vec<Vec<Cell>>, IVec2, IVec2) {
    let mut from = None;
    let mut to = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Cell::Empty,
                    'S' => {
                        from = Some(IVec2::new(x as i32, y as i32));
                        Cell::Empty
                    }
                    'E' => {
                        to = Some(IVec2::new(x as i32, y as i32));
                        Cell::Empty
                    }
                    _ => Cell::Wall,
                })
                .collect()
        })
        .collect();

    (grid, from.unwrap(), to.unwrap())
}

fn get_neighbors(position: &IVec2, grid: &[Vec<Cell>]) -> Vec<(IVec2, Dir)> {
    let dvecs = [
        (IVec2::X, Dir::Right),
        (IVec2::Y, Dir::Down),
        (IVec2::NEG_X, Dir::Left),
        (IVec2::NEG_Y, Dir::Up),
    ];
    dvecs
        .into_iter()
        .filter(|(dvec, _dir)| {
            let neighbor = *dvec + position;
            let neighbor_cell = grid
                .get(neighbor.y as usize)
                .and_then(|row| row.get(neighbor.x as usize));

            match neighbor_cell {
                Some(cell) => matches!(cell, Cell::Empty),
                None => false,
            }
        })
        .collect()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

fn pathfinding(grid: &[Vec<Cell>], from: IVec2, to: IVec2) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, from.x, from.y, Dir::Right)));

    let mut visited: HashMap<(IVec2, Dir), u32> = HashMap::new();
    visited.insert((from, Dir::Right), 0);

    let size = grid.len() * grid[0].len();

    while let Some(Reverse((cost, x, y, dir))) = queue.pop() {
        let position = IVec2::new(x, y);
        if size == visited.len() {
            break;
        }

        if let Some(&min_cost) = visited.get(&(position, dir)) {
            if cost > min_cost {
                continue;
            }
        }

        for (neighbor, next_dir) in get_neighbors(&position, grid) {
            let next_position = position + neighbor;
            let new_cost = if next_dir == dir { 1 } else { 1001 };
            let next_cost = cost + new_cost;

            if visited
                .get(&(next_position, next_dir))
                .map_or(true, |&min_cost| next_cost < min_cost)
            {
                visited.insert((next_position, next_dir), next_cost);
                queue.push(Reverse((
                    next_cost,
                    next_position.x,
                    next_position.y,
                    next_dir,
                )));
            }
        }
    }

    let min = [Dir::Left, Dir::Up, Dir::Right, Dir::Down]
        .into_iter()
        .filter_map(|d| visited.get(&(to, d)))
        .min();

    min.copied()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, from, to) = parse_grid(input);
    let result = pathfinding(&grid, from, to)?;

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3022));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
