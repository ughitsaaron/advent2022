use std::io;
use std::ops::RangeInclusive;

fn range_includes(ranges: &[RangeInclusive<&i32>; 2]) -> bool {
    let [compare_1, compare_2] = ranges;
    compare_1.contains(&compare_2.start()) && compare_1.contains(&compare_2.end())
        || compare_2.contains(&compare_1.start()) && compare_2.contains(&compare_1.end())
}

fn make_range_pair(chunk: &[i32]) -> [RangeInclusive<&i32>; 2] {
    if let [head_start, head_end, tail_start, tail_end] = chunk {
        let range_1 = RangeInclusive::new(head_start, head_end);
        let range_2 = RangeInclusive::new(tail_start, tail_end);

        [range_1, range_2]
    } else {
        panic!()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let nums: Vec<i32> = stdin
        .split_terminator(&['\n', ',', '-'][..])
        .map(|n| n.parse().unwrap())
        .collect();
    let chunks: Vec<[RangeInclusive<&i32>; 2]> =
        nums.chunks(4).map(|chunk| make_range_pair(chunk)).collect();
    let total_overlaps = chunks
        .iter()
        .filter(|&ranges| range_includes(ranges))
        .count();
    println!("{:?}", total_overlaps);
    Ok(())
}
