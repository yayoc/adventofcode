use util;

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut timers: Vec<i32> = input[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for _ in 1..=80 {
        let mut cnt = 0;
        println!("{:?}", timers);
        for timer in timers.iter_mut() {
            if *timer == 0 {
                *timer = 6;
                cnt += 1;
            } else {
                *timer -= 1;
            }
        }
        timers.append(&mut vec![8; cnt]);
    }
    println!("{:?}", timers.len());
}
