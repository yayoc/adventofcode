use util;

fn step_cost(mut num: u128) -> u128 {
    let mut cost = 0;
    while num > 0 {
        cost += num;
        num -= 1;
    }
    cost
}

fn abs_diff(j: u128, k: u128) -> u128 {
    if j < k {
        k - j
    } else {
        j - k
    }
}

fn fuel_cost(positions: &Vec<u128>, pos: u128) -> u128 {
    positions.iter().map(|p| step_cost(abs_diff(pos, *p))).sum()
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut positions: Vec<u128> = input[0]
        .split(",")
        .map(|v| v.parse::<u128>().unwrap())
        .collect();
    positions.sort();
    println!("positions: {:?}", positions);
    let last = positions[positions.len() - 1];
    println!("last: {:?}", last);
    let mut min = u128::max_value();

    for i in 0..=last {
        let cost = fuel_cost(&positions, i);
        if cost < min {
            min = cost;
        }
    }

    println!("{}", min);
}
