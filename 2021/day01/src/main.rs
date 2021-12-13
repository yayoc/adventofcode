use util;

fn main() {
    let nums: Vec<i32> = util::parse_input("input.txt").expect("can't parse input");

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
