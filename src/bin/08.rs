use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Antenna {
    frequency: char,
    position: Position,
}

enum Cell {
    Empty,
    Antenna,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug)]
struct Distance {
    col: isize,
    row: isize,
}

impl Distance {
    fn multiply(&self, by: isize) -> Distance {
        Distance {
            row: self.row * by,
            col: self.col * by,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn distance(&self, other: &Position) -> Distance {
        Distance {
            row: self.row as isize - other.row as isize,
            col: self.col as isize - other.col as isize,
        }
    }

    fn checked_sub(&self, distance: &Distance) -> Option<Position> {
        let row = self.row.checked_add_signed(distance.row);
        let col = self.col.checked_add_signed(distance.col);

        if let (Some(row), Some(col)) = (row, col) {
            Some(Position { row, col })
        } else {
            None
        }
    }

    fn checked_add(&self, distance: &Distance) -> Option<Position> {
        let row = self.row.checked_add_signed(-distance.row);
        let col = self.col.checked_add_signed(-distance.col);

        if let (Some(row), Some(col)) = (row, col) {
            Some(Position { row, col })
        } else {
            None
        }
    }

    fn is_valid(&self, grid: &Grid) -> bool {
        let row_len = grid.cells.len();
        let col_len = grid.cells[0].len();
        self.row < row_len && self.col < col_len
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas = vec![];
    let grid = Grid {
        cells: input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_index, char)| match char {
                        '.' => Cell::Empty,
                        c => {
                            let antenna = Antenna {
                                frequency: c,
                                position: Position {
                                    row: row_index,
                                    col: col_index,
                                },
                            };
                            antennas.push(antenna);
                            Cell::Antenna
                        }
                    })
                    .collect()
            })
            .collect(),
    };

    let mut antinodes: HashSet<Position> = HashSet::new();

    antennas.iter().combinations(2).for_each(|_antennas| {
        let a = _antennas[0];
        let b = _antennas[1];

        if a != b && a.frequency == b.frequency {
            let distance = a.position.distance(&b.position);

            let smaller_position = a.position.checked_sub(&distance);
            if let Some(smaller) = smaller_position {
                if smaller.is_valid(&grid) {
                    antinodes.insert(smaller);
                }
            }
            let bigger_position = b.position.checked_add(&distance);
            if let Some(bigger) = bigger_position {
                if bigger.is_valid(&grid) {
                    antinodes.insert(bigger);
                }
            }
        }
    });

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antennas = vec![];
    let grid = Grid {
        cells: input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_index, char)| match char {
                        '.' => Cell::Empty,
                        c => {
                            let antenna = Antenna {
                                frequency: c,
                                position: Position {
                                    row: row_index,
                                    col: col_index,
                                },
                            };
                            antennas.push(antenna);
                            Cell::Antenna
                        }
                    })
                    .collect()
            })
            .collect(),
    };

    let mut antinodes: HashSet<Position> = HashSet::new();

    antennas.iter().combinations(2).for_each(|_antennas| {
        let a = _antennas[0];
        let b = _antennas[1];

        if a != b && a.frequency == b.frequency {
            let distance_base = a.position.distance(&b.position);

            let mut positions = vec![a.position, b.position];

            let mut smaller_count = 1;
            loop {
                let dist = distance_base.multiply(smaller_count);
                let position = a.position.checked_sub(&dist);

                if let Some(smaller) = position {
                    if smaller.is_valid(&grid) {
                        positions.push(smaller);
                        smaller_count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let mut bigger_count = 1;
            loop {
                let dist = distance_base.multiply(bigger_count);
                let position = b.position.checked_add(&dist);

                if let Some(bigger) = position {
                    if bigger.is_valid(&grid) {
                        positions.push(bigger);
                        bigger_count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            positions.into_iter().for_each(|position| {
                antinodes.insert(position);
            });
        }
    });

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
