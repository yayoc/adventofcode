fn read(decoded: &[char], i: &mut usize, len: usize) -> usize {
    let mut str = String::from("");
    for k in *i..*i + len {
        str.push(decoded[k]);
    }
    *i += len;
    usize::from_str_radix(&str, 2).unwrap()
}

#[derive(Debug)]
pub struct Packet {
    version: usize,
    type_id: usize,
    sub: Vec<Packet>,
}

pub fn parse(decoded: &[char]) -> Option<(Packet, usize)> {
    if decoded.len() < 11 {
        return None;
    }

    let mut pos: usize = 0;
    let version = read(decoded, &mut pos, 3);
    let type_id = read(decoded, &mut pos, 3);

    // literal
    if type_id == 4 {
        let remain = decoded.get(pos..).unwrap();
        let mut is_last = false;
        // consume literal packets
        let mut k = 0;
        while k < remain.len() {
            if k % 5 == 0 && is_last {
                break;
            }
            if k % 5 == 0 && *remain.get(k).unwrap() == '0' {
                is_last = true;
            }
            k += 1;
        }
        pos += k;
        return Some((
            Packet {
                version,
                type_id,
                sub: vec![],
            },
            pos,
        ));
    }

    // operator
    let length_type_id_char = decoded[6];
    pos += 1;
    if length_type_id_char == '0' {
        let k = read(decoded, &mut pos, 15);
        let mut sub = vec![];
        while pos < k + pos {
            if let Some((packet, p)) = parse(&decoded[pos..]) {
                sub.push(packet);
                pos += p;
            } else {
                break;
            }
        }

        return Some((
            Packet {
                version,
                type_id,
                sub,
            },
            pos,
        ));
    }
    let num_sub = read(decoded, &mut pos, 11);
    println!("num_sub: {}", num_sub);
    let mut sub = vec![];
    for _ in 0..num_sub {
        if let Some((packet, p)) = parse(&decoded[pos..]) {
            sub.push(packet);
            pos += p;
        }
    }

    Some((
        Packet {
            version,
            type_id,
            sub,
        },
        pos,
    ))
}

fn sum_version(packet: Packet) -> usize {
    let mut stack = vec![packet];
    let mut res: usize = 0;

    while !stack.is_empty() {
        let mut p = stack.pop().unwrap();
        res += p.version;

        if !p.sub.is_empty() {
            stack.append(&mut p.sub);
        }
    }

    res
}

pub fn solve(input: &str) -> usize {
    let mut decoded = String::new();
    for c in input.trim().chars() {
        // hex to binary
        decoded += &format!("{:04b}", c.to_digit(16).unwrap());
    }
    let decoded: Vec<char> = decoded.chars().collect();
    if let Some((packet, _)) = parse(&decoded) {
        sum_version(packet)
    } else {
        0
    }
}

pub fn solve2(input: &str) -> usize {}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let ans = solve(&input[0]);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(super::solve("D2FE28"), 6);
        assert_eq!(super::solve("38006F45291200"), 9);
        assert_eq!(super::solve("EE00D40C823060"), 14);
        assert_eq!(super::solve("8A004A801A8002F478"), 16);
        assert_eq!(super::solve("620080001611562C8802118E34"), 12);
        assert_eq!(super::solve("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::solve("A0016C880162017C3686B18A3D4780"), 31);
    }

    fn example2() {}
}
