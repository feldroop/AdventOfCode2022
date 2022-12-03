use std::str::FromStr;

fn main() {
    let games = std::fs::read_to_string("input_data/puzzle2_rps_strategy.txt").unwrap();
    let total_score: u32 = games
        .lines()
        // .map(|line| GameWithMove::from_str(line).unwrap())
        .map(|line| GameWithOutcome::from_str(line).unwrap())
        .map(|game| game.evaluate())
        .sum();
    println!("Total score: {total_score}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_abc(value: char) -> Self {
        use Move::*;

        match value {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("Unknown rock, paper, scissors move (from abc)"),
        }
    }

    fn from_xyz(value: char) -> Self {
        use Move::*;

        match value {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!("Unknown rock, paper, scissors move (from xyz)"),
        }
    }
}

struct GameWithMove {
    opponent_move: Move,
    player_move: Move,
}

impl FromStr for GameWithMove {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err("String to turn into game not in the correct format");
        }

        let chars: Vec<_> = s.chars().collect();
        let opponent_move = chars[0];
        let space = chars[1];
        let player_move = chars[2];

        if space != ' ' {
            return Err("String to turn into game not in the correct format");
        }

        Ok(GameWithMove {
            opponent_move: Move::from_abc(opponent_move),
            player_move: Move::from_xyz(player_move),
        })
    }
}

impl GameWithMove {
    fn evaluate(&self) -> u32 {
        use Move::*;

        let outcome_score = match (self.player_move, self.opponent_move) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6, // win
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3, // draw
            _ => 0,                                                    // loss
        };

        let move_score = match self.player_move {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        outcome_score + move_score
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn from_xyz(value: char) -> Self {
        use Outcome::*;

        match value {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Unknown game outcome (from xyz)"),
        }
    }
}

struct GameWithOutcome {
    opponent_move: Move,
    desired_outcome: Outcome,
}

impl FromStr for GameWithOutcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err("String to turn into game not in the correct format");
        }

        let chars: Vec<_> = s.chars().collect();
        let opponent_move = chars[0];
        let space = chars[1];
        let desired_outcome = chars[2];

        if space != ' ' {
            return Err("String to turn into game not in the correct format");
        }

        Ok(GameWithOutcome {
            opponent_move: Move::from_abc(opponent_move),
            desired_outcome: Outcome::from_xyz(desired_outcome),
        })
    }
}

impl GameWithOutcome {
    fn evaluate(&self) -> u32 {
        use Move::*;
        use Outcome::*;

        let player_move = match (self.opponent_move, self.desired_outcome) {
            (Rock, Win) | (Paper, Draw) | (Scissors, Loss) => Paper,
            (Paper, Win) | (Scissors, Draw) | (Rock, Loss) => Scissors,
            _ => Rock,
        };

        GameWithMove {
            opponent_move: self.opponent_move,
            player_move,
        }
        .evaluate()
    }
}
