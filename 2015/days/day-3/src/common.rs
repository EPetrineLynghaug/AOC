use nom::{branch::alt, bytes::complete::tag, multi::many0, IResult};

#[derive(Debug,PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((tag("^"), alt((tag("v"), alt((tag(">"), tag("<")))))))(input)?;
    let direction = match direction {
        "^" => Direction::Up,
        "v" => Direction::Down,
        ">" => Direction::Right,
        "<" => Direction::Left,
        _ => panic!("Bad direction input"),
    };

    Ok((input, direction))
}

/*Alt er en forkortelse på veksling.  */
/*tag- finner du den tingen er det gyldig.  */
/*Enum er bare en liste men kan velge mellom en av verdiene. må alltid begyne med stor bokstav */

pub fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = many0(parse_direction)(input)?;

    Ok((input, directions))
}
