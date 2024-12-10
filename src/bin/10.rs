use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn neighbors(&self, grid: &[Vec<i32>]) -> Vec<Position> {
        let dxys: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        dxys.iter()
            .filter(|(col, row)| {
                *col >= 0 && *row >= 0 || *col < grid[0].len() as i32 || *row < grid.len() as i32
            })
            .map(|(col, row)| Position {
                col: self.col + col,
                row: self.row + row,
            })
            .collect()
    }

    fn is_valid(&self, grid: &[Vec<i32>]) -> bool {
        let row_len = grid.len();
        let col_len = grid[0].len();
        self.row > 0 && self.col > 0 && self.row < row_len as i32 && self.col < col_len as i32
    }
}

fn dfs_score(start_at: &Position, grid: &[Vec<i32>]) -> i32 {
    let mut stack = vec![start_at.clone()];
    let mut visited = HashSet::new();

    let mut score = 0;

    while let Some(position) = stack.pop() {
        let cell = grid
            .get(position.row as usize)
            .and_then(|row| row.get(position.col as usize))
            .expect("Cell not found");

        if visited.insert(position.clone()) {
            if *cell == 9 {
                score += 1;
            }
            for neighbor_position in position.neighbors(grid) {
                if let Some(neighbor_cell) = grid
                    .get(neighbor_position.row as usize)
                    .and_then(|row| row.get(neighbor_position.col as usize))
                {
                    if cell + 1 == *neighbor_cell {
                        stack.push(neighbor_position);
                    }
                }
            }
        }
    }

    score
}

fn dfs_rating(
    from: &Position,
    to: &Position,
    grid: &[Vec<i32>],
    visited: &mut HashSet<Position>,
) -> i32 {
    if from == to {
        return 1;
    }
    if !visited.insert(from.clone()) {
        return 0;
    }
    let mut path_count = 0;
    for neighbor in from.neighbors(grid) {
        if neighbor.is_valid(grid) {
            path_count += dfs_rating(&neighbor, to, grid, visited);
        }
    }
    path_count
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut zero_positions = vec![];

    let grid: Vec<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, char)| {
                    if char == '0' {
                        zero_positions.push(Position {
                            col: col_index as i32,
                            row: row_index as i32,
                        })
                    }

                    char.to_digit(10).expect("Parse failed") as i32
                })
                .collect()
        })
        .collect();

    Some(zero_positions.iter().fold(0, |acc, zero_position| {
        acc + dfs_score(zero_position, &grid)
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut zero_positions = vec![];
    let mut nine_positions = vec![];

    let grid: Vec<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, char)| {
                    if char == '0' {
                        zero_positions.push(Position {
                            col: col_index as i32,
                            row: row_index as i32,
                        })
                    } else if char == '9' {
                        nine_positions.push(Position {
                            col: col_index as i32,
                            row: row_index as i32,
                        })
                    }

                    char.to_digit(10).expect("Parse failed") as i32
                })
                .collect()
        })
        .collect();

    let mut total_rating = 0;
    for zero_position in zero_positions.iter() {
        for nine_position in nine_positions.iter() {
            let rating = dfs_rating(zero_position, nine_position, &grid, &mut HashSet::new());
            println!("{}", rating);
            total_rating += rating;
        }
    }

    Some(total_rating)

    // Some(zero_positions.iter().fold(0, |acc, zero_position| {
    //     nine_positions.iter().fold(acc, |_acc, nine_position| {
    //         _acc + dfs_rating(None, zero_position, nine_position, &grid)
    //     })
    // }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(180)); // WRONG
    }
}
