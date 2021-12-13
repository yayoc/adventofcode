use util;

fn main() {
    let commands: Vec<String> = util::parse_input("input.txt").expect("can't parse input");

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for command in commands {
        let mut iter = command.split_whitespace();
        match iter.next() {
            Some("forward") => {
                let v = iter.next().unwrap().parse::<i32>().unwrap();
                x += v;
                y += v * aim;
            }
            Some("up") => aim -= iter.next().unwrap().parse::<i32>().unwrap(),
            Some("down") => aim += iter.next().unwrap().parse::<i32>().unwrap(),
            _ => {}
        }
    }

    println!("{}", x * y);
}
