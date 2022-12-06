use std::cmp::{max, min};
use std::fs;
use std::ops::RangeInclusive;

use nom::character::complete::{char, digit1, line_ending};
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};

fn main() {
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let range_pairs = parse_input(&input);

    let num_fully_covering = range_pairs
        .iter()
        .filter(|range_pair| range_pair.one_range_fully_covers_other())
        .count();

    println!("Number of range pairs where one fully covers the other: {num_fully_covering}");

    let num_overlapping = range_pairs
        .iter()
        .filter(|range_pair| range_pair.ranges_overlap())
        .count();

    println!("Number of range pairs which are overlapping: {num_overlapping}");
}

fn parse_input(input: &str) -> Vec<RangePair> {
    fn number(input: &str) -> IResult<&str, u32> {
        digit1
            .map(|digits: &str| digits.parse().expect("Failed to parse number"))
            .parse(input)
    }

    fn range(input: &str) -> IResult<&str, SectionRange> {
        separated_pair(number, char('-'), number)
            .map(|(start, end)| start..=end)
            .parse(input)
    }

    let range_pair = separated_pair(range, char(','), range)
        .map(|(range1, range2)| RangePair { range1, range2 });
    let range_pair_line = terminated(range_pair, line_ending);
    let (rest, range_pairs) = many1(range_pair_line).parse(input).unwrap();

    assert!(rest.is_empty());
    range_pairs
}

type SectionRange = RangeInclusive<u32>;

struct RangePair {
    range1: SectionRange,
    range2: SectionRange,
}

impl RangePair {
    fn one_range_fully_covers_other(&self) -> bool {
        range_fully_covers_other(&self.range1, &self.range2)
            || range_fully_covers_other(&self.range2, &self.range1)
    }

    fn ranges_overlap(&self) -> bool {
        max(self.range1.start(), self.range2.start()) <= min(self.range1.end(), self.range2.end())
    }
}

fn range_fully_covers_other(covering_range: &SectionRange, covered_range: &SectionRange) -> bool {
    covering_range.start() <= covered_range.start() && covering_range.end() >= covered_range.end()
}
