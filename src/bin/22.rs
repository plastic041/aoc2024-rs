use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn process1(secret: u64) -> u64 {
    ((secret * 64) ^ secret) % 16777216
}

fn process2(secret: u64) -> u64 {
    ((secret / 32) ^ secret) % 16777216
}

fn process3(secret: u64) -> u64 {
    ((secret * 2048) ^ secret) % 16777216
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..2000 {
        numbers = numbers
            .into_iter()
            .map(|number| process3(process2(process1(number))))
            .collect();
    }

    Some(numbers.iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut one_digits = vec![vec![]; numbers.len()];

    for _ in 0..2000 {
        numbers = numbers
            .into_iter()
            .enumerate()
            .map(|(index, number)| {
                let result = process3(process2(process1(number)));
                one_digits[index].push(number % 10);
                result
            })
            .collect();
    }

    let mut maps: Vec<HashMap<(i32, i32, i32, i32), i32>> = vec![HashMap::new(); numbers.len()];

    println!("Created numbers");

    one_digits.iter().enumerate().for_each(|(index, digits)| {
        println!("Creating map {}/{}", index + 1, one_digits.len());
        digits.windows(5).for_each(|ds| {
            let n1 = ds[0] as i32;
            let n2 = ds[1] as i32;
            let n3 = ds[2] as i32;
            let n4 = ds[3] as i32;
            let n5 = ds[4] as i32;
            let tuple = (n2 - n1, n3 - n2, n4 - n3, n5 - n4);
            let value = maps[index].get(&tuple);
            if value.is_none() {
                maps[index].insert(tuple, n5);
            }
        });
    });

    let mut tuples = HashSet::new();
    maps.iter().for_each(|map| {
        map.iter().for_each(|(tuple, _)| {
            tuples.insert(*tuple);
        });
    });

    let mut result_map: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();

    tuples.iter().for_each(|tuple| {
        let result = maps.iter().map(|m| m.get(tuple).unwrap_or(&0)).sum::<i32>();
        result_map.insert(*tuple, result);
    });

    let a = result_map
        .iter()
        .max_by_key(|(_tuple, value)| **value)
        .unwrap();

    Some(*a.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(42 ^ 15, 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(100000000 % 16777216, 16113920);
    }

    #[test]
    fn test_processes() {
        let result = process1(123);
        let result = process2(result);
        let result = process3(result);

        assert_eq!(result, 15887950);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
