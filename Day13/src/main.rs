use std::collections::{HashMap, HashSet};
use std::fs;

fn abs_diff(x: u64, y: u64) -> u64 {
    if x < y {y - x} else {x - y}
}

fn fold_y(grid: HashSet<(u64,u64)>, line: u64) -> HashSet<(u64,u64)> {
    return grid.into_iter().map(|(x,y)| (x, line - abs_diff(y, line))).collect();
}

fn fold_x(grid: HashSet<(u64,u64)>, line: u64) -> HashSet<(u64,u64)> {
    return grid.into_iter().map(|(x,y)| (line - abs_diff(x, line), y)).collect();
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut dots: HashSet<(u64,u64)> = HashSet::new();
    let mut folds = 0;
    for line in input.lines() {
        let splits: Vec<&str> = line.split(",").collect();
        if splits.len() == 2 {
            dots.insert((splits[0].parse().unwrap(), splits[1].parse().unwrap()));
        } else {
            let splits: Vec<&str> = line.split("=").collect();
            if splits.len() == 2 {
                dots = match splits[0] {
                    "fold along x" => fold_x(dots, splits[1].parse().unwrap()),
                    "fold along y" => fold_y(dots, splits[1].parse().unwrap()),
                    _ => panic!("oop")
                };
                folds += 1;
                println!("Dots after fold {}: {}", folds, dots.len());
                let max_x = dots.iter().map(|&x|x.0).max().unwrap();
                let max_y = dots.iter().map(|&x|x.1).max().unwrap();
            }
        }
    }
    let max_x = dots.iter().map(|&x|x.0).max().unwrap();
    let max_y = dots.iter().map(|&x|x.1).max().unwrap();
    println!("Final grid:");
    for i in 0..=max_y {
        for j in 0..=max_x {
            if dots.contains(&(j,i)) {print!("#");} else {print!(".");}
        }
        println!();
    }
}
