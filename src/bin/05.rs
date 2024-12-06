use std::collections::HashSet;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
enum Cell {
    Obstacle,
    Empty,
}

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Guard {
    direction: Direction,
    position: Position,
}

impl Guard {
    fn next(&self) -> Position {
        match self.direction {
            Direction::Left => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
            Direction::Up => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Direction::Right => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Direction::Down => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard = Guard {
        position: Position { x: 0, y: 0 },
        direction: Direction::Up,
    };

    let grid: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(col_index, line)| {
            line.chars()
                .enumerate()
                .map(|(row_index, char)| {
                    if char == '^' {
                        guard.position.x = row_index as i32;
                        guard.position.y = col_index as i32;
                    }
                    if char == '#' {
                        Cell::Obstacle
                    } else {
                        Cell::Empty
                    }
                })
                .collect()
        })
        .collect();

    let mut visited = HashSet::new();
    visited.insert((guard.position.x, guard.position.y));

    loop {
        let next_position = guard.next();

        if next_position.y < 0
            || next_position.y >= grid.len() as i32
            || next_position.x < 0
            || next_position.x >= grid[0].len() as i32
        {
            break;
        }

        let next_cell = &grid[next_position.y as usize][next_position.x as usize];

        if *next_cell == Cell::Obstacle {
            guard.direction = guard.direction.next();
        } else {
            visited.insert((next_position.x, next_position.y));
            guard.position = next_position;
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
