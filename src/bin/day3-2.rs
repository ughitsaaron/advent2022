use std::io;

fn match_alpha_place_value(c: char) -> i16 {
    let mut alpha = ('a'..='z').collect::<Vec<_>>();
    alpha.extend(&('A'..='Z').collect::<Vec<_>>());

    let value = alpha.iter().position(|&v| c == v).unwrap();
    match i16::try_from(value).ok() {
        Some(n) => n + 1,
        None => panic!(),
    }
}

fn find_matched_char(chunk: &[&str]) -> char {
    let [head, tail @ ..]: [&str; 3] = chunk.try_into().ok().unwrap();
    let match_chars = head
        .chars()
        .find(|&c| tail.into_iter().all(|s| s.contains(c)));

    match match_chars {
        Some(c) => c,
        None => panic!(),
    }
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let stdin_lines = stdin.lines().collect::<Vec<&str>>();
    let chars_sum = stdin_lines
        .chunks(3)
        .map(|chunk| match_alpha_place_value(find_matched_char(chunk)))
        .sum::<i16>();

    println!("{:?}", chars_sum);
    Ok(())
}
