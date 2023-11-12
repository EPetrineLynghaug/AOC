use std::collections::HashSet;

use crate::common::{parse_directions, Direction, Position};

pub fn solution(input: &str) -> usize {
    let directions = parse_directions(input)
        .expect("Coudld'n parse the directions")
        .1;

    let mut santa_position = Position { x: 0, y: 0 };
    let mut robot_position = Position { x: 0, y: 0 };
    let mut position_history = HashSet::from([santa_position]);

    directions.iter().step_by(2).for_each(|dir| {
        match dir {
            Direction::Up => santa_position.y += 1,
            Direction::Down => santa_position.y -= 1,
            Direction::Left => santa_position.x -= 1,
            Direction::Right => santa_position.x += 1,
        };
        position_history.insert(santa_position);
    });

    directions.iter().skip(1).step_by(2).for_each(|dir| {
        match dir {
            Direction::Up => robot_position.y += 1,
            Direction::Down => robot_position.y -= 1,
            Direction::Left => robot_position.x -= 1,
            Direction::Right => robot_position.x += 1,
        };
        position_history.insert(robot_position);
    });

    position_history.len()
}
