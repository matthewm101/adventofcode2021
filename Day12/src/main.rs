use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut numbering_counter = 1usize;
    let mut string_map: HashMap<String,usize> = HashMap::new();
    let mut big_cave_bitvector = 0u64;
    let mut cave_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    string_map.insert("start".to_owned(), 0);
    string_map.insert("end".to_owned(), 63);
    for line in input.lines() {
        let splits: Vec<&str> = line.split("-").collect();
        let p1 = if let Some(n) = string_map.get(splits[0]) {*n} else {
            string_map.insert(splits[0].to_owned(), numbering_counter);
            if splits[0].chars().next().unwrap().is_uppercase() {
                big_cave_bitvector |= 1 << numbering_counter;
            }
            numbering_counter += 1;
            numbering_counter - 1
        };
        let p2 = if let Some(n) = string_map.get(splits[1]) {*n} else {
            string_map.insert(splits[1].to_owned(), numbering_counter);
            if splits[1].chars().next().unwrap().is_uppercase() {
                big_cave_bitvector |= 1 << numbering_counter;
            }
            numbering_counter += 1;
            numbering_counter - 1
        };
        if let Some(s) = cave_map.get_mut(&p1) {
            s.insert(p2);
        } else {
            cave_map.insert(p1, HashSet::from([p2]));
        }
        if let Some(s) = cave_map.get_mut(&p2) {
            s.insert(p1);
        } else {
            cave_map.insert(p2, HashSet::from([p1]));
        }
    }

    let mut counted_paths = 0usize;
    let mut worklist: VecDeque<(usize,u64)> = VecDeque::from([(0, 1)]);

    while !worklist.is_empty() {
        let (pos, met): (usize, u64) = worklist.pop_front().unwrap();
        for &dest in cave_map.get(&pos).unwrap() {
            if dest == 63 {
                counted_paths += 1;
            } else if big_cave_bitvector & (1 << dest) != 0 { // big cave
                worklist.push_back((dest,met));
            } else if met & (1 << dest) == 0 { // unvisited small cave
                worklist.push_back((dest, met | (1 << dest)));
            }
        }
    }

    println!("Counted paths (every small cave visited no more than once): {}", counted_paths);

    let mut counted_paths = 0usize;
    let mut worklist: VecDeque<(usize,u64,usize)> = VecDeque::from([(0, 1, 0)]);

    while !worklist.is_empty() {
        let (pos, met, dub): (usize, u64, usize) = worklist.pop_front().unwrap();
        for &dest in cave_map.get(&pos).unwrap() {
            if dest == 63 {
                counted_paths += 1;
            } else if big_cave_bitvector & (1 << dest) != 0 { // big cave
                worklist.push_back((dest,met,dub));
            } else if met & (1 << dest) == 0 { // unvisited small cave
                worklist.push_back((dest, met | (1 << dest), dub));
            } else if dub == 0 && dest != 0 { // double visit
                worklist.push_back((dest, met, dest))
            }
        }
    }

    println!("Counted paths (one small cave can be visited twice): {}", counted_paths);
}
