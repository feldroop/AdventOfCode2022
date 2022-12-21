mod input;

use std::{collections::HashSet, fs};

use input::{Direction, DirectionMoves};

use itertools::Itertools;

fn main() {
    let input_string = fs::read_to_string("input/day9.txt").unwrap();
    let all_direction_moves = input::parse(&input_string);

    simulate_rope(2, &all_direction_moves);
    simulate_rope(10, &all_direction_moves);
}

fn simulate_rope(rope_length: usize, all_direction_moves: &[DirectionMoves]) {
    let mut short_rope = Rope::with_knots_on_origin(rope_length);
    let mut tail_position_history = HashSet::new();

    for direction_moves in all_direction_moves {
        for _ in 0..direction_moves.amount {
            short_rope.move_head_and_follow_with_knots(&direction_moves.direction);
            tail_position_history.insert(short_rope.tail_position());
        }
    }

    println!(
        "Number of tail positions with rope of length {}: {}",
        rope_length,
        tail_position_history.len(),
    );
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn origin() -> Self {
        Position { x: 0, y: 0 }
    }

    fn is_touching(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn move_to_direction(&mut self, direction: &Direction) {
        use Direction::*;
        match direction {
            Left => self.x -= 1,
            Right => self.x += 1,
            Up => self.y += 1,
            Down => self.y -= 1,
        }
    }

    fn collect_differing_directions_to(&self, other: &Position) -> Vec<Direction> {
        let mut directions = Vec::new();

        use std::cmp::Ordering::*;
        use Direction::*;

        match self.x.cmp(&other.x) {
            Less => directions.push(Right),
            Greater => directions.push(Left),
            Equal => (),
        }

        match self.y.cmp(&other.y) {
            Less => directions.push(Up),
            Greater => directions.push(Down),
            Equal => (),
        }

        directions
    }

    fn move_towards(&mut self, other: &Position) {
        if self.is_touching(other) {
            return;
        }

        for direction in self.collect_differing_directions_to(other) {
            self.move_to_direction(&direction);
        }
    }
}

#[derive(Debug)]
struct Rope {
    knot_positions: Vec<Position>,
}

impl Rope {
    fn with_knots_on_origin(num_knots: usize) -> Self {
        Rope {
            knot_positions: vec![Position::origin(); num_knots],
        }
    }

    fn tail_position(&self) -> Position {
        *self
            .knot_positions
            .last()
            .expect("Rope shouldn't have 0 knots.")
    }

    fn move_head_and_follow_with_knots(&mut self, direction: &Direction) {
        self.knot_positions
            .first_mut()
            .expect("Rope shouldn't have 0 knots.")
            .move_to_direction(direction);

        for (already_moved_idx, to_be_moved_idx) in (0..self.knot_positions.len()).tuple_windows() {
            let already_moved_knot = self.knot_positions[already_moved_idx];
            self.knot_positions[to_be_moved_idx].move_towards(&already_moved_knot);
        }
    }
}

mod tests {
    use super::Position;

    #[allow(unused)]
    fn test_move_template(
        head_x: i32,
        head_y: i32,
        before_tail_x: i32,
        before_tail_y: i32,
        after_tail_x: i32,
        after_tail_y: i32,
    ) {
        let head = Position {
            x: head_x,
            y: head_y,
        };
        let mut tail = Position {
            x: before_tail_x,
            y: before_tail_y,
        };

        tail.move_towards(&head);
        assert!(tail.is_touching(&head));
        assert_eq!(
            tail,
            Position {
                x: after_tail_x,
                y: after_tail_y
            }
        );
    }

    #[test]
    fn test_move_cases() {
        test_move_template(2, 0, 0, 0, 1, 0);
        test_move_template(0, -2, 0, 0, 0, -1);
        test_move_template(2, 1, 0, 0, 1, 1);
        test_move_template(-1, -2, 0, 0, -1, -1);
        test_move_template(1, 0, 0, 0, 0, 0);
    }
}
