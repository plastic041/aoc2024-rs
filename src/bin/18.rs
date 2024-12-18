use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::bfs;

advent_of_code::solution!(18);

const SIZE: i32 = 70; // 6 on example, 70 on real input
const TAKE: usize = 1024; // 12 on example, 1024 on real input

pub fn part_one(input: &str) -> Option<u32> {
    let corrupteds = input
        .lines()
        .take(TAKE)
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect::<HashSet<_>>();

    let start = (0, 0);
    let goal = (SIZE, SIZE);

    let result = bfs(
        &start,
        |&(x, y)| {
            let a: Vec<(i32, i32)> = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter(|(x, y)| *x >= 0 && *x <= SIZE && *y >= 0 && *y <= SIZE)
                .filter(|(x, y)| !corrupteds.contains(&(*x, *y)))
                .map(|(x, y)| (*x, *y))
                .collect();
            a
        },
        |&p| p == goal,
    );

    Some(
        result.unwrap().len() as u32
        // minus start point
        - 1,
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut take = 0;

    while take < input.lines().count() {
        let corrupteds = input
            .lines()
            .take(take)
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect_tuple::<(i32, i32)>()
                    .unwrap()
            })
            .collect::<HashSet<_>>();
        let start = (0, 0);
        let goal = (SIZE, SIZE);

        let result = bfs(
            &start,
            |&(x, y)| {
                let a: Vec<(i32, i32)> = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *x <= SIZE && *y >= 0 && *y <= SIZE)
                    .filter(|(x, y)| !corrupteds.contains(&(*x, *y)))
                    .map(|(x, y)| (*x, *y))
                    .collect();
                a
            },
            |&p| p == goal,
        );

        match result {
            Some(_) => {
                take += 1;
            }
            None => break,
        }
    }

    Some(input.lines().nth(take - 1).unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(146));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("2,0".to_string()));
    }
}
