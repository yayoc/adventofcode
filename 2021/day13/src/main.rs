use util;

#[derive(Debug)]
enum Direction {
    X,
    Y,
}

#[derive(Debug)]
struct Instruction {
    line: usize,
    direction: Direction,
}

#[derive(Debug)]
struct Position(usize, usize);

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn parse<'a>(input: Vec<String>) -> (Vec<Vec<&'a str>>, Vec<Instruction>) {
    let mut positions: Vec<Position> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    let mut is_instruction = false;

    for line in input {
        if line.is_empty() {
            is_instruction = true;
            continue;
        }
        if is_instruction {
            let v: Vec<&str> = line.split_whitespace().collect();
            let k: Vec<&str> = v[2].split("=").collect();
            let d = k[0];
            let l = k[1].parse::<usize>().unwrap();

            match d {
                "x" => instructions.push(Instruction {
                    line: l,
                    direction: Direction::X,
                }),
                "y" => instructions.push(Instruction {
                    line: l,
                    direction: Direction::Y,
                }),
                _ => {}
            }
        } else {
            let v: Vec<usize> = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            positions.push(Position(v[0], v[1]));
        }
    }
    let max_x = positions.iter().max_by_key(|p| p.0).unwrap();
    let max_y = positions.iter().max_by_key(|p| p.1).unwrap();

    let mut paper = vec![vec!["."; max_x.0 + 1 as usize]; max_y.1 + 1 as usize];

    // mark
    for y in 0..paper.len() {
        for x in 0..paper[0].len() {
            if positions.contains(&Position(x, y)) {
                paper[y][x] = "#";
            }
        }
    }
    (paper, instructions)
}

fn fold(paper: &mut Vec<Vec<&str>>, instruction: &Instruction) {
    let len_y = paper.len();
    let len_x = paper[0].len();
    match instruction.direction {
        Direction::X => {
            for y in 0..len_y {
                for x in 0..len_x {
                    if x > instruction.line {
                        if paper[y][x] == "#" {
                            paper[y][instruction.line - (x - instruction.line)] = "#";
                        }
                    }
                }
            }

            for y in 0..len_y {
                paper[y].truncate(instruction.line);
            }
        }
        Direction::Y => {
            for y in 0..len_y {
                for x in 0..len_x {
                    if y > instruction.line {
                        if paper[y][x] == "#" {
                            paper[instruction.line - (y - instruction.line)][x] = "#";
                        }
                    }
                }
            }
            paper.truncate(instruction.line)
        }
    }
}

fn count(paper: &Vec<Vec<&str>>) {
    let mut count = 0;
    for y in 0..paper.len() {
        for x in 0..paper[0].len() {
            if paper[y][x] == "#" {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn print(paper: &Vec<Vec<&str>>) {
    for y in 0..paper.len() {
        for x in 0..paper[0].len() {
            if paper[y][x] == "#" {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let (mut paper, instructions) = parse(input);
    for instruction in instructions {
        fold(&mut paper, &instruction);
    }
    print(&paper);
}
