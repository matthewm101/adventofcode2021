use std::fs;

fn sim(mut vx: i64, mut vy: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut best_y = 0;
    while y >= min_y {
        x += vx;
        y += vy;
        if vx > 0 {vx -= 1;}
        vy -= 1;
        if y > best_y {best_y = y;}
        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            return Some(best_y);
        }
    }
    return None;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let splits: Vec<&str> = input.split(|c|c == '=' || c == '.' || c == ',' || c == ' ' || c == '\n').collect();
    let min_x: i64 = splits[3].parse().unwrap();
    let max_x: i64 = splits[5].parse().unwrap();
    let min_y: i64 = splits[8].parse().unwrap();
    let max_y: i64 = splits[10].parse().unwrap();
    let mut best_y = 0;
    let mut valid_velocities = 0;
    for vx in 0..=max_x {
        for vy in min_y..=(min_y * -1) {
            if let Some(y) = sim(vx, vy, min_x, max_x, min_y, max_y) {
                best_y = best_y.max(y);
                valid_velocities += 1;
            }
        }
    }
    println!("Highest point reached: y={}", best_y);
    println!("Number of valid velocities: {}", valid_velocities);
}
