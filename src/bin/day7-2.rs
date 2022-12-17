use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

const TOTAL_VOLUME: u32 = 70_000_000;
const UPDATE_SIZE: u32 = 30_000_000;

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

    let root_dir = sizes.get(&PathBuf::from("/")).unwrap();
    let available_storage = TOTAL_VOLUME - root_dir;
    let minimum_to_delete: u32 = sizes
        .into_values()
        .filter(|size| available_storage + size >= UPDATE_SIZE)
        .min()
        .unwrap();

    println!("{:?}", minimum_to_delete);

    Ok(())
}
