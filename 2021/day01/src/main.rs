use util;

fn main() {
    let lines = util::read_lines("input.txt").expect("can't read input.txt");
    let nums: Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    let mut cnt = 0;
    let mut prev = 0;
    for (i, num) in nums.iter().enumerate() {
        if i == 0 {
            prev = *num;
            continue;
        }
        if num > &prev {
            cnt += 1;
        }
        prev = *num;
    }
    println!("{}", cnt);
}
