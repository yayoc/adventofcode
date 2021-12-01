use util;

fn main() {
    let lines = util::read_lines("input.txt").expect("can't read input.txt");
    let nums: Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    let mut l = 0;
    let mut r = 3;
    let mut cnt = 0;
    while r < nums.len() {
        if nums[l] < nums[r] {
            cnt += 1;
        }
        l += 1;
        r += 1;
    }

    println!("{}", cnt);
}
