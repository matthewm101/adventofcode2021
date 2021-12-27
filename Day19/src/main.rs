use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn transform(v: &(i64,i64,i64), t: u64) -> (i64,i64,i64) {
    match t {
        0 => (v.0,v.1,v.2),
        1 => (v.0,-v.2,v.1),
        2 => (v.0,-v.1,-v.2),
        3 => (v.0,v.2,-v.1),
        4 => (-v.0,-v.1,v.2),
        5 => (-v.0,v.2,v.1),
        6 => (-v.0,v.1,-v.2),
        7 => (-v.0,-v.2,-v.1),
        8 => (v.1,v.2,v.0),
        9 => (-v.2,v.1,v.0),
        10 => (-v.1,-v.2,v.0),
        11 => (v.2,-v.1,v.0),
        12 => (-v.1,v.2,-v.0),
        13 => (v.2,v.1,-v.0),
        14 => (v.1,-v.2,-v.0),
        15 => (-v.2,-v.1,-v.0),
        16 => (v.2,v.0,v.1),
        17 => (v.1,v.0,-v.2),
        18 => (-v.2,v.0,-v.1),
        19 => (-v.1,v.0,v.2),
        20 => (v.2,-v.0,-v.1),
        21 => (v.1,-v.0,v.2),
        22 => (-v.2,-v.0,v.1),
        23 => (-v.1,-v.0,-v.2),
        _ => panic!()
    }
}

fn try_to_align(root: &HashSet<(i64,i64,i64)>, sat: &Vec<(i64,i64,i64)>) -> Option<(u64,(i64,i64,i64))> {
    for t in 0..24 {
        let mut frequencies: HashMap<(i64,i64,i64),usize> = HashMap::new();
        for v1 in root {
            for v2 in sat {
                let v3 = transform(v2,t);
                let diff = (v1.0-v3.0,v1.1-v3.1,v1.2-v3.2);
                frequencies.insert(diff, frequencies.get(&diff).unwrap_or(&0) + 1);
            }
        }
        if let Some(d) = frequencies.into_iter().filter_map(|(d,f)| if f >= 12 {Some(d)} else {None}).next() {
            return Some((t,d));
        }
    }
    return None;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut raw_readings: Vec<Vec<(i64,i64,i64)>> = Vec::new();
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        lines.next();
        raw_readings.push(Vec::new());
        'inner: loop {
            let s = lines.next().unwrap_or("");
            if s.len() == 0 {break 'inner;}
            let numbers: Vec<i64> = s.split(",").map(|s|s.parse().unwrap()).collect();
            raw_readings.last_mut().unwrap().push((numbers[0],numbers[1],numbers[2]));
        }
    }

    let mut aligned_readings: Vec<Vec<(i64,i64,i64)>> = Vec::new();
    for _ in 0..raw_readings.len() {aligned_readings.push(Vec::new());}
    let mut beacons: HashSet<(i64,i64,i64)> = HashSet::new();
    for r in &raw_readings[0] {
        beacons.insert(*r);
    }
    let mut unaligned_stations: VecDeque<usize> = VecDeque::from((1..raw_readings.len()).collect::<Vec<usize>>());
    let mut diffs: Vec<(i64,i64,i64)> = Vec::from([(0,0,0)]);
    while !unaligned_stations.is_empty() {
        let station_to_align = unaligned_stations.pop_front().unwrap();
        if let Some((t,d)) = try_to_align(&beacons, &raw_readings[station_to_align]) {
            diffs.push(d);
            for r in &raw_readings[station_to_align] {
                let transformed = transform(r, t);
                let moved = (transformed.0 + d.0, transformed.1 + d.1, transformed.2 + d.2);
                beacons.insert(moved);
                aligned_readings[station_to_align].push(moved);
            }
            println!("Aligned station {}, transform {} and diff {:?}", station_to_align, t, d);
        } else {
            unaligned_stations.push_back(station_to_align);
        }
    }
    println!("All stations aligned. Number of beacons: {}", beacons.len());
    let mut best_manhattan_distance = 0i64;
    for i in 0..diffs.len()-1 {
        for j in i+1..diffs.len() {
            let v1 = diffs[i];
            let v2 = diffs[j];
            let dist = (v1.0 - v2.0).abs() + (v1.1 - v2.1).abs() + (v1.2 - v2.2).abs();
            if dist > best_manhattan_distance {best_manhattan_distance = dist;}
        }
    }
    println!("Farthest distance between beacons: {}", best_manhattan_distance)
}
