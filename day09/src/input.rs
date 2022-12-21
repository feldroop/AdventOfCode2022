use nom::{
    character::complete::{char, digit1, line_ending, one_of},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

pub fn parse(input: &str) -> Vec<DirectionMoves> {
    let moves_line = terminated(moves, line_ending);
    let (rest, moves) = many1(moves_line)(input).expect("Input should conform to example");
    assert!(rest.is_empty());
    moves
}

fn direction(input: &str) -> IResult<&str, Direction> {
    use Direction::*;

    one_of("LRUD")
        .map(|c| match c {
            'L' => Left,
            'R' => Right,
            'U' => Up,
            'D' => Down,
            _ => unreachable!(),
        })
        .parse(input)
}

fn moves(input: &str) -> IResult<&str, DirectionMoves> {
    separated_pair(direction, char(' '), digit1)
        .map(|(direction, amount_digits)| DirectionMoves {
            direction,
            amount: amount_digits.parse().unwrap(),
        })
        .parse(input)
}

pub struct DirectionMoves {
    pub direction: Direction,
    pub amount: u32,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
