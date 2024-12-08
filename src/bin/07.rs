advent_of_code::solution!(7);

#[derive(Clone, Copy, Debug)]
enum Operation {
    Multiply,
    Add,
    Concatenation,
}

impl Operation {
    fn apply(&self, x: u64, y: u64) -> u64 {
        match self {
            Operation::Multiply => x * y,
            Operation::Add => x + y,
            Operation::Concatenation => format!("{}{}", x, y)
                .parse()
                .expect("Failed to concatenate"),
        }
    }
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn new(test_value: u64, numbers: &[u64]) -> Equation {
        Equation {
            test_value,
            numbers: numbers.to_vec(),
        }
    }

    fn calculate(&self, operations: &[Operation]) -> u64 {
        self.numbers.iter().enumerate().fold(
            match operations.first().expect("Operations is empty") {
                Operation::Multiply => 1,
                Operation::Add => 0,
                Operation::Concatenation => 0,
            },
            |acc, (index, number)| match index {
                0 => *number,
                _ => operations[index - 1].apply(acc, *number),
            },
        )
    }
}

fn cartesian_product(values: &[Operation], size: usize) -> Vec<Vec<Operation>> {
    if size == 0 {
        return vec![vec![]]; // Base case: single empty vector
    }

    let smaller = cartesian_product(values, size - 1);
    let mut result = vec![];

    for value in values {
        for mut subset in smaller.clone() {
            subset.push(*value);
            result.push(subset);
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let multiply = Operation::Multiply;
    let add = Operation::Add;
    let operations = &[multiply, add];

    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts
                .next()
                .map(|test_value| {
                    test_value
                        .parse::<u64>()
                        .expect("Failed to parse test number")
                })
                .expect("Valid test value not found");

            let numbers = parts
                .next()
                .map(|nums| {
                    nums.split_whitespace()
                        .map(|num_str| num_str.parse::<u64>().expect("Failed to parse numbers"))
                        .collect::<Vec<u64>>()
                })
                .expect("Valid numbers not found.");

            Equation::new(test_value, &numbers)
        })
        .collect();

    let mut total_calibration_result = 0;

    equations.iter().for_each(|equation| {
        let operation_combinations = cartesian_product(operations, equation.numbers.len() - 1);
        for operations in operation_combinations.iter() {
            if equation.calculate(operations) == equation.test_value {
                total_calibration_result += equation.test_value;
                break;
            }
        }
    });

    Some(total_calibration_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts
                .next()
                .map(|test_value| {
                    test_value
                        .parse::<u64>()
                        .expect("Failed to parse test number")
                })
                .expect("Valid test value not found");

            let numbers = parts
                .next()
                .map(|nums| {
                    nums.split_whitespace()
                        .map(|num_str| num_str.parse::<u64>().expect("Failed to parse numbers"))
                        .collect::<Vec<u64>>()
                })
                .expect("Valid numbers not found.");

            Equation::new(test_value, &numbers)
        })
        .collect();

    let mut total_calibration_result = 0;

    let operations = &[
        Operation::Multiply,
        Operation::Add,
        Operation::Concatenation,
    ];

    equations.iter().for_each(|equation| {
        let operation_combinations = cartesian_product(operations, equation.numbers.len() - 1);
        for operations in operation_combinations.iter() {
            if equation.calculate(operations) == equation.test_value {
                total_calibration_result += equation.test_value;
                break;
            }
        }
    });

    Some(total_calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
