use std::fs;
use Part::*;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Part {
    Num(u64),
    Push,
    Pop
}

fn parse(s: &str) -> Vec<Part> {
    s.chars().filter_map(|c| match c {
        '0'..='9' => Some(Num(c.to_digit(10).unwrap() as u64)),
        '[' => Some(Push),
        ']' => Some(Pop),
        _ => None
    }).collect()
}

fn explode(sfn: &mut Vec<Part>) -> bool {
    let mut depth = 0;
    for i in 0..sfn.len() {
        match sfn[i] {
            Push => {
                if depth >= 4 && sfn[i+3] == Pop {
                    let (left_value, right_value) = {
                        let mut drain = sfn.drain(i..=i+3);
                        drain.next().unwrap(); // push
                        let left = drain.next().unwrap();
                        let left_value = if let Num(n) = left {n} else {panic!()};
                        let right = drain.next().unwrap();
                        let right_value = if let Num(n) = right {n} else {panic!()};
                        drain.next().unwrap(); // pop
                        (left_value, right_value)
                    };

                    sfn.insert(i, Num(0));
                    'inner1: for j in (0..i).rev() {
                        if let Num(n) = sfn[j] {
                            sfn[j] = Num(n + left_value);
                            break 'inner1;
                        }
                    }
                    'inner2: for j in i+1..sfn.len() {
                        if let Num(n) = sfn[j] {
                            sfn[j] = Num(n + right_value);
                            break 'inner2;
                        }
                    }
                    return true;
                }
                depth += 1;
            },
            Pop => {
                depth -= 1;
            },
            _ => ()
        }
    }
    return false;
}

fn split(sfn: &mut Vec<Part>) -> bool {
    for i in 0..sfn.len() {
        match sfn[i] {
            Num(n) => {
                if n >= 10 {
                    sfn.remove(i);
                    sfn.insert(i, Push);
                    sfn.insert(i+1, Num(n / 2));
                    sfn.insert(i+2, Num(n - n/2));
                    sfn.insert(i+3, Pop);
                    return true;
                }
            },
            _ => ()
        }
    }
    return false;
}

fn magnitude(sfn: &Vec<Part>) -> u64 {
    let mut mult_stack: Vec<u64> = vec![1];
    let mut value_stack: Vec<u64> = vec![0];
    for p in sfn {
        match p {
            Push => {
                mult_stack.push(2);
                mult_stack.push(3);
                value_stack.push(0);
            },
            Num(n) => {
                let new_value = value_stack.pop().unwrap() + mult_stack.pop().unwrap() * n;
                value_stack.push(new_value);
            },
            Pop => {
                let popped_value = value_stack.pop().unwrap() * mult_stack.pop().unwrap();
                let popped_value_2 = value_stack.pop().unwrap() + popped_value;
                value_stack.push(popped_value_2);
            }
        }
    }
    return value_stack[0];
}

fn add(mut a: Vec<Part>, b: &Vec<Part>) -> Vec<Part> {
    if a.is_empty() {
        return b.clone();
    }
    a.insert(0,Push);
    a.append(&mut b.clone());
    a.push(Pop);
    loop {
        while explode(&mut a) {}
        if !split(&mut a) { break; }
    }
    return a;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let numbers: Vec<Vec<Part>> = input.lines().map(|l|parse(l)).collect();
    let sum: Vec<Part> = numbers.iter().fold(Vec::new(), add);
    println!("Magnitude of the all-number sum: {}", magnitude(&sum));
    let mut best_sum = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let sum = magnitude(&add(numbers[i].clone(), &numbers[j]));
                if sum > best_sum {best_sum = sum;}
            }
        }
    }
    println!("Magnitude of the best 2-number sum: {}", best_sum);
}
