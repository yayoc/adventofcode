use std::collections::HashMap;
use util;

fn parse<'a>(input: Vec<String>) -> (String, HashMap<String, String>) {
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
            hm.insert(String::from(v[0]), String::from(v[1]));
        } else {
            template = line.clone();
        }
    }

    (template, hm)
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let (mut template, insertions) = parse(input);
    let step = 40;

    for _ in 0..step {
        let mut i = 1;
        let mut inserted = String::from("");
        while i < template.len() {
            let mut key = String::from("");
            let chars_vec: Vec<char> = template.chars().collect();
            let prev_char = chars_vec[i - 1];
            let current_char = chars_vec[i];
            key.push(prev_char);
            key.push(current_char);
            if i == 1 {
                inserted.push(prev_char);
            }
            match insertions.get(&key) {
                Some(v) => {
                    // inserted.push(prev_char);
                    inserted.push(v.chars().next().unwrap());
                    inserted.push(current_char);
                }
                None => {}
            }
            i += 1;
        }
        template = inserted;
    }
    let counter = util::counter(template.chars());
    let most_common = counter.iter().max_by_key(|x| x.1).unwrap();
    let least_common = counter.iter().min_by_key(|x| x.1).unwrap();
    println!("{:?}", most_common);
    println!("{:?}", least_common);
    println!("{:?}", most_common.1 - least_common.1);
}
