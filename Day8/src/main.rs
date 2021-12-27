use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn string_to_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn sort_string(s: &str) -> String {
    let mut vec_s: Vec<char> = s.chars().collect();
    vec_s.sort();
    vec_s.into_iter().collect()
}

fn set_to_string(s: &HashSet<char>) -> String {
    let mut vec_s: Vec<char> = s.into_iter().map(|x|*x).collect();
    vec_s.sort();
    vec_s.into_iter().collect()
}

fn parse_segment_basic(s: &str) -> HashSet<usize> {
    match s.len() {
        2 => HashSet::from([1]),
        3 => HashSet::from([7]),
        4 => HashSet::from([4]),
        5 => HashSet::from([2,3,5]),
        6 => HashSet::from([0,6,9]),
        7 => HashSet::from([8]),
        _ => HashSet::new()
    }
}

fn solve(inputs: &Vec<&str>, outputs: &Vec<&str>) -> usize {
    let mut set_map: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut unmapped: Vec<HashSet<char>> = Vec::new();
    for &input in inputs {
        let input_set = string_to_set(input);
        let psb = parse_segment_basic(input);
        if psb.len() == 1 {
            set_map.insert(*psb.iter().next().unwrap(), input_set);
        } else {
            unmapped.push(input_set);
        }
    }
    for set in unmapped {
        let len = set.len();
        if len == 5 {
            if set.intersection(set_map.get(&4).unwrap()).count() == 2 {
                set_map.insert(2, set);
            } else if set.intersection(set_map.get(&1).unwrap()).count() == 2 {
                set_map.insert(3, set);
            } else {
                set_map.insert(5, set);
            }
        } else {
            if set.intersection(set_map.get(&7).unwrap()).count() == 2 {
                set_map.insert(6, set);
            } else if set.intersection(set_map.get(&4).unwrap()).count() == 3 {
                set_map.insert(0, set);
            } else {
                set_map.insert(9, set);
            }
        }
    }
    let rmap: HashMap<String,usize> = set_map.iter().map(|(&k,v)|(set_to_string(v),k)).collect();
    return outputs.iter().map(|&s|rmap.get(sort_string(s).as_str()).unwrap()).fold(0,|x,y|x*10+y);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut count_1478 = 0usize;
    let mut output_sum = 0usize;
    for line in input.lines() {
        let mut io_splits = line.split(" | ");
        let inputs: Vec<&str> = io_splits.next().unwrap().split(" ").collect();
        let outputs: Vec<&str> = io_splits.next().unwrap().split(" ").collect();
        count_1478 += outputs.iter().filter(|&&s|parse_segment_basic(s).len() == 1).count();
        output_sum += solve(&inputs,&outputs)
    }
    println!("Number of output segments with 1, 4, 7, or 8: {}", count_1478);
    println!("Sum of all output values: {}", output_sum);
}
