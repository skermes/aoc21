use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use crate::aoc_error::AocError;

pub const NAME: &str = "Dirac Dice";

// Starting positions as taken from input - 1-indexed
const P1_START: usize = 7;
const P2_START: usize = 3;
// Example inputs
// const P1_START: usize = 4;
// const P2_START: usize = 8;

#[derive(Debug, Copy, Clone)]
struct Game {
    // 0-indexed position on the board
    position: usize,
    score: usize,
    turn: usize
}

#[derive(Debug)]
struct MultiverseMemo {
    states: HashMap<(Game, Game, bool), (usize, usize)>
}

impl Hash for Game {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        // Specifically not hashing turn here because we don't care about it
        // for part 2
        self.position.hash(state);
        self.score.hash(state);
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Game) -> bool {
        self.position == other.position && self.score == other.score
    }
}

impl Eq for Game {}

impl Game {
    fn new(position: usize) -> Self {
        Game { position, score: 0, turn: 0 }
    }

    fn turn(&self, roll: usize) -> Self {
        let new_pos = (self.position + roll) % 10;
        Game {
            position: new_pos,
            // Add one here to shift 0-indexed positions into 1-indexed scores
            score: self.score + new_pos + 1,
            turn: self.turn + 1
        }
    }

    fn won_part1(&self) -> bool {
        self.score >= 1000
    }

    fn won_part2(&self) -> bool {
        self.score >= 21
    }
}

impl MultiverseMemo {
    fn new() -> Self {
        MultiverseMemo { states: HashMap::new() }
    }

    fn get(&self, player_1: Game, player_2: Game, player_1_turn: bool) -> Option<&(usize, usize)> {
        self.states.get(&(player_1, player_2, player_1_turn))
    }

    fn insert(&mut self, player_1: Game, player_2: Game, player_1_turn: bool, player_1_wins: usize, player_2_wins: usize) {
        self.states.insert(
            (player_1, player_2, player_1_turn),
            (player_1_wins, player_2_wins)
        );
    }
}

// (roll_value, number_of_combinations)
const ROLLS: [(usize, usize); 7] = [
    (3, 1), // 1, 1, 1
    (4, 3), // 1, 1, 2
    (5, 6), // 1, 1, 3; 1, 2, 2
    (6, 7), // 1, 2, 3; 2, 2, 2
    (7, 6), // 2, 2, 3; 3, 3, 1
    (8, 3), // 3, 3, 2
    (9, 1)  // 3, 3, 3
];

fn play_multiversal(memo: &mut MultiverseMemo, player_1: Game, player_2: Game, player_1_turn: bool) -> (usize, usize) {
    if let Some(win_counts) = memo.get(player_1, player_2, player_1_turn) {
        *win_counts
    } else if player_1.won_part2() {
        (1, 0)
    } else if player_2.won_part2() {
        (0, 1)
    } else {
        let mut win_counts = (0, 0);

        for (roll, multiplier) in ROLLS {
            let (new_p1, new_p2) = if player_1_turn {
                (player_1.turn(roll), player_2)
            } else {
                (player_1, player_2.turn(roll))
            };
            let (p1_wins, p2_wins) = play_multiversal(memo, new_p1, new_p2, !player_1_turn);
            win_counts.0 += p1_wins * multiplier;
            win_counts.1 += p2_wins * multiplier;
        }

        memo.insert(player_1, player_2, player_1_turn, win_counts.0, win_counts.1);
        win_counts
    }
}

pub fn part_one(_input: &str) -> Result<String, AocError> {
    let p1_turns = (0..usize::MAX)
        .step_by(2)
        .map(|turn| turn * 9 + 6)
        .scan(Game::new(P1_START - 1), |game, roll| {
            *game = game.turn(roll);
            Some(*game)
        })
        .take_while(|game| !game.won_part1());

    let p2_turns = (1..usize::MAX)
        .step_by(2)
        .map(|turn| turn * 9 + 6)
        .scan(Game::new(P2_START - 1), |game, roll| {
            *game = game.turn(roll);
            Some(*game)
        })
        .take_while(|game| !game.won_part1());

    let (almost_last_p1, almost_last_p2) = p1_turns
        .zip(p2_turns)
        .last()
        .unwrap();

    // Assume whoever has the higher score is winning on this turn
    if almost_last_p1.score > almost_last_p2.score {
        let loser_score = almost_last_p2.score;
        let total_rolls = (almost_last_p1.turn + almost_last_p2.turn + 1) * 3;
        Ok((loser_score * total_rolls).to_string())
    } else {
        let last_p1 = almost_last_p1.turn((almost_last_p1.turn * 2 + 1) * 9 + 6);
        let loser_score = last_p1.score;
        let total_rolls = (last_p1.turn + almost_last_p2.turn + 1) * 3;
        Ok((loser_score * total_rolls).to_string())
    }
}

pub fn part_two(_input: &str) -> Result<String, AocError> {
    let player_1 = Game::new(P1_START - 1);
    let player_2 = Game::new(P2_START - 1);
    let (p1_wins, p2_wins) = play_multiversal(&mut MultiverseMemo::new(), player_1, player_2, true);

    if p1_wins > p2_wins {
        Ok(p1_wins.to_string())
    } else {
        Ok(p2_wins.to_string())
    }
}
