use std::ops::{Deref, DerefMut};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, char, digit1, line_ending},
        is_alphabetic,
    },
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

pub fn parse(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let (rest, stacks_and_moves) = pair(stacks, moves)(input).expect("Parsing error");
    assert!(rest.is_empty());

    stacks_and_moves
}

fn stacks(input: &str) -> IResult<&str, Vec<Stack>> {
    let stack_positions = separated_list1(char(' '), stack_position);
    let stack_matrix = separated_list1(line_ending, stack_positions);

    let stacks = stack_matrix.map(|matrix| {
        let num_stacks = matrix[0].len();
        let mut stacks = vec![Stack::new(); num_stacks];

        for stack_line in matrix.iter().rev() {
            for (i, stack_pos) in stack_line.iter().enumerate() {
                if let StackPosition::Crate(c) = stack_pos {
                    stacks[i].push(*c);
                }
            }
        }

        stacks
    });

    let terminated_stacks = terminated(stacks, line_ending);

    let padded_number = delimited(char(' '), digit1, char(' '));

    let stack_numbers = separated_list1(char(' '), padded_number);
    let stack_number_line = terminated(stack_numbers, line_ending);

    let stacks_with_numbers_dropped = terminated(terminated_stacks, stack_number_line);

    terminated(stacks_with_numbers_dropped, line_ending)(input)
}

fn stack_position(input: &str) -> IResult<&str, StackPosition> {
    let empty = tag("   ");
    let crate_ = delimited(char('['), alpha1, char(']'));

    alt((empty, crate_)).map(|s: &str| s.into()).parse(input)
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    let move_line = terminated(move_, line_ending);

    many1(move_line)(input)
}

fn move_(input: &str) -> IResult<&str, Move> {
    let move_tag = tag::<_, &str, _>("move ");
    let from_tag = tag(" from ");
    let to_tag = tag(" to ");

    let amount_value = preceded(move_tag, digit1);
    let from_value = preceded(from_tag, digit1);
    let to_value = preceded(to_tag, digit1);

    let whole_move = tuple((amount_value, from_value, to_value));

    whole_move
        .map(|(amount, from, to)| Move {
            index_from: from.parse::<usize>().unwrap() - 1, // stacks are counted 1-based in input
            index_to: to.parse::<usize>().unwrap() - 1,     // stacks are counted 1-based in input
            amount: amount.parse().unwrap(),
        })
        .parse(input)
}

enum StackPosition {
    Empty,
    Crate(char),
}

impl From<&str> for StackPosition {
    fn from(s: &str) -> Self {
        if s == "   " {
            return StackPosition::Empty;
        };

        if s.len() != 1 || !s.is_ascii() {
            panic!("Expected single ASCII character crate names")
        }

        let c = s.chars().next().unwrap();

        if !is_alphabetic(c as u8) {
            panic!("Expected alphabetic crate names")
        }

        StackPosition::Crate(c)
    }
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub elements: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }
}

impl Deref for Stack {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

#[derive(Debug)]
pub struct Move {
    pub index_from: usize,
    pub index_to: usize,
    pub amount: usize,
}
