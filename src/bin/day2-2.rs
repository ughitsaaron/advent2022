use core::panic;
use std::{io, borrow::Borrow};

enum Shape {
    Rock,
    Paper,
    Scissors,
    ShouldWin,
    ShouldLose,
    ShouldDraw
}

enum Outcome {
    Win,
    Lose,
    Draw
}

fn match_move(code: &str) -> Shape {
    match code {
        "A"  => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        "X" => Shape::ShouldLose,
        "Y" => Shape::ShouldDraw,
        "Z" => Shape::ShouldWin,
        _ => panic!()
    }
}

fn flip_move(round: [&Shape; 2]) -> [Shape; 2] {
    match round {
        [Shape::Rock, Shape::ShouldWin] => [Shape::Rock, Shape::Paper],
        [Shape::Paper, Shape::ShouldWin] => [Shape::Paper, Shape::Scissors],
        [Shape::Scissors, Shape::ShouldWin] => [Shape::Scissors, Shape::Rock],
        [Shape::Rock, Shape::ShouldLose] => [Shape::Rock, Shape::Scissors],
        [Shape::Paper, Shape::ShouldLose] => [Shape::Paper, Shape::Rock],
        [Shape::Scissors, Shape::ShouldLose] => [Shape::Scissors, Shape::Paper],
        [opponent_move, Shape::ShouldDraw] => [opponent_move, opponent_move],
        _ => panic!()
    }
}

fn outcome(round: [&Shape; 2]) -> Outcome {
    match round {
        [Shape::Rock, Shape::Paper] => Outcome::Win,
        [Shape::Rock, Shape::Scissors] => Outcome::Lose,
        [Shape::Paper, Shape::Scissors] => Outcome::Win,
        [Shape::Paper, Shape::Rock] => Outcome::Lose,
        [Shape::Scissors, Shape::Rock] => Outcome::Win,
        [Shape::Scissors, Shape::Paper] => Outcome::Lose,
        _ => Outcome::Draw
    }
}

fn outcome_score(outcome: Outcome) -> i16 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3
    }
}

fn move_score(shape: &Shape) -> i16 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let total = stdin
        .split_terminator('\n')
        .map(|s| {
            let round = s.split(' ').map(|code| match_move(code)).collect::<Vec<Shape>>();
            if let [opponent_move, player_move] = round.as_slice() {
                let outcome = outcome(flip_move([opponent_move, player_move]));
                outcome_score(outcome) + move_score(player_move)
            } else {
                0
            }
        }).sum::<i16>();
    println!("{}", total);
    Ok(())
}
