use std::collections::{HashMap, HashSet};

use glam::IVec2;

advent_of_code::solution!(12);

type Graph = HashMap<(IVec2, IVec2), bool>;

fn toggle_edge(graph: &mut Graph, pos1: IVec2, pos2: IVec2) {
    let e = graph.get(&(pos1, pos2));
    let value = match e {
        Some(edge) => !edge,
        None => true,
    };

    graph.insert((pos1, pos2), value);
}

fn get_perimeter(map: &[IVec2]) -> usize {
    let mut graph = HashMap::new();

    map.iter().for_each(|vec| {
        toggle_edge(
            &mut graph,
            IVec2::new(vec.x, vec.y),
            IVec2::new(vec.x, vec.y + 1),
        );
        toggle_edge(
            &mut graph,
            IVec2::new(vec.x, vec.y + 1),
            IVec2::new(vec.x + 1, vec.y + 1),
        );
        toggle_edge(
            &mut graph,
            IVec2::new(vec.x + 1, vec.y),
            IVec2::new(vec.x + 1, vec.y + 1),
        );
        toggle_edge(
            &mut graph,
            IVec2::new(vec.x, vec.y),
            IVec2::new(vec.x + 1, vec.y),
        );
    });

    graph.values().filter(|v| **v).count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<IVec2> = HashSet::new();

    let mut maps: Vec<HashSet<IVec2>> = vec![];

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let start = IVec2::new(x as i32, y as i32);
            if !visited.contains(&start) {
                let mut stack = vec![start];
                let mut local_visited: HashSet<IVec2> = HashSet::new();
                let current_char = grid[y][x];

                // loop
                while let Some(vec) = stack.pop() {
                    if local_visited.insert(vec) {
                        for neighbor_vec in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                            .iter()
                            .map(|dvec| vec + dvec)
                        {
                            if let Some(neighbor_char) = grid
                                .get(neighbor_vec.y as usize)
                                .and_then(|row| row.get(neighbor_vec.x as usize))
                            {
                                if *neighbor_char == current_char && visited.insert(neighbor_vec) {
                                    stack.push(neighbor_vec)
                                }
                            }
                        }
                    }
                }

                maps.push(local_visited);
            }
        }
    }

    let mut result = 0;
    maps.into_iter().for_each(|set| {
        let area = set.len();
        let perimeter = get_perimeter(&Vec::from_iter(set));
        let price = perimeter * area;
        result += price;
    });

    Some(result as u32)
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
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
