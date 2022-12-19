use itertools::Itertools;
use std::io;

fn directions(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column: Vec<u32> = grid.iter().map(|row| row[x]).collect();

    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let grid = stdin
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let visible_count: usize = (1..grid.len() - 1)
        // Is it possible to find a way to do this without
        // Itertools or `cartesian_product`?
        .cartesian_product(1..grid.len() - 1)
        .map(|(y, x)| {
            let height = grid[y][x];
            directions(&grid, x, y)
                .iter()
                .map(|direction| direction.iter().all(|h| *h < height))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count()
        + (grid.len() - 1) * 4;

    println!("{:?}", visible_count);

    Ok(())
}
