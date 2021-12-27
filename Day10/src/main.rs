use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut syntax_score = 0u64;
    let mut completion_scores: Vec<u64> = Vec::new();
    'outer: for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '[' || c == '<' || c == '(' || c == '{' {
                stack.push(c);
            } else {
                if stack.pop().unwrap() != match c {
                    ']' => '[',
                    '>' => '<',
                    ')' => '(',
                    '}' => '{',
                    _ => ' '
                } {
                    let error_value = match c {
                        ']' => 57,
                        '>' => 25137,
                        ')' => 3,
                        '}' => 1197,
                        _ => 0
                    };
                    syntax_score += error_value;
                    continue 'outer;
                }
            }
        }
        let mut completion_score = 0u64;
        while !stack.is_empty() {
            completion_score *= 5;
            completion_score += match stack.pop().unwrap() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            };
        }
        completion_scores.push(completion_score);
    }
    println!("Total syntax error score: {}", syntax_score);
    completion_scores.sort();
    println!("Middle completion error score: {}", completion_scores[completion_scores.len() / 2]);
}
