use util;

struct Cell {
    num: i32,
    marked: bool,
}

trait Bingo {
    fn mark(&mut self, num: i32);
    fn check(&self) -> bool;
    fn check_vertical(&self) -> bool;
    fn check_horizontal(&self) -> bool;
    fn check_oblique(&self) -> bool;
    fn sum_unmarked(&self) -> i32;
}

struct Board {
    items: Vec<Cell>,
}

impl Bingo for Board {
    fn mark(&mut self, num: i32) {
        let mut items: Vec<Cell> = Vec::new();
        for item in &self.items {
            if item.num == num {
                items.push(Cell {
                    num: item.num,
                    marked: true,
                })
            } else {
                items.push(Cell {
                    num: item.num,
                    marked: item.marked,
                });
            }
        }
        self.items = items;
    }

    fn check(&self) -> bool {
        self.check_horizontal() || self.check_vertical() || self.check_oblique()
    }

    fn check_horizontal(&self) -> bool {
        let rows = self.items.chunks(5);
        for row in rows {
            let mut cnt = 0;
            for c in row {
                if c.marked {
                    cnt += 1;
                }
            }
            if cnt == 5 {
                return true;
            }
        }
        false
    }

    fn check_vertical(&self) -> bool {
        let mut columns: Vec<Vec<&Cell>> = vec![vec!(); 5];
        for (i, item) in self.items.iter().enumerate() {
            let m = i % 5;
            columns[m].push(item);
        }

        for column in columns {
            let mut cnt = 0;
            for c in &column {
                if c.marked {
                    cnt += 1;
                }
            }
            if cnt == 5 {
                return true;
            }
        }
        false
    }

    fn check_oblique(&self) -> bool {
        let left = vec![0, 6, 12, 18, 24];
        let right = vec![4, 8, 12, 16, 20];

        let mut cnt = 0;

        for i in left {
            if self.items[i].marked {
                cnt += 1;
            } else {
                break;
            }
        }

        if cnt == 5 {
            return true;
        }

        for i in right {
            if !self.items[i].marked {
                return false;
            }
        }

        return true;
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for item in &self.items {
            if !item.marked {
                sum += item.num;
            }
        }
        sum
    }
}

fn main() {
    let data: Vec<String> = util::parse_input("input.txt").expect("can't parse input");

    let draw_nums = data[0].split(",").map(|x| x.parse::<i32>().unwrap());
    let mut boards: Vec<Board> = Vec::new();

    let mut tmp_cells: Vec<Cell> = Vec::new();
    for (i, d) in data.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if d == "" && tmp_cells.len() == 25 {
            boards.push(Board { items: tmp_cells });
            tmp_cells = Vec::new();
        } else {
            for num in d.split_whitespace() {
                tmp_cells.push(Cell {
                    num: num.parse::<i32>().unwrap(),
                    marked: false,
                })
            }
        }
    }

    'outer: for num in draw_nums {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.check() {
                println!("{}", board.sum_unmarked());
                println!("{}", num);
                println!("{}", board.sum_unmarked() * num);
                break 'outer;
            }
        }
    }
}
