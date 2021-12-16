use std::collections::HashMap;
use util;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn key<'a>(&'a self) -> (&'a i32, &'a i32) {
        (&self.x, &self.y)
    }
}

struct Line {
    start: Pos,
    end: Pos,
}

#[derive(Debug)]
enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

trait Diagram {
    fn points(&self) -> Vec<Pos>;
    fn direction(&self) -> Direction;
}

impl Diagram for Line {
    fn points(&self) -> Vec<Pos> {
        let mut pos = vec![];
        match self.direction() {
            Direction::Top => {
                for i in 0..=self.start.y - self.end.y {
                    pos.push(Pos {
                        x: self.start.x,
                        y: self.start.y - i,
                    });
                }
            }
            Direction::TopRight => {
                for i in 0..=self.start.y - self.end.y {
                    pos.push(Pos {
                        x: self.start.x + i,
                        y: self.start.y - i,
                    });
                }
            }
            Direction::Right => {
                for i in 0..=self.end.x - self.start.x {
                    pos.push(Pos {
                        x: self.start.x + i,
                        y: self.start.y,
                    });
                }
            }
            Direction::BottomRight => {
                for i in 0..=self.end.x - self.start.x {
                    pos.push(Pos {
                        x: self.start.x + i,
                        y: self.start.y + i,
                    });
                }
            }
            Direction::Bottom => {
                for i in 0..=self.end.y - self.start.y {
                    pos.push(Pos {
                        x: self.start.x,
                        y: self.start.y + i,
                    });
                }
            }
            Direction::BottomLeft => {
                for i in 0..=self.end.y - self.start.y {
                    pos.push(Pos {
                        x: self.start.x - i,
                        y: self.start.y + i,
                    });
                }
            }
            Direction::Left => {
                for i in 0..=self.start.x - self.end.x {
                    pos.push(Pos {
                        x: self.start.x - i,
                        y: self.start.y,
                    });
                }
            }
            Direction::TopLeft => {
                for i in 0..=self.start.y - self.end.y {
                    pos.push(Pos {
                        x: self.start.x - i,
                        y: self.start.y - i,
                    });
                }
            }
        }
        println!("start: {:?}", self.start);
        println!("end: {:?}", self.end);
        println!("direction: {:?}", self.direction());
        println!("pointer: {:?}", pos);
        pos
    }

    fn direction(&self) -> Direction {
        let diff_x = self.start.x - self.end.x;
        let diff_y = self.start.y - self.end.y;
        if diff_x == 0 {
            if diff_y > 0 {
                return Direction::Top;
            } else {
                return Direction::Bottom;
            }
        }

        if diff_x < 0 {
            if diff_y == 0 {
                return Direction::Right;
            }
            if diff_y < 0 {
                return Direction::BottomRight;
            }
            return Direction::TopRight;
        } else {
            if diff_y == 0 {
                return Direction::Left;
            }
            if diff_y < 0 {
                return Direction::BottomLeft;
            }
            return Direction::TopLeft;
        }
    }
}

fn parse_vent(s: String) -> (i32, i32, i32, i32) {
    let res = s.replace(" -> ", ",");
    let parsed: Vec<i32> = res.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    (parsed[0], parsed[1], parsed[2], parsed[3])
}

fn main() {
    let vents: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut points: Vec<Pos> = vec![];
    for vent in vents {
        let (x1, y1, x2, y2) = parse_vent(vent);
        let line = Line {
            start: Pos { x: x1, y: y1 },
            end: Pos { x: x2, y: y2 },
        };
        points.append(&mut line.points());
    }

    let mut keyed = HashMap::new();
    for p in &points {
        keyed.entry(p.key()).or_insert(vec![]).push(p)
    }

    let mut cnt = 0;
    for (k, v) in &keyed {
        if v.len() > 1 {
            cnt += 1;
            println!("Key {:?} has {} duplicates!", k, v.len());
        }
    }

    println!("{:?}", cnt);
}
