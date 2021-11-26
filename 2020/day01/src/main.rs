use std::collections::HashMap;
use util;

fn main() {
    if let Ok(lines) = util::read_lines("input.txt") {
        let nums: Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

        for num in &nums {
            let remain = 2020 - num;
            if let Some((a, b)) = find_two_sum(remain, &nums) {
                println!("{}, {}, {}", a, b, num);
                println!("{}", a * b * num);
            }
        }
    }
}

fn find_two_sum(target: i32, nums: &Vec<i32>) -> Option<(i32, i32)> {
    let mut hm = HashMap::new();
    for num in nums {
        hm.insert(target - num, *num);
    }

    for num in nums {
        if hm.contains_key(num) {
            return Some((*num, *hm.get(num).unwrap()));
        }
    }
    None
}
