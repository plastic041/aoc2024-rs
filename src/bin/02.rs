advent_of_code::solution!(2);

fn is_asc(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[1] > w[0] && w[1] - w[0] <= 3)
}

fn is_desc(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] <= 3)
}

pub fn part_one(input: &str) -> Option<i32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().expect("Not a number"))
                .collect()
        })
        .collect();

    Some(
        reports
            .iter()
            .filter(|report| is_asc(report) || is_desc(report))
            .count() as i32,
    )
}

fn slices_with_removed_elements(input: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for i in 0..input.len() {
        let mut temp = Vec::new();
        temp.extend(&input[..i]); // Elements before index i
        temp.extend(&input[i + 1..]); // Elements after index i
        result.push(temp);
    }

    result
}

pub fn part_two(input: &str) -> Option<i32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().expect("Not a number"))
                .collect()
        })
        .collect();

    Some(
        reports
            .iter()
            .filter(|report| {
                slices_with_removed_elements(report)
                    .iter()
                    .any(|r| is_asc(r) || is_desc(r))
            })
            .count() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
