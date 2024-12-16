use grid::Grid;
use std::fmt::{Display, Write as _};

use glam::IVec2;
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug)]
enum Cell {
    Empty,
    Wall,
    Box,
    Character,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => ".",
                Cell::Wall => "#",
                Cell::Box => "O",
                Cell::Character => "@",
            }
        )
    }
}

struct Game {
    grid: Grid<Cell>,
}

impl Game {
    fn get(&self, position: &IVec2) -> Option<&Cell> {
        self.grid.get(position.y, position.x)
    }

    fn new(input: &str) -> (IVec2, Self) {
        let mut character = None;
        let mut width = None;
        let grid: Vec<Cell> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                width = Some(line.len());
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '#' => Cell::Wall,
                        'O' => Cell::Box,
                        '.' => Cell::Empty,
                        '@' => {
                            character = Some(IVec2::new(x as i32, y as i32));
                            Cell::Character
                        }
                        _ => panic!("Unknown character"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        (
            character.unwrap(),
            Game {
                grid: Grid::from_vec(grid, width.unwrap()),
            },
        )
    }

    fn get_neighbor(&self, from: &IVec2, dir: &Dir) -> Option<&Cell> {
        let neighbor_position = get_neighbor_position(from, dir);
        self.get(&neighbor_position)
    }

    fn checked_move(&mut self, from: &IVec2, dir: &Dir) -> Option<IVec2> {
        let neighbor_cell = self.get_neighbor(from, dir).unwrap();

        match neighbor_cell {
            Cell::Empty => self.safe_move(from, dir),
            Cell::Wall => None,
            Cell::Box => {
                let neighbor_position = get_neighbor_position(from, dir);
                if self.checked_move(&neighbor_position, dir).is_some() {
                    self.safe_move(from, dir);
                    Some(neighbor_position)
                } else {
                    None
                }
            }
            Cell::Character => panic!("?"),
        }
    }

    fn safe_move(&mut self, from: &IVec2, dir: &Dir) -> Option<IVec2> {
        let neighbor = get_neighbor_position(from, dir);
        self.grid.swap(
            (from.y as usize, from.x as usize),
            (neighbor.y as usize, neighbor.x as usize),
        );
        Some(neighbor)
    }

    fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        self.grid.iter_rows().for_each(|row| {
            row.for_each(|cell| {
                write!(&mut buffer, "{}", cell).unwrap();
            });
            writeln!(&mut buffer).unwrap();
        });
        println!("{}", buffer);

        Ok(())
    }
}

fn get_neighbor_position(position: &IVec2, dir: &Dir) -> IVec2 {
    position + dir.ddir()
}

#[derive(Debug)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

impl Dir {
    fn ddir(&self) -> IVec2 {
        use Dir::*;
        match self {
            Left => IVec2::NEG_X,
            Up => IVec2::NEG_Y,
            Right => IVec2::X,
            Down => IVec2::Y,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (game_str, dirs_str) = input.split("\n\n").collect_tuple().unwrap();
    let (mut character, mut game) = Game::new(game_str);
    let binding = dirs_str.replace("\n", "");
    let dirs: Vec<Dir> = binding
        .chars()
        .map(|char| match char {
            '<' => Dir::Left,
            '^' => Dir::Up,
            '>' => Dir::Right,
            'v' => Dir::Down,
            _ => panic!("Unknown Dir"),
        })
        .collect();

    dirs.iter().for_each(|dir| {
        if let Some(next_pos) = game.checked_move(&character, dir) {
            character = next_pos;
        }
    });

    game.render().unwrap();

    Some(
        game.grid
            .iter_rows()
            .enumerate()
            .flat_map(|(y, row)| {
                row.enumerate()
                    .map(|(x, cell)| match cell {
                        Cell::Box => y * 100 + x,
                        _ => 0,
                    })
                    .collect::<Vec<_>>()
            })
            .sum::<usize>() as u32,
    )
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
