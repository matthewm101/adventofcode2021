use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn get_risk(risks: &Vec<Vec<u64>>, x: usize, y: usize) -> u64 {
    let height = risks.len();
    let width = risks[0].len();
    let base_risk = risks[y % height][x % width];
    let increased_risk = base_risk + (x / width) as u64 + (y / height) as u64;
    let fixed_risk = (increased_risk - 1) % 9 + 1;
    return fixed_risk;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let risks: Vec<Vec<u64>> = input.lines().map(|l|l.chars().map(|c|c.to_digit(10).unwrap() as u64).collect()).collect();
    let height = risks.len();
    let width = risks[0].len();
    let mut found_risks: HashMap<(usize,usize),u64> = HashMap::new();
    let mut worklist: BinaryHeap<Reverse<(u64,usize,usize)>> = BinaryHeap::new();
    worklist.push(Reverse((0,0,0)));
    while !worklist.is_empty() {
        let (risk, x, y) = worklist.pop().unwrap().0;
        if !found_risks.contains_key(&(x,y)) {
            found_risks.insert((x,y), risk);
            if x > 0 {
                worklist.push(Reverse((risk + get_risk(&risks,x-1, y), x-1, y)));
            }
            if y > 0 {
                worklist.push(Reverse((risk + get_risk(&risks,x, y-1), x, y-1)));
            }
            if x < width * 5 - 1 {
                worklist.push(Reverse((risk + get_risk(&risks,x+1, y), x+1, y)));
            }
            if y < height * 5 - 1 {
                worklist.push(Reverse((risk + get_risk(&risks,x, y+1), x, y+1)));
            }
        }
    }
    println!("Risk of bottom-right corner: {}", found_risks.get(&(width-1,height-1)).unwrap());
    println!("Risk of really bottom-right corner: {}", found_risks.get(&(width*5-1,height*5-1)).unwrap());
    // for y in 0..height {
    //     for x in 0..width {
    //         print!("{} ", found_risks.get(&(x,y)).unwrap());
    //     }
    //     println!();
    // }
}
