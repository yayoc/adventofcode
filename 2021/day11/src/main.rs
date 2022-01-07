use util;

#[derive(Clone, Debug)]
struct Pos {
    x: i8,
    y: i8,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Debug)]
struct Octopus {
    pos: Pos,
    level: u16,
}

impl Octopus {
    fn increment(&mut self) {
        self.level = self.level + 1;
    }

    fn reset(&mut self) {
        self.level = 0;
    }
}

fn get_adjacents(pos: &Pos, max_x: i8, max_y: i8) -> Vec<Pos> {
    let top = Pos {
        x: pos.x,
        y: pos.y - 1,
    };
    let top_right = Pos {
        x: pos.x + 1,
        y: pos.y - 1,
    };
    let right = Pos {
        x: pos.x + 1,
        y: pos.y,
    };
    let bottom_right = Pos {
        x: pos.x + 1,
        y: pos.y + 1,
    };
    let bottom = Pos {
        x: pos.x,
        y: pos.y + 1,
    };
    let bottom_left = Pos {
        x: pos.x - 1,
        y: pos.y + 1,
    };
    let left = Pos {
        x: pos.x - 1,
        y: pos.y,
    };
    let top_left = Pos {
        x: pos.x - 1,
        y: pos.y - 1,
    };

    vec![
        top,
        top_right,
        right,
        bottom_right,
        bottom,
        bottom_left,
        left,
        top_left,
    ]
    .into_iter()
    .filter(|p| p.x >= 0 && p.y >= 0 && p.x <= max_x && p.y <= max_y)
    .collect()
}

const MAX_ENEGY: u16 = 9;

fn flash(octopuses: &mut Vec<Octopus>, flashed: &mut Vec<Pos>, pos: Pos, max_x: i8, max_y: i8) {
    let mut adjacents = get_adjacents(&pos, max_x, max_y);
    while adjacents.len() > 0 {
        let target_pos = adjacents.pop().unwrap();
        let adjacent = octopuses
            .iter_mut()
            .find(|o| o.pos.x == target_pos.x && o.pos.y == target_pos.y)
            .unwrap();
        adjacent.increment();
        if adjacent.level > MAX_ENEGY && !flashed.contains(&adjacent.pos) {
            flashed.push(adjacent.pos.clone());
            adjacents.append(&mut get_adjacents(&adjacent.pos, max_x, max_y))
        }
    }
}

fn increment(octopuses: &mut Vec<Octopus>) {
    for oct in octopuses.iter_mut() {
        oct.increment();
    }
}

fn reset(octopuses: &mut Vec<Octopus>) {
    for oct in octopuses {
        if oct.level > 9 {
            oct.reset()
        }
    }
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut octopuses: Vec<Octopus> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, r) in input.iter().enumerate() {
        for (x, l) in r.chars().enumerate() {
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            octopuses.push(Octopus {
                pos: Pos {
                    x: x as i8,
                    y: y as i8,
                },
                level: l.to_digit(10).expect("no number") as u16,
            })
        }
    }

    let step = 100;
    let mut cnt = 0;
    for _ in 0..step {
        increment(&mut octopuses);
        let mut flashed = vec![];
        for k in 0..octopuses.len() {
            let oct = octopuses[k].clone();
            if oct.level > MAX_ENEGY && !flashed.contains(&oct.pos) {
                flashed.push(oct.pos.clone());
                flash(
                    &mut octopuses,
                    &mut flashed,
                    oct.pos,
                    max_x as i8,
                    max_y as i8,
                );
            }
        }
        cnt += flashed.len();
        reset(&mut octopuses);
    }
    println!("{:?}", cnt);
}
