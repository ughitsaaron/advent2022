use std::io;

fn outcome_score(opponent: &str, player: &str) -> i16 {
    // A = Rock, X = Rock
    // B = Paper, Y = Paper
    // C = Scissors, Z = Scissors
    match [opponent, player] {
        ["B", "X"] | ["C", "Y"] | ["A", "Z"] => 0,
        ["C", "X"] | ["A", "Y"] | ["B", "Z"] => 6,
        _ => 3,
    }
}

fn player_score(i: &str) -> i16 {
    match i {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let guide = stdin
        .split_terminator('\n')
        .map(|s| {
            let round = s.split(' ').collect::<Vec<&str>>();

            if let [player_one, player_two] = round.as_slice() {
                player_score(player_one) + outcome_score(player_one, player_two)
            } else {
                0
            }
        })
        .collect::<Vec<i16>>();
    let total = guide.iter();
    println!("{:#?}", total.sum::<i16>());
    Ok(())
}
