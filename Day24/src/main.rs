use std::fs;

enum Inst {
    INP(usize),
    ADD(usize,usize),
    ADDI(usize,i64),
    MUL(usize,usize),
    MULI(usize,i64),
    DIV(usize,usize),
    DIVI(usize,i64),
    MOD(usize,usize),
    MODI(usize,i64),
    EQL(usize,usize),
    EQLI(usize,i64)
}

impl Inst {
    fn is_divisor_26(&self) -> bool {match self {Inst::DIVI(_,n) => {*n == 26}, _ => panic!()}}
    fn get_addi_num(&self) -> i64 {match self {Inst::ADDI(_,n) => {*n}, _ => panic!()}}
}

fn parse_var(s: &str) -> usize {
    (s.chars().next().unwrap() as usize) - ('w' as usize)
}

fn parse_inst(s: &str) -> Inst {
    let splits: Vec<&str> = s.split(" ").collect();
    match splits[0] {
        "inp" => Inst::INP(parse_var(splits[1])),
        "add" => if let Ok(n) = splits[2].parse::<i64>() {Inst::ADDI(parse_var(splits[1]),n)} else {Inst::ADD(parse_var(splits[1]),parse_var(splits[2]))},
        "mul" => if let Ok(n) = splits[2].parse::<i64>() {Inst::MULI(parse_var(splits[1]),n)} else {Inst::MUL(parse_var(splits[1]),parse_var(splits[2]))},
        "div" => if let Ok(n) = splits[2].parse::<i64>() {Inst::DIVI(parse_var(splits[1]),n)} else {Inst::DIV(parse_var(splits[1]),parse_var(splits[2]))},
        "mod" => if let Ok(n) = splits[2].parse::<i64>() {Inst::MODI(parse_var(splits[1]),n)} else {Inst::MOD(parse_var(splits[1]),parse_var(splits[2]))},
        "eql" => if let Ok(n) = splits[2].parse::<i64>() {Inst::EQLI(parse_var(splits[1]),n)} else {Inst::EQL(parse_var(splits[1]),parse_var(splits[2]))},
        _ => panic!()
    }
}

fn execute(program: &Vec<Inst>, inputs: &Vec<i64>) -> Option<[i64;4]> {
    let mut registers = [0i64;4];
    let mut input_index = 0;
    for inst in program {
        match inst {
            Inst::INP(i) => {registers[*i] = inputs[input_index]; input_index += 1;},
            Inst::ADD(i,j) => {registers[*i] += registers[*j];},
            Inst::ADDI(i,n) => {registers[*i] += *n;},
            Inst::MUL(i,j) => {registers[*i] *= registers[*j];},
            Inst::MULI(i,n) => {registers[*i] *= *n;},
            Inst::DIV(i,j) => {if registers[*j] == 0 {return None;} registers[*i] /= registers[*j];},
            Inst::DIVI(i,n) => {if *n == 0 {return None;} registers[*i] /= *n;},
            Inst::MOD(i,j) => {if registers[*j] == 0 {return None;} registers[*i] %= registers[*j];},
            Inst::MODI(i,n) => {if *n == 0 {return None;} registers[*i] %= *n;},
            Inst::EQL(i,j) => {if registers[*i] == registers[*j] {registers[*i] = 1;} else {registers[*i] = 0;}},
            Inst::EQLI(i,n) => {if registers[*i] == *n {registers[*i] = 1;} else {registers[*i] = 0;}}
        }
    }
    return Some(registers);
}

fn maximize(step: usize, stack: &[i64], is_pops: &Vec<bool>, remaining_pops: usize, pre_nums: &Vec<i64>, push_nums: &Vec<i64>) -> Option<Vec<i64>> {
    if step == 14 {return Some(vec![])}
    let new_remaining_pops = if is_pops[step] {remaining_pops - 1} else {remaining_pops};
    for i in (1i64..=9).rev() {
        if i == pre_nums[step] + stack.last().unwrap_or(&0) {
            let new_stack = if is_pops[step] && !stack.is_empty() {&stack[0..stack.len()-1]} else {stack};
            if let Some(v) = maximize(step + 1, new_stack, is_pops, new_remaining_pops, pre_nums, push_nums) {
                return Some(vec![i].into_iter().chain(v.into_iter()).collect());
            }
        } else if remaining_pops >= stack.len() + 1 { // this check should significantly help the runtime
            let pushed_value = i + push_nums[step];
            let new_stack = if is_pops[step] && !stack.is_empty() {
                let mut v = stack.to_vec(); *v.last_mut().unwrap() = pushed_value; v
            } else {
                let mut v = stack.to_vec(); v.push(pushed_value); v
            };
            if let Some(v) = maximize(step + 1, new_stack.as_slice(), is_pops, new_remaining_pops, pre_nums, push_nums) {
                return Some(vec![i].into_iter().chain(v.into_iter()).collect());
            }
        }
    }
    return None;
}

fn minimize(step: usize, stack: &[i64], is_pops: &Vec<bool>, remaining_pops: usize, pre_nums: &Vec<i64>, push_nums: &Vec<i64>) -> Option<Vec<i64>> {
    if step == 14 {return Some(vec![])}
    let new_remaining_pops = if is_pops[step] {remaining_pops - 1} else {remaining_pops};
    for i in 1i64..=9 {
        if i == pre_nums[step] + stack.last().unwrap_or(&0) {
            let new_stack = if is_pops[step] && !stack.is_empty() {&stack[0..stack.len()-1]} else {stack};
            if let Some(v) = minimize(step + 1, new_stack, is_pops, new_remaining_pops, pre_nums, push_nums) {
                return Some(vec![i].into_iter().chain(v.into_iter()).collect());
            }
        } else if remaining_pops >= stack.len() + 1 { // this check should significantly help the runtime
            let pushed_value = i + push_nums[step];
            let new_stack = if is_pops[step] && !stack.is_empty() {
                let mut v = stack.to_vec(); *v.last_mut().unwrap() = pushed_value; v
            } else {
                let mut v = stack.to_vec(); v.push(pushed_value); v
            };
            if let Some(v) = minimize(step + 1, new_stack.as_slice(), is_pops, new_remaining_pops, pre_nums, push_nums) {
                return Some(vec![i].into_iter().chain(v.into_iter()).collect());
            }
        }
    }
    return None;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let program: Vec<Inst> = input.lines().map(parse_inst).collect();
    let is_pops: Vec<bool> = (0..14).map(|i|program[18*i+4].is_divisor_26()).collect();
    let total_pops: usize = is_pops.iter().filter(|b|**b).count();
    let pre_nums: Vec<i64> = (0..14).map(|i|program[18*i+5].get_addi_num()).collect();
    let push_nums: Vec<i64> = (0..14).map(|i|program[18*i+15].get_addi_num()).collect();
    let result = maximize(0, &[], &is_pops, total_pops, &pre_nums, &push_nums).unwrap();
    let model_number = result.iter().fold(0i64,|x,y|x*10+*y);
    if execute(&program, &result).unwrap_or([-1,-1,-1,-1])[3] == 0 {
        println!("Successfully found the highest valid model number: {}", model_number);
    } else {
        println!("Found an invalid model number: {}", model_number);
    }
    let result = minimize(0, &[], &is_pops, total_pops, &pre_nums, &push_nums).unwrap();
    let model_number = result.iter().fold(0i64,|x,y|x*10+*y);
    if execute(&program, &result).unwrap_or([-1,-1,-1,-1])[3] == 0 {
        println!("Successfully found the lower valid model number: {}", model_number);
    } else {
        println!("Found an invalid model number: {}", model_number);
    }
}
