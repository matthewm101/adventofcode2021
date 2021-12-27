use std::fs;

fn update(map: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let height = map.len();
    let width = map[0].len();
    let mut movement_detected = false;
    let mut new_map: Vec<Vec<char>> = map.into_iter().map(|v|v.into_iter().map(|_|'.').collect()).collect();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '>' {
                if map[y][(x + 1) % width] == '.' {
                    new_map[y][(x + 1) % width] = '>';
                    movement_detected = true;
                } else {
                    new_map[y][x] = '>';
                }
            } else if map[y][x] == 'v' {
                new_map[y][x] = 'v';
            }
        }
    }
    let mut newer_map: Vec<Vec<char>> = map.into_iter().map(|v|v.into_iter().map(|_|'.').collect()).collect();
    for y in 0..height {
        for x in 0..width {
            if new_map[y][x] == 'v' {
                if new_map[(y + 1) % height][x] == '.' {
                    newer_map[(y + 1) % height][x] = 'v';
                    movement_detected = true;
                } else {
                    newer_map[y][x] = 'v';
                }
            } else if new_map[y][x] == '>' {
                newer_map[y][x] = '>';
            }
        }
    }
    return (newer_map, movement_detected);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|s|s.chars().collect()).collect();
    let mut keep_going = true;
    let mut steps = 0usize;
    while keep_going {
        let (next_map, movement_detected) = update(&map);
        map = next_map;
        keep_going = movement_detected;
        steps += 1;
    }
    println!("Steps until sea cucumbers settle: {}", steps);
}
