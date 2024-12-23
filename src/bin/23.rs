use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

type Graph = HashMap<String, HashSet<String>>;

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph: Graph = HashMap::new();

    input.lines().for_each(|line| {
        let (first, second) = line.split("-").collect_tuple().unwrap();

        graph
            .entry(first.to_string())
            .or_default()
            .insert(second.to_string());

        graph
            .entry(second.to_string())
            .or_default()
            .insert(first.to_string());
    });

    let mut triples: HashSet<Vec<String>> = HashSet::new();

    graph.iter().for_each(|(a, value)| {
        if value.len() >= 2 {
            value.iter().tuple_combinations().for_each(|(b, c)| {
                if graph.get(b).unwrap().contains(c) {
                    let mut v = vec![a.clone(), b.clone(), c.clone()];
                    v.sort_unstable();
                    triples.insert(v);
                }
            });
        }
    });

    Some(
        triples
            .iter()
            .filter(|&set| {
                set[0].starts_with("t") || set[1].starts_with("t") || set[2].starts_with("t")
            })
            .count(),
    )
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
