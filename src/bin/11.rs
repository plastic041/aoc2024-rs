use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Hash, PartialEq, Eq, Clone)]
struct Stone(String);

impl Stone {
    fn one() -> Self {
        Stone("1".to_string())
    }

    fn split(&self) -> (Self, Self) {
        let (first, second) = self.0.split_at(self.0.len() / 2);

        (
            Stone(first.parse::<u64>().unwrap().to_string()),
            Stone(second.parse::<u64>().unwrap().to_string()),
        )
    }

    fn multiply(&self, by: u64) -> Self {
        Stone((self.0.parse::<u64>().unwrap() * by).to_string())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = vec![];

    input
        .split_whitespace()
        .for_each(|stone| stones.push(Stone(stone.to_string())));

    for _ in 0..25 {
        let mut new_stones = vec![];

        for stone in stones.iter() {
            if stone.0 == "0" {
                new_stones.push(Stone::one());
            } else if stone.0.len() % 2 == 0 {
                let (first, second) = stone.split();
                new_stones.push(first);
                new_stones.push(second);
            } else {
                new_stones.push(stone.multiply(2024));
            }
        }

        stones = new_stones;
    }

    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones: HashMap<Stone, u64> = HashMap::new();

    input.split_whitespace().for_each(|stone| {
        stones.insert(Stone(stone.to_string()), 1);
    });

    for _ in 0..75 {
        let mut new_stones = HashMap::new();

        for (stone, count) in stones.iter() {
            if stone.0 == "0" {
                let one = Stone::one();
                let value = new_stones.get(&one).unwrap_or(&0);
                new_stones.insert(one, value + count);
            } else if stone.0.len() % 2 == 0 {
                let (first, second) = stone.split();

                let first_value = new_stones.get(&first).unwrap_or(&0);
                new_stones.insert(first, first_value + count);
                let second_value = new_stones.get(&second).unwrap_or(&0);
                new_stones.insert(second, second_value + count);
            } else {
                let multiplied = stone.multiply(2024);
                let value = new_stones.get(&multiplied).unwrap_or(&0);
                new_stones.insert(multiplied, count + value);
            }
        }

        stones = new_stones;
    }

    Some(stones.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
