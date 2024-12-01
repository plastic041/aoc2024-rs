use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect();

        if numbers.len() == 2 {
            lefts.push(numbers[0]);
            rights.push(numbers[1]);
        }
    });

    lefts.sort_unstable();
    rights.sort_unstable();

    Some(
        lefts
            .iter()
            .zip(rights.iter())
            .map(|(a, b)| (a - b).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lefts: Vec<u32> = Vec::new();
    let mut rights: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Invalid number"))
            .collect();

        if numbers.len() == 2 {
            lefts.push(numbers[0]);
            rights.push(numbers[1]);
        }
    });

    let mut counter_map = HashMap::new();

    for right in rights {
        let value = counter_map.get(&right);
        match value {
            Some(v) => counter_map.insert(right, v + 1),
            None => counter_map.insert(right, 1),
        };
    }

    Some(
        lefts
            .iter()
            .map(|left| left * counter_map.get(left).unwrap_or(&0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
