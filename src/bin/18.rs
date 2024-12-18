use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::bfs;

advent_of_code::solution!(18);

const SIZE: i32 = 6; // 6 on example, 70 on real input
const TAKE: usize = 12; // 12 on example, 1024 on real input

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
    let all_corrupteds = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>();

    let mut left = 0usize;
    let mut right = all_corrupteds.len() - 1;

    loop {
        let mid = (left + right) / 2;
        let first = check_take(&all_corrupteds, mid);
        let second = check_take(&all_corrupteds, mid + 1);
        match (first.is_some(), second.is_some()) {
            // too left
            (true, true) => left = mid + 1,
            // answer
            (true, false) => break,
            // should not happen
            (false, true) => unreachable!(),
            // too right
            (false, false) => right = mid - 1,
        }
    }

    Some(input.lines().nth((left + right) / 2).unwrap().to_string())
}

fn check_take(all_corrupteds: &[(i32, i32)], take: usize) -> Option<Vec<(i32, i32)>> {
    let corrupteds: HashSet<&(i32, i32)> = all_corrupteds.iter().take(take).collect();
    let start = (0, 0);
    let goal = (SIZE, SIZE);

    bfs(
        &start,
        |&(x, y)| {
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter(|(x, y)| {
                    *x >= 0
                        && *x <= SIZE
                        && *y >= 0
                        && *y <= SIZE
                        && !corrupteds.contains(&(*x, *y))
                })
                .map(|(x, y)| (*x, *y))
                .collect::<Vec<_>>()
        },
        |&p| p == goal,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
