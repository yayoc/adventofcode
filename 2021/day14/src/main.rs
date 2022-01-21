use std::collections::HashMap;
use util;

fn parse<'a>(input: Vec<String>) -> (String, HashMap<String, char>) {
    let mut is_insertion = false;
    let mut template: String = String::from("");
    let mut hm = HashMap::new();
    for line in input {
        if line.is_empty() {
            is_insertion = true;
            continue;
        }
        if is_insertion {
            let v: Vec<&str> = line.split(" -> ").collect();
            let chars: Vec<char> = v[1].chars().collect();
            hm.insert(String::from(v[0]), chars[0]);
        } else {
            template = line.clone();
        }
    }

    (template, hm)
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let (template, insertions) = parse(input);
    let step = 40;

    let mut pair_counter: HashMap<String, usize> = HashMap::new();
    let chars_vec: Vec<char> = template.chars().collect();
    let mut i = 1;
    while i < template.len() {
        let mut key = String::from("");
        let prev_char = chars_vec[i - 1];
        let current_char = chars_vec[i];
        key.push(prev_char);
        key.push(current_char);
        *pair_counter.entry(key).or_insert(0) += 1;
        i += 1;
    }

    for _ in 0..step {
        let mut new_pair_counter = HashMap::new();
        for (key, value) in pair_counter.iter() {
            match insertions.get(key) {
                Some(v) => {
                    let chars_vec: Vec<char> = key.chars().collect();
                    let left: String = vec![chars_vec[0], *v].into_iter().collect();
                    let right: String = vec![*v, chars_vec[1]].into_iter().collect();
                    *new_pair_counter.entry(left).or_insert(0) += *value;
                    *new_pair_counter.entry(right).or_insert(0) += *value;
                }
                None => {}
            }
        }
        pair_counter = new_pair_counter;
    }

    let mut char_counter = HashMap::new();

    for (key, value) in pair_counter {
        let chars_vec: Vec<char> = key.chars().collect();
        *char_counter.entry(chars_vec[0]).or_insert(0) += value;
        *char_counter.entry(chars_vec[1]).or_insert(0) += value;
        println!("char_counter: {:?}", char_counter);
    }
    println!("char_counter: {:?}", char_counter);

    let most_common = char_counter.iter().max_by_key(|x| x.1).unwrap();
    let least_common = char_counter.iter().min_by_key(|x| x.1).unwrap();
    println!("most_common: {:?}", most_common);
    println!("least_common: {:?}", least_common);
    println!("sub: {:?}", most_common.1 / 2 - least_common.1 / 2 + 1);
}
