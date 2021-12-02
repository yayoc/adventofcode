use util;

fn main() {
    let lines = util::read_lines("input.txt").expect("can't read input.txt");
    let commands: Vec<String> = lines
        .map(|l| l.unwrap().parse::<String>().unwrap())
        .collect();

    let mut x = 0;
    let mut y = 0;

    for command in commands {
        let mut iter = command.split_whitespace();
        match iter.next() {
            Some("forward") => x += iter.next().unwrap().parse::<i32>().unwrap(),
            Some("up") => y -= iter.next().unwrap().parse::<i32>().unwrap(),
            Some("down") => y += iter.next().unwrap().parse::<i32>().unwrap(),
            _ => {}
        }
    }

    println!("{}", x * y);
}
