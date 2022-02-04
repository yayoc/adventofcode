fn num(decoded: &[char], start: usize, len: usize) -> usize {
    let mut str = String::from("");
    for i in start..start + len {
        str.push(decoded[i]);
    }
    usize::from_str_radix(&str, 2).unwrap()
}

pub fn parse(i: &mut usize, decoded: &[char]) -> usize {
    if decoded.len() < 6 {
        return 0;
    }
    let version_id = num(decoded, 0, 3);
    let type_id = num(decoded, 3, 3);

    // literal
    if type_id == 4 {
        let remain = decoded.get(*i + 6..).unwrap();
        let mut is_last = false;
        // consume literal packets
        while i < &mut remain.len() {
            if *i % 5 == 0 && is_last {
                break;
            }
            if *i % 5 == 0 && *remain.get(*i).unwrap() == '0' {
                is_last = true;
            }
            *i += 1;
        }
        let remain = decoded.get(6 + *i..).unwrap();
        return version_id + parse(&mut 0, remain);
    }

    // operator
    let length_type_id_char = decoded[*i + 6];
    if length_type_id_char == '0' {
        let num_binary: String = decoded.get(*i + 7..*i + 22).unwrap().iter().collect();
        let num = usize::from_str_radix(&num_binary, 2).unwrap();
        let sub = decoded.get(*i + 22..*i + 22 + num).unwrap();
        return version_id + parse(&mut 0, sub);
    } else if length_type_id_char == '1' {
        let num_binary: String = decoded.get(*i + 7..*i + 18).unwrap().iter().collect();
        let num_sub = usize::from_str_radix(&num_binary, 2).unwrap();
        let remain = decoded.get(*i + 18..*i + 18 + num_sub * 11).unwrap();
        return version_id + parse(&mut 0, remain);
    }

    0
}

pub fn solve(input: &str) -> usize {
    let mut decoded = String::new();
    for c in input.trim().chars() {
        // hex to binary
        decoded += &format!("{:04b}", c.to_digit(16).unwrap());
    }
    let decoded: Vec<char> = decoded.chars().collect();
    let mut i = 0;
    let mut version_sum: usize = 0;
    while i < input.len() {
        version_sum += parse(&mut i, &decoded);
    }
    version_sum
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let ans = solve(&input[0]);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        //assert_eq!(super::solve("D2FE28"), 6);
        //assert_eq!(super::solve("38006F45291200"), 9);
        assert_eq!(super::solve("EE00D40C823060"), 14);
        //assert_eq!(super::solve("8A004A801A8002F478"), 16);
        /*
        assert_eq!(super::solve("620080001611562C8802118E34"), 12);
        assert_eq!(super::solve("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::solve("A0016C880162017C3686B18A3D4780"), 31);
        */
    }
}
