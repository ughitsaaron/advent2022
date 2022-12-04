use std::io;

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let list = stdin.split("\n\n").map(|s| {
        s.split_terminator('\n')
            .map(|n| n.parse::<i32>().unwrap())
            .sum::<i32>()
    });

    let max = list.max().unwrap();

    println!("{max}");

    Ok(())
}
