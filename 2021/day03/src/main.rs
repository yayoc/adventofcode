use util;

fn count_nums(nums: &Vec<String>, pos: usize) -> (i32, i32) {
    let mut z_cnt = 0;
    let mut o_cnt = 0;

    for num in nums {
        let bit = num.chars().nth(pos).unwrap();
        if bit == '0' {
            z_cnt += 1;
        } else {
            o_cnt += 1;
        }
    }
    (z_cnt, o_cnt)
}

fn find_common_char(pos: usize, nums: &Vec<String>) -> char {
    let (z_cnt, o_cnt) = count_nums(nums, pos);
    if z_cnt > o_cnt {
        '0'
    } else {
        '1'
    }
}

fn find_least_char(pos: usize, nums: &Vec<String>) -> char {
    let (z_cnt, o_cnt) = count_nums(nums, pos);
    if o_cnt < z_cnt {
        '1'
    } else {
        '0'
    }
}

fn filter_nums(nums: Vec<String>, pos: usize, bit: char) -> Vec<String> {
    nums.into_iter()
        .filter(|x| x.chars().nth(pos).unwrap() == bit)
        .collect()
}

fn rating(nums: &Vec<String>, f: fn(usize, &Vec<String>) -> char) -> String {
    let mut pos = 0;
    let mut res = nums.clone();
    while res.len() != 1 {
        let c = f(pos, &res);
        res = filter_nums(res, pos, c);
        pos += 1;
    }
    res[0].clone()
}

fn main() {
    let nums: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let o_rat = rating(&nums, find_common_char);
    let co2_rat = rating(&nums, find_least_char);

    println!("{}", o_rat);
    println!("{}", co2_rat);

    let res =
        isize::from_str_radix(&o_rat, 2).unwrap() * isize::from_str_radix(&co2_rat, 2).unwrap();

    println!("{:?}", res);
}
