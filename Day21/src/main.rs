use std::cmp::min;
use std::collections::HashMap;
use std::fs;

#[derive(Copy,Clone,Debug,Hash,Eq,PartialEq)]
struct Universe {
    p1_pos: u8,
    p2_pos: u8,
    p1_score: u8,
    p2_score: u8
}

impl Universe {
    fn split_p1(self) -> [Universe;7] {
        let new_pos_1 = (self.p1_pos + 2) % 10 + 1;
        let new_pos_2 = (self.p1_pos + 3) % 10 + 1;
        let new_pos_3 = (self.p1_pos + 4) % 10 + 1;
        let new_pos_4 = (self.p1_pos + 5) % 10 + 1;
        let new_pos_5 = (self.p1_pos + 6) % 10 + 1;
        let new_pos_6 = (self.p1_pos + 7) % 10 + 1;
        let new_pos_7 = (self.p1_pos + 8) % 10 + 1;
        return [
            Universe{p1_pos: new_pos_1, p2_pos: self.p2_pos, p1_score: self.p1_score + new_pos_1, p2_score: self.p2_score},
            Universe{p1_pos: new_pos_2, p2_pos: self.p2_pos, p1_score: self.p1_score + new_pos_2, p2_score: self.p2_score},
            Universe{p1_pos: new_pos_3, p2_pos: self.p2_pos, p1_score: self.p1_score + new_pos_3, p2_score: self.p2_score},
            Universe{p1_pos: new_pos_4, p2_pos: self.p2_pos, p1_score: self.p1_score + new_pos_4, p2_score: self.p2_score},
            Universe{p1_pos: new_pos_5, p2_pos: self.p2_pos, p1_score: self.p1_score + new_pos_5, p2_score: self.p2_score},
            Universe{p1_pos: new_pos_6, p2_pos: self.p2_pos, p1_score: self.p1_score + new_pos_6, p2_score: self.p2_score},
            Universe{p1_pos: new_pos_7, p2_pos: self.p2_pos, p1_score: self.p1_score + new_pos_7, p2_score: self.p2_score}
        ];
    }
    fn split_p2(self) -> [Universe;7] {
        let new_pos_1 = (self.p2_pos + 2) % 10 + 1;
        let new_pos_2 = (self.p2_pos + 3) % 10 + 1;
        let new_pos_3 = (self.p2_pos + 4) % 10 + 1;
        let new_pos_4 = (self.p2_pos + 5) % 10 + 1;
        let new_pos_5 = (self.p2_pos + 6) % 10 + 1;
        let new_pos_6 = (self.p2_pos + 7) % 10 + 1;
        let new_pos_7 = (self.p2_pos + 8) % 10 + 1;
        return [
            Universe{p2_pos: new_pos_1, p1_pos: self.p1_pos, p2_score: self.p2_score + new_pos_1, p1_score: self.p1_score},
            Universe{p2_pos: new_pos_2, p1_pos: self.p1_pos, p2_score: self.p2_score + new_pos_2, p1_score: self.p1_score},
            Universe{p2_pos: new_pos_3, p1_pos: self.p1_pos, p2_score: self.p2_score + new_pos_3, p1_score: self.p1_score},
            Universe{p2_pos: new_pos_4, p1_pos: self.p1_pos, p2_score: self.p2_score + new_pos_4, p1_score: self.p1_score},
            Universe{p2_pos: new_pos_5, p1_pos: self.p1_pos, p2_score: self.p2_score + new_pos_5, p1_score: self.p1_score},
            Universe{p2_pos: new_pos_6, p1_pos: self.p1_pos, p2_score: self.p2_score + new_pos_6, p1_score: self.p1_score},
            Universe{p2_pos: new_pos_7, p1_pos: self.p1_pos, p2_score: self.p2_score + new_pos_7, p1_score: self.p1_score}
        ];
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let p1_starting_position: u8 = lines.next().unwrap().trim_start_matches("Player 1 starting position: ").parse().unwrap();
    let p2_starting_position: u8 = lines.next().unwrap().trim_start_matches("Player 2 starting position: ").parse().unwrap();
    let mut p1_position = p1_starting_position as u64;
    let mut p2_position = p2_starting_position as u64;
    let mut rolls = 0u64;
    let mut p1_score = 0u64;
    let mut p2_score = 0u64;
    let mut next_roll = 1u64;
    let mut turn = 1u64;
    while p1_score < 1000 && p2_score < 1000 {
        let mut rolled_value = 0u64;
        rolls += 1;
        rolled_value += next_roll;
        next_roll = next_roll % 100 + 1;
        rolls += 1;
        rolled_value += next_roll;
        next_roll = next_roll % 100 + 1;
        rolls += 1;
        rolled_value += next_roll;
        next_roll = next_roll % 100 + 1;
        if turn == 1 {
            p1_position = (p1_position - 1 + rolled_value) % 10 + 1;
            p1_score += p1_position;
        } else {
            p2_position = (p2_position - 1 + rolled_value) % 10 + 1;
            p2_score += p2_position;
        }
        turn = 1 - turn;
    }
    println!("Results of the deterministic game:");
    println!("\tFinal P1 score: {}", p1_score);
    println!("\tFinal P2 score: {}", p2_score);
    println!("\tNumber of rolls: {}", rolls);
    println!("\tLoser score times rolls: {}", rolls * min(p1_score, p2_score));
    println!();

    let mut universe_counts: HashMap<Universe, usize> = HashMap::new();
    let mut p1_wins = 0usize;
    let mut p2_wins = 0usize;
    let mut p2s_turn = false;
    let start = Universe{
        p1_pos: p1_starting_position,
        p2_pos: p2_starting_position,
        p1_score: 0,
        p2_score: 0
    };
    universe_counts.insert(start, 1);
    while !universe_counts.is_empty() {
        let mut new_universe_counts: HashMap<Universe,usize> = HashMap::new();
        for (universe, count) in universe_counts {
            let new_universes = if p2s_turn {universe.split_p2()} else {universe.split_p1()};
            new_universe_counts.insert(new_universes[0], new_universe_counts.get(&new_universes[0]).unwrap_or(&0) + count);
            new_universe_counts.insert(new_universes[1], new_universe_counts.get(&new_universes[1]).unwrap_or(&0) + count * 3);
            new_universe_counts.insert(new_universes[2], new_universe_counts.get(&new_universes[2]).unwrap_or(&0) + count * 6);
            new_universe_counts.insert(new_universes[3], new_universe_counts.get(&new_universes[3]).unwrap_or(&0) + count * 7);
            new_universe_counts.insert(new_universes[4], new_universe_counts.get(&new_universes[4]).unwrap_or(&0) + count * 6);
            new_universe_counts.insert(new_universes[5], new_universe_counts.get(&new_universes[5]).unwrap_or(&0) + count * 3);
            new_universe_counts.insert(new_universes[6], new_universe_counts.get(&new_universes[6]).unwrap_or(&0) + count);
        }
        let mut finished_universes: Vec<Universe> = Vec::new();
        for (&universe,&count) in &new_universe_counts {
            if universe.p1_score >= 21 {
                finished_universes.push(universe);
                p1_wins += count;
            } else if universe.p2_score >= 21 {
                finished_universes.push(universe);
                p2_wins += count;
            }
        }
        for universe in finished_universes {
            new_universe_counts.remove(&universe);
        }
        universe_counts = new_universe_counts;
        p2s_turn = !p2s_turn;
    }
    println!("Results of the quantum game:");
    println!("\tP1's wins: {}", p1_wins);
    println!("\tP2's wins: {}", p2_wins);
}
