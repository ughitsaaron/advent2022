use std::io;

fn match_alpha_place_value(c: char) -> i16 {
  let mut alpha = ('a'..='z').collect::<Vec<_>>();
  alpha.extend(&('A'..='Z').collect::<Vec<_>>());

  let value = alpha.iter().position(|&v| c == v).unwrap();
  match i16::try_from(value).ok() {
    Some(n) => n + 1,
    None => panic!()
  }
}

fn find_matched_char(source: &str) -> char {
  let (head, tail) = source.split_at(source.chars().count() / 2);

  match head.chars().find(|c| tail.contains(|t| &t == c)) {
    Some(c) => c,
    None => panic!()
  }
}


fn main() -> io::Result<()> {
  let stdin = io::read_to_string(io::stdin())?;
  let chars_sum = stdin.lines().map(|l| match_alpha_place_value(find_matched_char(l))).sum::<i16>();
  println!("{:?}", chars_sum);
  Ok(())
}
