use std::collections::HashMap;
use util;

fn main() {
    let nums: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let len = nums[0].len();
    let mut map = HashMap::new();
    map.insert("0", 0);
    map.insert("1", 0);
    let mut counter = vec![map; len];

    for num in nums {
        for (i, c) in num.chars().enumerate() {
            let count = counter.get_mut(i).expect("");
            match c {
                '0' => *count.get_mut("0").unwrap() += 1,
                '1' => *count.get_mut("1").unwrap() += 1,
                _ => {}
            }
        }
    }

    let mut gamma = String::from("");
    for c in counter {
        if c["0"] > c["1"] {
            gamma.push_str("0")
        } else {
            gamma.push_str("1")
        }
    }

    let mut epsilon = String::from("");
    for c in gamma.chars() {
        if c == '0' {
            epsilon.push_str("1")
        } else {
            epsilon.push_str("0")
        }
    }

    let res =
        isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{:?}", res);
}
