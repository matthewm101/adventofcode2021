use std::collections::HashMap;
use std::fs;

fn step(chain: HashMap<(char,char),usize>, rules: &HashMap<(char,char),char>) -> HashMap<(char,char),usize> {
    let mut new_chain = chain.clone();
    for (ab,&c) in rules {
        let (a,b) = *ab;
        if let Some(amt) = chain.get(ab) {
            new_chain.insert(*ab, new_chain.get(ab).unwrap_or(&0) - amt);
            new_chain.insert((a,c), new_chain.get(&(a,c)).unwrap_or(&0) + amt);
            new_chain.insert((c,b), new_chain.get(&(c,b)).unwrap_or(&0) + amt);
        }
    }
    return new_chain;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut chain: HashMap<(char,char),usize> = HashMap::new();
    let original_chain: Vec<char> = lines.next().unwrap().chars().collect();
    for i in 0..original_chain.len()-1 {
        let k = (original_chain[i],original_chain[i+1]);
        chain.insert(k, chain.get(&k).unwrap_or(&0) + 1);
    }
    lines.next();
    let mut rules: HashMap<(char,char),char> = HashMap::new();
    for line in lines {
        let splits: Vec<&str> = line.split(" -> ").collect();
        let ab: Vec<char> = splits[0].chars().collect();
        let c = splits[1].chars().next().unwrap();
        rules.insert((ab[0],ab[1]), c);
    }
    // print!("Starting "); print_chain(&chain);
    for _ in 0..10 {
        chain = step(chain, &rules);
    }
    let mut letter_frequencies: HashMap<char,usize> = HashMap::new();
    for (&c,&n) in chain.iter() {
        letter_frequencies.insert(c.0, letter_frequencies.get(&c.0).unwrap_or(&0) + n);
    }
    let last_letter = original_chain.last().unwrap();
    letter_frequencies.insert(*last_letter, letter_frequencies.get(last_letter).unwrap_or(&0) + 1);
    let max = *letter_frequencies.values().max().unwrap();
    let min = *letter_frequencies.values().min().unwrap();
    println!("Most common frequency after 10 iterations: {}", max);
    println!("Least common frequency after 10 iterations: {}", min);
    println!("Difference between most and least common frequencies: {}", max - min);
    println!();
    for _ in 0..30 {
        chain = step(chain, &rules);
    }
    let mut letter_frequencies: HashMap<char,usize> = HashMap::new();
    for (c,n) in chain {
        letter_frequencies.insert(c.0, letter_frequencies.get(&c.0).unwrap_or(&0) + n);
    }
    let last_letter = original_chain.last().unwrap();
    letter_frequencies.insert(*last_letter, letter_frequencies.get(last_letter).unwrap_or(&0) + 1);
    let max = *letter_frequencies.values().max().unwrap();
    let min = *letter_frequencies.values().min().unwrap();
    println!("Most common frequency after 40 iterations: {}", max);
    println!("Least common frequency after 40 iterations: {}", min);
    println!("Difference between most and least common frequencies: {}", max - min);
}
