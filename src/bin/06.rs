use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
enum Cell {
    Obstacle,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn next(&self) -> Self {
        use Direction::*;
        match self {
            Left => Up,
            Up => Right,
            Right => Down,
            Down => Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    col: i32,
    row: i32,
}

#[derive(Debug)]
struct Guard {
    direction: Direction,
    position: Position,
}

impl Guard {
    fn next(&self) -> Position {
        use Direction::*;
        let (dx, dy) = match self.direction {
            Left => (-1, 0),
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
        };
        Position {
            col: self.position.col + dx,
            row: self.position.row + dy,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard_position = None;

    let grid: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, char)| match char {
                    '^' => {
                        guard_position = Some(Position {
                            col: col_index as i32,
                            row: row_index as i32,
                        });
                        Cell::Empty
                    }
                    '#' => Cell::Obstacle,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    let mut guard = Guard {
        position: guard_position?,
        direction: Direction::Up,
    };
    let mut visited = HashSet::new();
    visited.insert((guard.position.col, guard.position.row));

    loop {
        let next_position = guard.next();

        if next_position.row < 0
            || next_position.row >= grid.len() as i32
            || next_position.col < 0
            || next_position.col >= grid[0].len() as i32
        {
            break;
        }

        let next_cell = &grid[next_position.row as usize][next_position.col as usize];

        if *next_cell == Cell::Obstacle {
            guard.direction = guard.direction.next();
        } else {
            visited.insert((next_position.col, next_position.row));
            guard.position = next_position;
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut initial_guard_position = None;

    let grid: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, char)| match char {
                    '^' => {
                        initial_guard_position = Some(Position {
                            col: col_index as i32,
                            row: row_index as i32,
                        });
                        Cell::Empty
                    }
                    '#' => Cell::Obstacle,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    let mut valid_new_obstacle_count = 0;

    grid.iter().enumerate().for_each(|(row_index, row)| {
        'row: for (col_index, cell) in row.iter().enumerate() {
            match cell {
                Cell::Obstacle => {}
                Cell::Empty => {
                    let mut guard = Guard {
                        position: initial_guard_position.unwrap(),
                        direction: Direction::Up,
                    };
                    let mut visited = HashSet::new();
                    visited.insert((guard.position, guard.direction));

                    loop {
                        let next_position = guard.next();

                        if visited.contains(&(next_position, guard.direction)) {
                            valid_new_obstacle_count += 1;
                            continue 'row;
                        }

                        if next_position.row < 0
                            || next_position.row >= grid.len() as i32
                            || next_position.col < 0
                            || next_position.col >= grid[0].len() as i32
                        {
                            break;
                        }

                        let next_cell =
                            &grid[next_position.row as usize][next_position.col as usize];

                        if *next_cell == Cell::Obstacle
                            || (next_position.col == col_index as i32
                                && next_position.row == row_index as i32)
                        {
                            guard.direction = guard.direction.next();
                        } else {
                            guard.position = next_position;
                            visited.insert((next_position, guard.direction));
                        }
                    }
                }
            }
        }
        println!("Row {} done", { row_index });
    });

    Some(valid_new_obstacle_count)
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
        assert_eq!(result, Some(6));
    }
}
