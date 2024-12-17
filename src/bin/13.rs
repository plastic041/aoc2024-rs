use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Button {
    x: u128,
    y: u128,
}

#[derive(Debug)]
struct Game {
    a: Button,
    b: Button,
    prize: (u128, u128),
}

impl Button {
    fn parse(input: &str) -> Self {
        let re = Regex::new(r"X\+([\d]+), Y\+([\d]+)").unwrap();
        let captures = re.captures(input).unwrap();

        Button {
            x: captures.get(1).unwrap().as_str().parse::<u128>().unwrap(),
            y: captures.get(2).unwrap().as_str().parse::<u128>().unwrap(),
        }
    }
}

impl Game {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let a = Button::parse(lines.next().unwrap());
        let b = Button::parse(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        Game { a, b, prize }
    }

    fn parse2(input: &str) -> Self {
        let mut lines = input.lines();
        let a = Button::parse(lines.next().unwrap());
        let b = Button::parse(lines.next().unwrap());
        let prize = parse_prize2(lines.next().unwrap());

        Game { a, b, prize }
    }

    fn min_cost(&self) -> Option<u128> {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, 0, 0)));

        let mut visited: HashMap<(u128, u128), u128> = HashMap::new();
        visited.insert((0, 0), 0);

        while let Some(Reverse((cost, x, y))) = queue.pop() {
            if (x, y) == self.prize {
                println!("{}: {}, {}", cost, x, y);
                return Some(cost);
            }

            if let Some(&min_cost) = visited.get(&(x, y)) {
                if cost > min_cost {
                    continue;
                }
            }

            let next_a = (x + self.a.x, y + self.a.y);
            let new_cost_a = cost + 3;

            if next_a.0 <= self.prize.0
                && next_a.1 <= self.prize.1
                && visited
                    .get(&next_a)
                    .map_or(true, |&min_cost| new_cost_a < min_cost)
            {
                visited.insert(next_a, new_cost_a);
                queue.push(Reverse((new_cost_a, next_a.0, next_a.1)));
            }

            let next_b = (x + self.b.x, y + self.b.y);
            let new_cost_b = cost + 1;
            if next_b.0 <= self.prize.0
                && next_b.1 <= self.prize.1
                && visited
                    .get(&next_b)
                    .map_or(true, |&min_cost| new_cost_b < min_cost)
            {
                visited.insert(next_b, new_cost_b);
                queue.push(Reverse((new_cost_b, next_b.0, next_b.1)));
            }
        }

        None
    }
}

fn parse_prize(input: &str) -> (u128, u128) {
    let re = Regex::new(r"X=([\d]+), Y=([\d]+)").unwrap();
    let captures = re.captures(input).unwrap();

    (
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    )
}

fn parse_prize2(input: &str) -> (u128, u128) {
    let re = Regex::new(r"X=([\d]+), Y=([\d]+)").unwrap();
    let captures = re.captures(input).unwrap();

    (
        captures.get(1).unwrap().as_str().parse::<u128>().unwrap() + 10000000000000,
        captures.get(2).unwrap().as_str().parse::<u128>().unwrap() + 10000000000000,
    )
}

pub fn part_one(input: &str) -> Option<u128> {
    let games: Vec<Game> = input.split("\n\n").map(Game::parse).collect();

    Some(
        games
            .iter()
            .filter_map(|game| {
                // 94a + 22b = 8400 => x
                // 34a + 67b = 5400 => y

                let determinant = (game.a.x * game.b.y).abs_diff(game.a.y * game.b.x);

                if determinant == 0 {
                    None
                } else {
                    let xfirst = (game.prize.0 * game.b.y).abs_diff(game.prize.1 * game.b.x);
                    let yfirst = (game.a.x * game.prize.1).abs_diff(game.a.y * game.prize.0);

                    let (x, xrem) = (xfirst / determinant, xfirst % determinant);
                    let (y, yrem) = (yfirst / determinant, yfirst % determinant);

                    if xrem == 0 && yrem == 0 {
                        Some((x, y))
                    } else {
                        None
                    }
                }
            })
            .map(|(x, y)| x * 3 + y)
            .sum::<u128>(),
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    let games: Vec<Game> = input.split("\n\n").map(Game::parse2).collect();

    Some(
        games
            .iter()
            .filter_map(|game| {
                // 94a + 22b = 8400 => x
                // 34a + 67b = 5400 => y

                let determinant = (game.a.x * game.b.y).abs_diff(game.a.y * game.b.x);

                if determinant == 0 {
                    None
                } else {
                    let xfirst = (game.prize.0 * game.b.y).abs_diff(game.prize.1 * game.b.x);
                    let yfirst = (game.a.x * game.prize.1).abs_diff(game.a.y * game.prize.0);

                    let (x, xrem) = (xfirst / determinant, xfirst % determinant);
                    let (y, yrem) = (yfirst / determinant, yfirst % determinant);

                    if xrem == 0 && yrem == 0 {
                        Some((x, y))
                    } else {
                        None
                    }
                }
            })
            .map(|(x, y)| x * 3 + y)
            .sum::<u128>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
