use std::fmt::Display;

advent_of_code::solution!(9);

enum Block {
    Free,
    /// Data block with id
    Data(u64),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = if let Block::Data(data_block) = self {
            &data_block.to_string()
        } else {
            "."
        };

        write!(f, "{}", str)
    }
}

/// 0 -> 0, 2 -> 1, 4 -> 2, 6 -> 3
fn get_id(index: usize) -> u64 {
    index as u64 / 2
}

fn has_gap(blocks: &[Block]) -> bool {
    let mut met_free = false;
    for block in blocks {
        if let Block::Free = block {
            met_free = true;
        } else if met_free {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks: Vec<Block> = vec![];

    for (index, char) in input.chars().enumerate() {
        let number = char.to_digit(10).expect("Char is not a number");
        if index % 2 == 0 {
            // even index(0, 2, 4, ...): data block
            for _ in 0..number {
                blocks.push(Block::Data(get_id(index)));
            }
        } else {
            // odd index(1, 3, 5, ...): free space block
            for _ in 0..number {
                blocks.push(Block::Free);
            }
        }
    }

    loop {
        if !has_gap(&blocks) {
            break;
        }

        // do
        // find from backwards
        let mut free_block_index = None;
        for (index, _) in blocks.iter().enumerate() {
            if let Block::Free = blocks[index] {
                free_block_index = Some(index);
                break;
            }
        }

        let mut data_block_index = None;
        for (i, _) in blocks.iter().enumerate() {
            let index = blocks.len() - i - 1;
            if let Block::Data(_) = blocks[index] {
                data_block_index = Some(index);
                break;
            }
        }

        blocks.swap(
            free_block_index.expect("Free block index is None"),
            data_block_index.expect("Data block index is None"),
        );
    }

    Some(blocks.iter().enumerate().fold(0, |acc, (index, block)| {
        acc + match block {
            Block::Data(data_block) => index as u64 * data_block,
            Block::Free => 0,
        }
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
