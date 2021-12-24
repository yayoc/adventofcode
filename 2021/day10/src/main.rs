use std::collections::HashMap;
use util;

fn get_open_close() -> HashMap<char, char> {
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')])
}

/**
 * return None if subsystem is corrupted, otherwise, return remaining stack
 */
fn check_corrupted(s: &str) -> Option<Vec<char>> {
    let open_close = get_open_close();
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if open_close.get(&c).is_some() {
            stack.push(c)
        } else {
            let top = stack.pop().unwrap();
            let expected_close = open_close.get(&top).unwrap();
            if *expected_close != c {
                return None;
            }
        }
    }
    Some(stack)
}

fn get_score(stack: &mut Vec<char>) -> u64 {
    let open_close = get_open_close();
    let mut score = 0;
    while stack.len() > 0 {
        let top = stack.pop().unwrap();
        let expected_close = open_close.get(&top).unwrap();
        let point = match expected_close {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
        score = score * 5;
        score += point;
    }
    score
}

fn main() {
    let subsystems: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut scores = vec![];
    for subsystem in &subsystems {
        match check_corrupted(subsystem) {
            Some(mut stack) => scores.push(get_score(&mut stack)),
            _ => {}
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
