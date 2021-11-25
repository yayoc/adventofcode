use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
