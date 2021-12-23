use util;

fn main() {
    let rows: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let rows_len = rows.len();
    let columns_len = rows.get(0).unwrap().chars().count();
    let mut heightmap: Vec<Vec<Option<u32>>> = vec![vec![None; columns_len + 2]; rows_len + 2];
    for (i, row) in rows.iter().enumerate() {
        for (k, c) in row.chars().map(|c| c.to_digit(10)).enumerate() {
            heightmap[i + 1][k + 1] = c;
        }
    }

    let mut size = vec![];
    let mut y = 1;
    while y <= rows_len {
        let mut x = 1;
        while x <= columns_len {
            let v = heightmap[y][x];
            let (top, right, bottom, left) = (
                heightmap[y - 1][x],
                heightmap[y][x + 1],
                heightmap[y + 1][x],
                heightmap[y][x - 1],
            );

            let min = vec![top, right, bottom, left]
                .into_iter()
                .filter_map(|s| s)
                .min()
                .unwrap();

            if v.unwrap() < min {
                fn explore(
                    map: &Vec<Vec<Option<u32>>>,
                    x: usize,
                    y: usize,
                    visited: &mut Vec<(usize, usize)>,
                ) -> u32 {
                    if visited.contains(&(x, y)) {
                        return 0;
                    }

                    let val = map[y][x];
                    match val {
                        Some(9) => return 0,
                        None => return 0,
                        _ => {}
                    }
                    visited.push((x, y));

                    let top_count = explore(map, x, y - 1, visited);
                    let right_count = explore(map, x + 1, y, visited);
                    let bottom_count = explore(map, x, y + 1, visited);
                    let left_count = explore(map, x - 1, y, visited);
                    return 1 + top_count + right_count + bottom_count + left_count;
                }
                let visited: &mut Vec<(usize, usize)> = &mut vec![];
                size.push(explore(&heightmap, x, y, visited));
            }
            x += 1;
        }
        y += 1;
    }
    size.sort_by(|a, b| b.cmp(a));
    println!("{}", size[0] * size[1] * size[2]);
}
