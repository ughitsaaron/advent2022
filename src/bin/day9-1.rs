use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let input: Vec<(&str, u32)> = stdin
        .lines()
        .map(|line| {
            let (dir, amount) = line.split_once(' ').unwrap();
            let amount: u32 = amount.parse().unwrap();
            (dir, amount)
        })
        .collect();

    let mut tail_visits = HashSet::new();

    let (mut head_x, mut head_y) = (0, 0);
    let (mut tail_x, mut tail_y) = (0, 0);
    tail_visits.insert(format!("{tail_x}, {tail_y}"));

    for (dir, amount) in input {
        for _ in 0..amount {
            match dir {
                "U" => head_y -= 1,
                "D" => head_y += 1,
                "L" => head_x -= 1,
                "R" => head_x += 1,
                _ => panic!("how did i get here?"),
            }

            let diff_x: i32 = head_x - tail_x;
            let diff_y: i32 = head_y - tail_y;

            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                tail_x += diff_x.signum();
                tail_y += diff_y.signum();
                tail_visits.insert(format!("{tail_x}, {tail_y}"));
            }
        }
    }

    println!("{:?}", tail_visits.len());

    Ok(())
}
