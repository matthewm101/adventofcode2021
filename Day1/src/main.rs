use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut measurements: Vec<u64> = Vec::new();
    for line in input.lines() {
        measurements.push(line.parse().unwrap());
    }
    let mut counted_diffs1 = 0u64;
    for i in 0..measurements.len()-1 {
        if measurements[i] < measurements[i+1] {
            counted_diffs1 += 1;
        }
    }
    let mut counted_diffs2 = 0u64;
    for i in 0..measurements.len()-3 {
        if measurements[i] < measurements[i+3] {
            counted_diffs2 += 1;
        }
    }
    println!("Number of depth increases w/o sliding window: {}", counted_diffs1);
    println!("Number of depth increases with sliding window: {}", counted_diffs2);
}
