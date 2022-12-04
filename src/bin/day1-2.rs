use std::io;

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let mut list: Vec<i32> = stdin
        .split("\n\n")
        .map(|s| {
            s.split_terminator('\n')
                .map(|n| n.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    list.sort_by(|a, b| b.cmp(a));
    let total = list[0..3].iter();

    println!("{:#?}", total.sum::<i32>());

    Ok(())
}
