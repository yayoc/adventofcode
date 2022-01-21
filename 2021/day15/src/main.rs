use util;

fn parse(input: Vec<String>) -> Vec<Vec<usize>> {
    let len_y = input.len();
    let len_x = input[0].len();

    let mut levels = vec![vec![0; len_x]; len_y];
    for y in 0..input.len() {
        let row = &input[y];
        for (x, c) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            levels[y][x] = c as usize;
        }
    }
    levels
}

#[derive(Clone, Debug, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    fn has_right(&self, max: usize) -> bool {
        self.x < max
    }

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn has_bottom(&self, max: usize) -> bool {
        self.y < max
    }

    fn bottom(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
}

type Path = Vec<Position>;

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let levels = parse(input);
    let len_y = levels.len();
    let len_x = levels[0].len();

    fn get_all_paths(
        start: Position,
        end: Position,
        path: &mut Path,
        max_x: usize,
        max_y: usize,
    ) -> Vec<Path> {
        path.push(start);

        if start == end {
            println!("{:?}", path);
            return vec![path.clone()];
        }
        let mut right_paths: Vec<Path> = vec![];
        let mut bottom_paths: Vec<Path> = vec![];

        let mut original_path: Path = vec![start];
        if start.has_right(max_x) {
            right_paths = get_all_paths(start.right(), end, path, max_x, max_y)
                .iter_mut()
                .map(|path| {
                    original_path.append(path);
                    return original_path.clone();
                })
                .collect();
        }
        if start.has_bottom(max_y) {
            bottom_paths = get_all_paths(start.bottom(), end, path, max_x, max_y)
                .iter_mut()
                .map(|path| {
                    original_path.append(path);
                    return original_path.clone();
                })
                .collect();
        }

        right_paths.append(&mut bottom_paths);
        return right_paths;
    }

    let start = Position { x: 0, y: 0 };
    let end = Position {
        x: len_x - 1,
        y: len_y - 1,
    };

    let mut path = vec![];
    let paths = get_all_paths(start, end, &mut path, len_x - 1, len_y - 1);

    for path in paths {
        // println!("{:?}", path);
    }
}
