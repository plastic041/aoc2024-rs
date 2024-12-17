use core::panic;
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (registers, programs) = input.split("\n\n").collect_tuple().unwrap();
    let mut register_lines = registers.lines();
    let mut reg_a = register_lines
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();
    let mut reg_b = register_lines
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();
    let mut reg_c = register_lines
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();

    let program: Vec<u32> = programs.split(": ").collect::<Vec<_>>()[1]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut pointer = 0;
    let mut outputs = vec![];

    while let Some(opcode) = program.get(pointer) {
        if pointer + 1 >= program.len() {
            break;
        }
        match opcode {
            0 => {
                // adv
                let combo = program[pointer + 1];
                let combo_value = match combo {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => panic!("invalid combo"),
                };
                println!("adv: a = {} / {}", reg_a, 2u32.pow(combo_value));
                reg_a /= 2u32.pow(combo_value);

                pointer += 2;
            }
            1 => {
                let literal = program[pointer + 1];
                reg_b ^= literal;
                println!("bxl: b = {} ^ {}", reg_b, literal);

                pointer += 2;
            }
            2 => {
                let combo = program[pointer + 1];
                let combo_value = match combo {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => panic!("invalid combo"),
                };
                println!("bst: b = {} mod {}", combo_value, 8);
                reg_b = combo_value.rem_euclid(8);

                pointer += 2;
            }
            3 => {
                if reg_a == 0 {
                    println!("jnz, a: {}", reg_a);
                    pointer += 2;
                } else {
                    let literal = program[pointer + 1];
                    println!("jnz, a: {} -> jump to {}", reg_a, literal);
                    pointer = literal as usize;
                }
            }
            4 => {
                println!("bxc, b = {} ^ {}", reg_b, reg_c);
                reg_b ^= reg_c;

                pointer += 2;
            }
            5 => {
                let combo = program[pointer + 1];
                let combo_value = match combo {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => panic!("invalid combo"),
                };
                println!("out, {} mod 8", combo_value);
                outputs.push(combo_value.rem_euclid(8));

                pointer += 2;
            }
            6 => {
                // bdv
                let combo = program[pointer + 1];
                let combo_value = match combo {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => panic!("invalid combo"),
                };
                println!("bdv, b = {} / {}", reg_a, 2u32.pow(combo_value));
                reg_b = reg_a / 2u32.pow(combo_value);

                pointer += 2;
            }
            7 => {
                // cdv
                let combo = program[pointer + 1];
                let combo_value = match combo {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => panic!("invalid combo"),
                };
                println!("cdv, c = {} / {}", reg_a, 2u32.pow(combo_value));
                reg_c = reg_a / 2u32.pow(combo_value);

                pointer += 2;
            }
            _ => panic!("invalid opcode"),
        }
    }

    Some(outputs.into_iter().join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (registers, programs) = input.split("\n\n").collect_tuple().unwrap();
    let mut register_lines = registers.lines();
    register_lines.next();
    let reg_b_def = register_lines
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();
    let reg_c_def = register_lines
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();

    let program: Vec<u32> = programs.split(": ").collect::<Vec<_>>()[1]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .map(|mut n| {
            n += 111200000;
            n
        })
        .into_par_iter()
        .try_for_each(|start| {
            let mut reg_a_iter = start;
            loop {
                let mut pointer = 0;
                let mut reg_a = reg_a_iter;
                let mut reg_b = reg_b_def;
                let mut reg_c = reg_c_def;

                let mut outputs = vec![];

                while let Some(opcode) = program.get(pointer) {
                    if pointer + 1 >= program.len() {
                        break;
                    }
                    match opcode {
                        0 => {
                            // adv
                            let combo = program[pointer + 1];
                            let combo_value = match combo {
                                0 => 0,
                                1 => 1,
                                2 => 2,
                                3 => 3,
                                4 => reg_a,
                                5 => reg_b,
                                6 => reg_c,
                                _ => panic!("invalid combo"),
                            };
                            reg_a /= 2u32.pow(combo_value);

                            pointer += 2;
                        }
                        1 => {
                            let literal = program[pointer + 1];
                            reg_b ^= literal;

                            pointer += 2;
                        }
                        2 => {
                            let combo = program[pointer + 1];
                            let combo_value = match combo {
                                0 => 0,
                                1 => 1,
                                2 => 2,
                                3 => 3,
                                4 => reg_a,
                                5 => reg_b,
                                6 => reg_c,
                                _ => panic!("invalid combo"),
                            };
                            reg_b = combo_value.rem_euclid(8);

                            pointer += 2;
                        }
                        3 => {
                            if reg_a == 0 {
                                pointer += 2;
                            } else {
                                let literal = program[pointer + 1];
                                pointer = literal as usize;
                            }
                        }
                        4 => {
                            reg_b ^= reg_c;

                            pointer += 2;
                        }
                        5 => {
                            let combo = program[pointer + 1];
                            let combo_value = match combo {
                                0 => 0,
                                1 => 1,
                                2 => 2,
                                3 => 3,
                                4 => reg_a,
                                5 => reg_b,
                                6 => reg_c,
                                _ => panic!("invalid combo"),
                            };
                            outputs.push(combo_value.rem_euclid(8));

                            pointer += 2;
                        }
                        6 => {
                            // bdv
                            let combo = program[pointer + 1];
                            let combo_value = match combo {
                                0 => 0,
                                1 => 1,
                                2 => 2,
                                3 => 3,
                                4 => reg_a,
                                5 => reg_b,
                                6 => reg_c,
                                _ => panic!("invalid combo"),
                            };
                            reg_b = reg_a / 2u32.pow(combo_value);

                            pointer += 2;
                        }
                        7 => {
                            // cdv
                            let combo = program[pointer + 1];
                            let combo_value = match combo {
                                0 => 0,
                                1 => 1,
                                2 => 2,
                                3 => 3,
                                4 => reg_a,
                                5 => reg_b,
                                6 => reg_c,
                                _ => panic!("invalid combo"),
                            };
                            reg_c = reg_a / 2u32.pow(combo_value);

                            pointer += 2;
                        }
                        _ => panic!("invalid opcode"),
                    }
                }

                if outputs == program {
                    println!("tried reg_a == {}", reg_a_iter);
                    return None;
                } else {
                    if reg_a_iter % 1000000 == 0 {
                        println!("tried reg_a == {}", reg_a_iter);
                    }
                    reg_a_iter += 10;
                }
            }
        });

    // Some(reg_a_iter)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}