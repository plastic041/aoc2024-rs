use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
// use std::io::{self, Write as IoWrite};
// use std::{fmt::Write, io::stdin};
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

impl Robot {
    fn elapse(&self, seconds: u32, width: i32, height: i32) -> IVec2 {
        let next_x = (self.position.x + self.velocity.x * seconds as i32).rem_euclid(width);
        let next_y = (self.position.y + self.velocity.y * seconds as i32).rem_euclid(height);

        IVec2::new(next_x, next_y)
    }
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<i32>()
    })(input)?;

    Ok((i, number))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (_, x, _, y, _, vx, _, vy)) = tuple((
        tag("p="),
        parse_i32,
        tag(","),
        parse_i32,
        tag(" v="),
        parse_i32,
        tag(","),
        parse_i32,
    ))(input)?;

    Ok((
        input,
        Robot {
            position: IVec2::new(x, y),
            velocity: IVec2::new(vx, vy),
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, robots) = separated_list1(line_ending, parse_robot)(input).expect("Parse failed");
    let width = 101;
    let height = 103;

    let mut quadrant_counts = [0; 4];
    robots.iter().for_each(|robot| {
        let position = robot.elapse(100, width, height);

        if position.x != width / 2 && position.y != height / 2 {
            match (position.x < width / 2, position.y < height / 2) {
                (true, true) => quadrant_counts[0] += 1,
                (false, true) => quadrant_counts[1] += 1,
                (true, false) => quadrant_counts[2] += 1,
                (false, false) => quadrant_counts[3] += 1,
            }
        }
    });

    Some(quadrant_counts.iter().product::<u32>())
}

pub fn part_two(_input: &str) -> Option<u32> {
    // let stdin = stdin();
    // let mut stdout = io::stdout().into_raw_mode().unwrap();

    // let (_, robots) = separated_list1(line_ending, parse_robot)(input).expect("Parse failed");
    // let width: i32 = 101;
    // let height: i32 = 103;

    // let capacity = (width * height) as usize + height as usize;

    // let mut i = 7770;
    // for c in stdin.keys() {
    //     write!(
    //         stdout,
    //         "{}{}",
    //         termion::cursor::Goto(1, 1),
    //         termion::clear::All
    //     )
    //     .unwrap();
    //     stdout.flush().unwrap();

    //     let mut chars = vec![vec![" "; width as usize]; height as usize];

    //     robots.iter().for_each(|robot| {
    //         let position = robot.elapse(i, width, height);
    //         chars[position.y as usize][position.x as usize] = "@";
    //     });

    //     let mut buffer = String::with_capacity(capacity);
    //     chars.reverse();
    //     chars.iter().for_each(|row| {
    //         row.iter().for_each(|char| {
    //             write!(buffer, "{}", char).unwrap();
    //         });
    //         write!(buffer, "&&&&&&&&").unwrap();
    //         write!(buffer, "\n\r").unwrap();
    //     });
    //     write!(buffer, "-------------------------------------").unwrap();
    //     stdout.write_all(buffer.as_bytes()).unwrap();
    //     stdout.flush().unwrap();

    //     println!("{}", i);
    //     match c.unwrap() {
    //         Key::Left => i -= 1,
    //         Key::Right => i += 1,
    //         Key::Alt('h') => break,
    //         _ => (),
    //     }
    // }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_mov() {
        let robot = Robot {
            position: IVec2::new(2, 4),
            velocity: IVec2::new(2, -3),
        };

        assert_eq!(robot.elapse(1, 11, 7), IVec2::new(4, 1));
        assert_eq!(robot.elapse(4, 11, 7), IVec2::new(10, 6));
        assert_eq!(robot.elapse(5, 11, 7), IVec2::new(1, 3));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
