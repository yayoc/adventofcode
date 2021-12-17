use util;

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut positions: Vec<i32> = input[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    positions.sort();
    println!("{:?}", positions);
    let mid = positions[(positions.len() / 2) as usize];
    println!("{:?}", mid);

    let sum: i32 = positions.iter().map(|v| (v - mid).abs()).sum();
    println!("{}", sum);
}
