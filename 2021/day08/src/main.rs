use util;

enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

fn str_to_digit(s: &str) -> Option<Digit> {
    match s.len() {
        2 => Some(Digit::One),
        4 => Some(Digit::Four),
        3 => Some(Digit::Seven),
        7 => Some(Digit::Eight),
        _ => None,
    }
}

struct Entry {
    // singals: String,
    output: Vec<Digit>,
}

fn main() {
    let lines: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut entries: Vec<Entry> = vec![];
    for line in lines {
        let mut parsed = line.split(" | ");
        let output: Vec<Digit> = parsed
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter_map(|s| str_to_digit(s))
            .collect();
        entries.push(Entry { output })
    }
    let count = entries.iter().fold(0, |acc, x| acc + x.output.len());
    println!("{:?}", count);
}
