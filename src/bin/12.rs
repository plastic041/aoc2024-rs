use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

type Graph = HashMap<((usize, usize), (usize, usize)), bool>;

fn toggle_edge(graph: &mut Graph, pos1: &(usize, usize), pos2: &(usize, usize)) {
    let e = graph.get(&(*pos1, *pos2));
    let value = match e {
        Some(edge) => !edge,
        None => true,
    };

    graph.insert((*pos1, *pos2), value);
}

fn to_graph(map: &[Vec<bool>]) -> Graph {
    let mut graph = HashMap::new();

    map.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(col_index, value)| {
            if *value {
                toggle_edge(
                    &mut graph,
                    &(col_index, row_index),
                    &(col_index, row_index + 1),
                );
                toggle_edge(
                    &mut graph,
                    &(col_index, row_index + 1),
                    &(col_index + 1, row_index + 1),
                );
                toggle_edge(
                    &mut graph,
                    &(col_index + 1, row_index),
                    &(col_index + 1, row_index + 1),
                );
                toggle_edge(
                    &mut graph,
                    &(col_index, row_index),
                    &(col_index + 1, row_index),
                );
            }
        });
    });

    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<_> = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .sorted()
        .dedup_with_count()
        .collect();

    let mut result = 0;
    chars.iter().for_each(|(area, char)| {
        let map: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == *char).collect())
            .collect();

        let graph = to_graph(&map);
        let perimeter = graph.values().filter(|v| **v).count();
        let price = perimeter * area;
        result += price;

        println!("{}: {} * {}", char, area, perimeter);
    });

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
