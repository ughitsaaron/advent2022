use std::io;

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let mut windows = stdin.as_bytes().windows(4);
    let position = windows
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(i, c)| !window[..i].contains(c))
        })
        .unwrap();

    println!("{:?}", position + 4);

    Ok(())
}
