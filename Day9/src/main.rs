use std::fs;
use std::collections::{HashSet, LinkedList};

struct FloorMap(Vec<Vec<usize>>);
impl FloorMap {
    fn get(&self, x: usize, y: usize) -> usize {self.0[y][x]}
    fn width(&self) -> usize {self.0[0].len()}
    fn height(&self) -> usize {self.0.len()}

    fn check_low(&self, x: usize, y: usize) -> bool {
        let mut min_surr: usize = 999999;
        if x > 0 {min_surr = min_surr.min(self.get(x-1,y));}
        if y > 0 {min_surr = min_surr.min(self.get(x,y-1));}
        if x < self.width()-1 {min_surr = min_surr.min(self.get(x+1,y));}
        if y < self.height()-1 {min_surr = min_surr.min(self.get(x,y+1));}
        return self.get(x,y) < min_surr;
    }

    fn get_risk(&self, x: usize, y: usize) -> usize {
        if self.check_low(x,y) {self.get(x,y) + 1}
        else {0}
    }

    fn get_basin_size(&self, first_x: usize, first_y: usize) -> usize {
        let mut basin: HashSet<(usize,usize)> = HashSet::from([(first_x,first_y)]);
        let mut worklist: LinkedList<(usize,usize)> = LinkedList::from([(first_x,first_y)]);
        while !worklist.is_empty() {
            let (x,y) = worklist.pop_front().unwrap();
            if x > 0 && !basin.contains(&(x-1,y)) && self.get(x-1,y) < 9 {
                basin.insert((x-1,y));
                worklist.push_back((x-1,y));
            }
            if y > 0 && !basin.contains(&(x,y-1)) && self.get(x,y-1) < 9 {
                basin.insert((x,y-1));
                worklist.push_back((x,y-1));
            }
            if x < self.width() - 1 && !basin.contains(&(x+1,y)) && self.get(x+1,y) < 9 {
                basin.insert((x+1,y));
                worklist.push_back((x+1,y));
            }
            if y < self.height() - 1 && !basin.contains(&(x,y+1)) && self.get(x,y+1) < 9 {
                basin.insert((x,y+1));
                worklist.push_back((x,y+1));
            }
        }
        return basin.len();
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let fmap = FloorMap(input.lines().map(|l|l.chars().map(|c|c.to_digit(10).unwrap() as usize).collect()).collect());
    let mut total_risk = 0usize;
    let mut basins: Vec<(usize,usize)> = Vec::new();
    for i in 0..fmap.width() {
        for j in 0..fmap.height() {
            let risk = fmap.get_risk(i,j);
            if risk > 0 {
                total_risk += risk;
                basins.push((i,j));
            }
        }
    }
    println!("Total risk: {}", total_risk);
    let mut basin_sizes: Vec<usize> = basins.into_iter().map(|(x,y)|fmap.get_basin_size(x,y)).collect();
    basin_sizes.sort();
    println!("Top 3 basin sizes: {} {} {}", basin_sizes[basin_sizes.len()-1], basin_sizes[basin_sizes.len()-2], basin_sizes[basin_sizes.len()-3]);
    println!("Product of top 3 sizes: {}", basin_sizes[basin_sizes.len()-1] * basin_sizes[basin_sizes.len()-2] * basin_sizes[basin_sizes.len()-3]);
}
