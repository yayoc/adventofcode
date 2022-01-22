use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn parse(input: Vec<String>) -> (HashMap<Pos, usize>, usize, usize) {
    let mut levels: HashMap<Pos, usize> = HashMap::new();
    let len_y = input.len();
    let len_x = input[0].len();

    for (y, _) in input.iter().enumerate() {
        let row = &input[y];
        for (x, c) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            for i_x in 0..5 {
                for i_y in 0..5 {
                    let new_x = x + (i_x * len_x);
                    let new_y = y + (i_y * len_y);
                    let mut new_level = c as usize + i_x + i_y as usize;
                    if new_level > 9 {
                        new_level -= 9;
                    }

                    levels.insert(Pos { x: new_x, y: new_y }, new_level);
                }
            }
        }
    }

    (levels, len_y * 5, len_x * 5)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: Pos,
    level: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: Pos,
    total_level: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.total_level.cmp(&self.total_level)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let (levels, len_y, len_x) = parse(input);

    let start = Pos { x: 0, y: 0 };
    let end = Pos {
        x: len_x - 1,
        y: len_y - 1,
    };

    // Dijkstra's shortest path algorithm.

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut dist: HashMap<Pos, usize> = HashMap::new();
    for pos in levels.keys() {
        dist.insert(*pos, usize::MAX);
    }

    queue.push(State {
        pos: start,
        total_level: 0,
    });

    while let Some(State { pos, total_level }) = queue.pop() {
        if pos == end {
            println!("lowest level {:?}", total_level);
        }

        if total_level > *dist.get(&pos).unwrap() {
            continue;
        }

        let adj_positions = vec![
            Pos {
                x: pos.x,
                y: pos.y - 1,
            },
            Pos {
                x: pos.x + 1,
                y: pos.y,
            },
            Pos {
                x: pos.x,
                y: pos.y + 1,
            },
            Pos {
                x: pos.x - 1,
                y: pos.y,
            },
        ];

        let positions: Vec<&Pos> = adj_positions
            .iter()
            .filter(|p| p.x >= 0 && p.x < len_x && p.y >= 0 && p.y < len_y)
            .collect();

        for pos in positions {
            let next_node_level = levels.get(pos).unwrap();
            let next = State {
                total_level: total_level + next_node_level,
                pos: *pos,
            };

            if next.total_level < *dist.get(pos).unwrap() {
                queue.push(next);
                dist.insert(next.pos, next.total_level);
            }
        }
    }
}
