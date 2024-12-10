advent_of_code::solution!(9);

enum Block {
    Free,
    /// Data block with id
    Data(u32),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks: Vec<Block> = vec![];

    for (index, char) in input.chars().enumerate() {
        let number = char.to_digit(10).expect("Char is not a number");
        if index % 2 == 0 {
            // even index(0, 2, 4, ...): data block
            for _ in 0..number {
                blocks.push(Block::Data(index as u32 / 2));
            }
        } else {
            // odd index(1, 3, 5, ...): free space block
            for _ in 0..number {
                blocks.push(Block::Free);
            }
        }
    }

    let mut free_block_index = 0;
    let mut data_block_index = blocks.len() - 1;

    loop {
        while let Block::Data(_) = &blocks[free_block_index] {
            free_block_index += 1;
        }
        while let Block::Free = &blocks[data_block_index] {
            data_block_index -= 1;
        }

        if free_block_index >= data_block_index {
            break;
        } else {
            blocks.swap(free_block_index, data_block_index);
        }
    }

    Some(blocks.iter().enumerate().fold(0, |acc, (index, block)| {
        acc + match block {
            Block::Data(data_block) => index as u64 * *data_block as u64,
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
