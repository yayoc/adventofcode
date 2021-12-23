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

    let mut risk_levels = 0;
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
                risk_levels += v.unwrap() + 1;
            }
            x += 1;
        }
        y += 1;
    }
    println!("{}", risk_levels);
}
