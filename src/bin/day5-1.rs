use std::io;

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let (starting_positions, instruction_set) = stdin.split_once("\n\n").unwrap();
    let (stacks_str, platforms) = starting_positions.rsplit_once('\n').unwrap();
    let columns_count = platforms
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut columns = vec![Vec::<char>::new(); columns_count];

    stacks_str.lines().rev().for_each(|line| {
        let line_to_chunk = line.chars().collect::<Vec<char>>();
        for (i, chunk) in line_to_chunk.chunks(4).into_iter().enumerate() {
            let container = chunk[1];
            if container.is_alphabetic() {
                columns[i].push(container);
            }
        }
    });

    instruction_set.lines().for_each(|line| {
        let inst: Vec<&str> = line.split_whitespace().collect();
        let move_count = inst[1].parse::<usize>().unwrap();
        let from = inst[3].parse::<usize>().unwrap() - 1;
        let to = inst[5].parse::<usize>().unwrap() - 1;

        for _ in 0..move_count {
            if let Some(to_move) = columns[from].pop() {
                columns[to].push(to_move);
            }
        }
    });

    let top_columns: String = columns.iter().filter_map(|column| column.last()).collect();

    println!("{:?}", top_columns);

    Ok(())
}
