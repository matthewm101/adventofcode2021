use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let pos_vec: Vec<usize> = input.split(",").map(|x|x.parse().unwrap()).collect();
    let cost_per_pos: Vec<usize> = (0usize..=*pos_vec.iter().max().unwrap()).map(|i|pos_vec.iter().map(|p| if i < *p {p-i} else {i-p}).sum()).collect();
    let best_pos = (1..cost_per_pos.len()).fold(0usize, |x,y| if cost_per_pos[x] < cost_per_pos[y] {x} else {y});
    println!("Best position to minimize fuel: {}", best_pos);
    println!("Cost to align to position {}: {}", best_pos, cost_per_pos[best_pos]);
    let cost_per_pos: Vec<usize> = (0usize..=*pos_vec.iter().max().unwrap()).map(|i|pos_vec.iter().map(|p| if i < *p {(p-i)*(p-i+1)/2} else {(i-p)*(i-p+1)/2}).sum()).collect();
    let best_pos = (1..cost_per_pos.len()).fold(0usize, |x,y| if cost_per_pos[x] < cost_per_pos[y] {x} else {y});
    println!("Actually best position to minimize fuel: {}", best_pos);
    println!("Actual cost to align to position {}: {}", best_pos, cost_per_pos[best_pos]);
}
