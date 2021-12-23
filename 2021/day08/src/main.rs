use std::collections::HashMap;
use std::hash::Hash;
use std::mem::swap;
use util;

// deduction
// 1. find 3 by comparing with 1's chars. The diff between 3's chars 1's chars should be 3 chars.
// 2. find 9 by comparing with 3's chars. The diff between 9's chars and 3's chars should be 1 char.
// 3. find 5 by comparing with 9's chars. The diff between 5's chars and 9's chars should be 1 char.
// 4. find 2 from len 5 remain.
// 5. find 6 by comparing with 5's chars. The diff between 6's chars and 5's chars should be 1 char.
// 6. find 0 from len 6 remain.

fn counter<T, I>(it: I) -> HashMap<T, usize>
where
    T: Eq + Hash,
    I: Iterator<Item = T>,
{
    let mut count_by_element = HashMap::new();
    for e in it {
        *count_by_element.entry(e).or_insert(0) += 1;
    }
    count_by_element
}

fn abs_diff(a: &usize, b: &usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn diff_count<'a>(mut a: &'a str, mut b: &'a str) -> u8 {
    if a.len() < b.len() {
        swap(&mut a, &mut b);
    }

    let counter_a = counter(a.chars());
    let counter_b = counter(b.chars());

    let mut cnt: u8 = 0;
    for (c, n_a) in &counter_a {
        let n_b = counter_b.get(c).unwrap_or(&0);
        cnt += abs_diff(n_a, n_b) as u8;
    }
    cnt
}

fn main() {
    let lines: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut total = 0;
    for line in lines {
        let mut parsed = line.split(" | ");
        let input = parsed.nth(0).unwrap().split_whitespace();
        // key chars, value digit
        let mut hm = HashMap::new();
        let mut one_str = "";
        let mut five_len = vec![];
        let mut six_len = vec![];

        for s in input {
            match s.len() {
                2 => {
                    one_str = s;
                    hm.insert(s, 1);
                }
                3 => {
                    hm.insert(s, 7);
                }
                4 => {
                    hm.insert(s, 4);
                }
                5 => five_len.push(s),
                6 => six_len.push(s),
                7 => {
                    hm.insert(s, 8);
                }
                _ => {}
            }
        }

        let three_str = five_len
            .iter()
            .find(|s| diff_count(s, one_str) == 3)
            .unwrap();
        hm.insert(three_str, 3);

        let nine_str = six_len
            .iter()
            .find(|s| diff_count(s, three_str) == 1)
            .unwrap();
        hm.insert(nine_str, 9);

        let five_str = five_len
            .iter()
            .find(|s| diff_count(nine_str, s) == 1 && *s != three_str)
            .unwrap();
        hm.insert(five_str, 5);

        let six_str = six_len
            .iter()
            .find(|s| diff_count(s, five_str) == 1 && *s != nine_str)
            .unwrap();
        hm.insert(six_str, 6);

        let two_str = five_len
            .iter()
            .find(|&&s| (s != *three_str && s != *five_str))
            .unwrap();
        hm.insert(two_str, 2);

        let zero_str = six_len
            .iter()
            .find(|&&s| (s != *six_str && s != *nine_str))
            .unwrap();
        hm.insert(zero_str, 0);

        println!("{:?}", hm);

        let output = parsed.nth(0).unwrap().split_whitespace();

        let mut cnt = 0;
        for (i, s) in output.enumerate() {
            for (key, val) in &hm {
                if diff_count(key, s) == 0 {
                    println!("diff 0 when key: {} s: {}", key, s);
                    let base: i32 = 10;
                    cnt += base.pow(3 - i as u32) * (*val as i32);
                }
            }
        }
        total += cnt;
    }
    println!("{}", total);
}
