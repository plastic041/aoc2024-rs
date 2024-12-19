use itertools::Itertools;

advent_of_code::solution!(19);

#[derive(PartialEq, Eq)]
enum Color {
    W,
    U,
    B,
    R,
    G,
}

impl Color {
    fn new(c: char) -> Self {
        match c {
            'w' => Color::W,
            'u' => Color::U,
            'b' => Color::B,
            'r' => Color::R,
            'g' => Color::G,
            _ => unreachable!(),
        }
    }
}

fn towel(patterns: &[Vec<Color>], design: &[Color]) -> bool {
    let length = design.len();
    let mut dp = vec![false; length + 1];
    dp[0] = true;

    for i in 1..=length {
        for pattern in patterns {
            let pattern_length = pattern.len();
            if i >= pattern_length
                && dp[i - pattern_length]
                && design[i - pattern_length..i] == *pattern
            {
                dp[i] = true;
                break;
            }
        }
    }

    dp[length]
}

pub fn part_one(input: &str) -> Option<usize> {
    let (patterns_str, designs_str) = input.split("\n\n").collect_tuple().unwrap();
    let patterns: Vec<Vec<Color>> = patterns_str
        .split(", ")
        .map(|pattern_str| pattern_str.chars().map(Color::new).collect())
        .collect();
    let designs: Vec<Vec<Color>> = designs_str
        .lines()
        .map(|line| line.chars().map(Color::new).collect())
        .collect();

    let count = designs
        .iter()
        .filter(|design| towel(&patterns, design))
        .count();

    Some(count)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
