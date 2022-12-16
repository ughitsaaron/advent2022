use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_ascii_whitespace().collect();

        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for index in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=index]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }

    // let total_usage: u32 = sizes.into_values().filter(|size| *size <= 100_000).sum();
    let total_usage: u32 = sizes
        .values()
        .filter(|&value| value <= &(100_000 as u32))
        .sum();

    println!("{:?}", total_usage);

    Ok(())
}
