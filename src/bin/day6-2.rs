use std::env;
use std::io;

const FALLBACK_WINDOW_SIZE: usize = 14;

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let args: Vec<String> = env::args().collect();

    let window_size = if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(n) => n,
            _ => FALLBACK_WINDOW_SIZE,
        }
    } else {
        FALLBACK_WINDOW_SIZE
    };

    let mut windows = stdin.as_bytes().windows(window_size);
    let position = windows
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(i, c)| !window[..i].contains(c))
        })
        .unwrap();

    println!("{:?}", position + window_size);

    Ok(())
}
