use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut counts = [0u64;9];
    for n in input.split(",").map(|x|x.parse::<usize>().unwrap()) {
        counts[n] += 1;
    }
    for _ in 0..80 {
        counts = [
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[0] + counts[7],
            counts[8],
            counts[0]
        ];
    }
    println!("Lanternfish after 80 iterations: {}", counts.iter().sum::<u64>());
    for _ in 80..256 {
        counts = [
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[0] + counts[7],
            counts[8],
            counts[0]
        ];
    }
    println!("Lanternfish after 256 iterations: {}", counts.iter().sum::<u64>());
}
