use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64
}

impl Cuboid {
    fn size(&self) -> i64 {
        return (self.x2 + 1 - self.x1) * (self.y2 + 1 - self.y1) * (self.z2 + 1 - self.z1);
    }
}

fn parse_line(s: &str) -> (Cuboid, bool) {
    let splits: Vec<&str> = s.split(" ").collect();
    let is_on = splits[0] == "on";
    let splits: Vec<&str> = splits[1].split(|c| c == 'x' || c == 'y' || c == 'z' || c == '=' || c == ',' || c == '.' || c == '\n').filter(|s|s.len() > 0).collect();
    let numbers: Vec<i64> = splits.into_iter().map(|s|s.parse().unwrap()).collect();
    return (Cuboid{
        x1: numbers[0],
        x2: numbers[1],
        y1: numbers[2],
        y2: numbers[3],
        z1: numbers[4],
        z2: numbers[5]
    }, is_on)
}

fn pair_intersection(low_a: i64, high_a: i64, low_b: i64, high_b: i64) -> Option<(i64,i64)> {
    if high_a < low_b || high_b < low_a {return None;} // No intersection
    if low_b <= low_a && high_a <= high_b {return Some((low_a, high_a))} // Complete envelopment
    if low_a < low_b && high_b < high_a {return Some((low_b, high_b))} // Original interval completely envelops subtract interval
    if low_a < low_b && low_b <= high_a {return Some((low_b,high_a))}
    if low_a <= high_b && high_b < high_a {return Some((low_a,high_b))}
    panic!("Missed a case")
}

fn bounded_intersection(a: &Cuboid, b: &Cuboid) -> Option<Cuboid> {
    if let Some((x1,x2)) = pair_intersection(a.x1,a.x2,b.x1,b.x2) {
        if let Some((y1,y2)) = pair_intersection(a.y1,a.y2,b.y1,b.y2) {
            if let Some((z1,z2)) = pair_intersection(a.z1,a.z2,b.z1,b.z2) {
                return Some(Cuboid{x1,x2,y1,y2,z1,z2});
            }
        }
    }
    return None;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let actions: Vec<(Cuboid,bool)> = input.lines().map(parse_line).collect();
    let init_actions = &actions[0..20];
    let mut cubes: Vec<(Cuboid,i64)> = Vec::new();
    for (action_cube, do_add) in init_actions {
        let mut new_cubes = Vec::new();
        for (base_cube,weight) in &cubes {
            new_cubes.push((*base_cube, *weight));
            if let Some(intersection) = bounded_intersection(base_cube, action_cube) {
                new_cubes.push((intersection, weight * -1));
            }
        }
        if *do_add {
            new_cubes.push((*action_cube, 1));
        }
        cubes = new_cubes;
    }
    let amount = cubes.iter().map(|(c,v)|c.size() * v).sum::<i64>();
    println!("Number of lit cubes after initialization steps: {}", amount);
    let mut cubes: Vec<(Cuboid,i64)> = Vec::new();
    for (action_cube, do_add) in &actions {
        let mut new_cubes = Vec::new();
        for (base_cube,weight) in &cubes {
            new_cubes.push((*base_cube, *weight));
            if let Some(intersection) = bounded_intersection(base_cube, action_cube) {
                new_cubes.push((intersection, weight * -1));
            }
        }
        if *do_add {
            new_cubes.push((*action_cube, 1));
        }
        cubes = new_cubes;
    }
    let amount = cubes.iter().map(|(c,v)|c.size() * v).sum::<i64>();
    println!("Number of lit cubes after reboot steps: {}", amount);
}

/*
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let actions: Vec<(Cuboid,bool)> = input.lines().map(parse_line).collect();
    let init_actions = &actions[0..20];
    let mut cubes: Vec<Cuboid> = Vec::new();
    let mut i = 1;
    for (action_cube, do_add) in init_actions {
        let mut new_cubes = Vec::new();
        for base_cube in &cubes {
            new_cubes.append(&mut subtract_cubes(base_cube, action_cube));
        }
        if *do_add {
            new_cubes.push(*action_cube);
        }
        cubes = new_cubes;
        i += 1;
    }
    // let init_cubes: Vec<&Cuboid> = cubes.iter().filter(|c|
    //     c.x1 >= -50 && c.x2 <= 50 && c.y1 >= -50 && c.y2 <= 50 && c.z1 >= -50 && c.z2 <= 50).collect();
    // println!("Number of lit cubes in initialization region: {}", init_cubes.iter().map(|c|c.size()).sum::<i64>());
    println!("Number of lit cubes after initialization steps: {}", cubes.iter().map(|c|c.size()).sum::<i64>());
    let mut cubes: Vec<Cuboid> = Vec::new();
    let mut i = 1;
    for (action_cube, do_add) in &actions {
        let mut new_cubes = Vec::new();
        for base_cube in &cubes {
            new_cubes.append(&mut subtract_cubes(base_cube, action_cube));
        }
        if *do_add {
            new_cubes.push(*action_cube);
        }
        cubes = new_cubes;
        i += 1;
    }
    println!("Number of lit cubes after reboot steps: {}", cubes.iter().map(|c|c.size()).sum::<i64>());
}
*/