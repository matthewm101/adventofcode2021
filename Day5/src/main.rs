use std::fs;
use std::collections::HashMap;

struct Vent{x1: usize, y1: usize, x2: usize, y2: usize}
impl Vent {
    fn get_points_if_h(&self) -> Vec<(usize,usize)>{
        if self.y1 == self.y2 {
            return if self.x1 < self.x2 {
                (self.x1..=self.x2).map(|x| (x, self.y1)).collect()
            } else {
                (self.x2..=self.x1).map(|x| (x, self.y1)).collect()
            };
        }
        return Vec::new();
    }
    fn get_points_if_v(&self) -> Vec<(usize,usize)>{
        if self.x1 == self.x2 {
            return if self.y1 < self.y2 {
                (self.y1..=self.y2).map(|y| (self.x1, y)).collect()
            } else {
                (self.y2..=self.y1).map(|y| (self.x1, y)).collect()
            };
        }
        return Vec::new();
    }
    fn get_points_if_d(&self) -> Vec<(usize,usize)> {
        if self.x1 == self.x2 || self.y1 == self.y2 {return Vec::new();}
        let mut mx1 = self.x1;
        let mut mx2 = self.x2;
        let mut my1 = self.y1;
        let mut my2 = self.y2;
        if mx1 > mx2 {
            let temp = mx1; mx1 = mx2; mx2 = temp;
            let temp = my1; my1 = my2; my2 = temp;
        }
        return if my1 < my2 {
            let mut dps: Vec<(usize, usize)> = Vec::new();
            dps.push((mx1, my1));
            while mx1 != mx2 && my1 != my2 {
                mx1 += 1;
                my1 += 1;
                dps.push((mx1, my1));
            }
            dps
        } else {
            let mut dps: Vec<(usize, usize)> = Vec::new();
            dps.push((mx1, my1));
            while mx1 != mx2 && my1 != my2 {
                mx1 += 1;
                my1 -= 1;
                dps.push((mx1, my1));
            }
            dps
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut vents: Vec<Vent> = Vec::new();
    for line in input.lines() {
        let mut splits_12_34 = line.split(" -> ");
        let mut splits_1_2 = splits_12_34.next().unwrap().split(",");
        let mut splits_3_4 = splits_12_34.next().unwrap().split(",");
        vents.push(Vent {
            x1: splits_1_2.next().unwrap().parse().unwrap(),
            y1: splits_1_2.next().unwrap().parse().unwrap(),
            x2: splits_3_4.next().unwrap().parse().unwrap(),
            y2: splits_3_4.next().unwrap().parse().unwrap()
        });
    }
    let mut vent_map: HashMap<(usize,usize), u64> = HashMap::with_capacity(1000000);
    for vent in &vents {
        for p in vent.get_points_if_h() {
            if vent_map.contains_key(&p) {*vent_map.get_mut(&p).unwrap() += 1;}
            else {vent_map.insert(p, 1);}
        }
        for p in vent.get_points_if_v() {
            if vent_map.contains_key(&p) {*vent_map.get_mut(&p).unwrap() += 1;}
            else {vent_map.insert(p, 1);}
        }
    }
    let overlapping_vents: u64 = (0..1000000).map(|i| if *vent_map.get(&(i/1000,i%1000)).unwrap_or(&0) > 1 {1} else {0}).sum();
    println!("Number of spaces with overlapping vents (vertical and horizontal only): {}", overlapping_vents);
    for vent in &vents {
        for p in vent.get_points_if_d() {
            if vent_map.contains_key(&p) {*vent_map.get_mut(&p).unwrap() += 1;}
            else {vent_map.insert(p, 1);}
        }
    }
    let overlapping_vents: u64 = (0..1000000).map(|i| if *vent_map.get(&(i/1000,i%1000)).unwrap_or(&0) > 1 {1} else {0}).sum();
    println!("Number of spaces with overlapping vents (including diagonals): {}", overlapping_vents);
}
