use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([\d]+),([\d]+)\)").expect("Invalid regex");

    let mut results = vec![];
    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((
            first.parse::<u32>().expect("Invalid number"),
            second.parse::<u32>().expect("Invalid number"),
        ));
    }

    let sum: u32 = results.iter().map(|result| result.0 * result.1).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_instruction =
        Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").expect("Invalid regex");

    let mut enabled = true; // Tracks whether mul instructions are enabled
    let mut result = 0;

    for cap in re_instruction.captures_iter(input) {
        if let Some(inst) = cap.get(1) {
            match inst.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        // Parse and multiply numbers if mul is enabled
                        if let (Some(x), Some(y)) = (cap.get(2), cap.get(3)) {
                            let x: u32 = x.as_str().parse().expect("Invalid number");
                            let y: u32 = y.as_str().parse().expect("Invalid number");
                            result += x * y;
                        }
                    }
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
