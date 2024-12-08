advent_of_code::solution!(4);

struct Direction {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    let xmas = "XMAS";
    let xmas_chars: Vec<_> = xmas.chars().collect();

    let directions = [
        Direction { x: 0, y: 1 },
        Direction { x: 0, y: -1 },
        Direction { x: 1, y: 0 },
        Direction { x: -1, y: 0 },
        Direction { x: 1, y: 1 },
        Direction { x: 1, y: -1 },
        Direction { x: -1, y: 1 },
        Direction { x: -1, y: -1 },
    ];

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            directions.iter().for_each(|direction| {
                let row_index = direction.y;
                let col_index = direction.x;
                let mut is_xmas = true;

                for char_index in 0..xmas.len() {
                    let row_offset = row as i32 + row_index * char_index as i32;
                    let col_offset = col as i32 + col_index * char_index as i32;

                    let char_option = grid
                        .get(row_offset as usize)
                        .and_then(|row| row.get(col_offset as usize));

                    match char_option {
                        Some(char) => {
                            if *char != xmas_chars[char_index] {
                                is_xmas = false;
                                break;
                            }
                        }
                        None => {
                            is_xmas = false;
                            break;
                        }
                    }
                }

                if is_xmas {
                    count += 1;
                }
            });
        }
    }

    Some(count)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
