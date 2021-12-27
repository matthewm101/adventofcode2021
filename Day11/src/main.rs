use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut octopuses: Vec<u32> = input.chars().filter_map(|x|x.to_digit(10)).collect();
    let mut flashes = 0u64;
    for _ in 0..100 {
        let mut worklist: Vec<usize> = (0..100).collect();
        let mut lit: Vec<bool> = (0..100).map(|_|false).collect();
        while !worklist.is_empty() {
            let i = worklist.pop().unwrap();
            octopuses[i] += 1;
            if octopuses[i] > 9 && !lit[i] {
                flashes += 1;
                lit[i] = true;
                let x = i % 10;
                let y = i / 10;
                if x > 0 {worklist.push(i-1);}
                if x < 9 {worklist.push(i+1);}
                if y > 0 {worklist.push(i-10);}
                if y < 9 {worklist.push(i+10);}
                if x > 0 && y > 0 {worklist.push(i-11);}
                if x < 9 && y > 0 {worklist.push(i-9);}
                if x > 0 && y < 9 {worklist.push(i+9);}
                if x < 9 && y < 9 {worklist.push(i+11);}
            }
        }
        for i in 0..100 {
            if octopuses[i] > 9 {octopuses[i] = 0;}
        }
    }
    println!("Total flashes: {}", flashes);
    let mut iters = 100u64;
    loop {
        iters += 1;
        let mut worklist: Vec<usize> = (0..100).collect();
        let mut lit: Vec<bool> = (0..100).map(|_|false).collect();
        while !worklist.is_empty() {
            let i = worklist.pop().unwrap();
            octopuses[i] += 1;
            if octopuses[i] > 9 && !lit[i] {
                lit[i] = true;
                let x = i % 10;
                let y = i / 10;
                if x > 0 {worklist.push(i-1);}
                if x < 9 {worklist.push(i+1);}
                if y > 0 {worklist.push(i-10);}
                if y < 9 {worklist.push(i+10);}
                if x > 0 && y > 0 {worklist.push(i-11);}
                if x < 9 && y > 0 {worklist.push(i-9);}
                if x > 0 && y < 9 {worklist.push(i+9);}
                if x < 9 && y < 9 {worklist.push(i+11);}
            }
        }
        let mut count = 0u64;
        for i in 0..100 {
            if octopuses[i] > 9 {
                octopuses[i] = 0;
                count += 1;
            }
        }
        if count == 100 {break;}
    }
    println!("Total iterations until synchronization: {}", iters);
}
