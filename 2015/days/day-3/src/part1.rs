use std::collections::HashSet;

use crate::common::{parse_directions, Direction, Position};

pub fn solution(input: &str) -> usize {
    let directions = parse_directions(input)
        .expect("Coudld'n parse the directions")
        .1;

    let mut position = Position { x: 0, y: 0 };
    let mut position_history = HashSet::from([position]);

    directions.into_iter().for_each(|dir| {
        match dir {
            Direction::Up => position.y += 1,
            Direction::Down => position.y -= 1,
            Direction::Left => position.x -= 1,
            Direction::Right => position.x += 1,
        };
        position_history.insert(position);
    });

    position_history.len()
}
