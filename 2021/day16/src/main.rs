fn hex_to_binary(s: char) -> &'static str {
    match s {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

pub fn parse(i: &mut usize, decoded: &[char]) -> usize {
    if decoded.len() < 6 {
        return 0;
    }
    let version_id_binary = format!("{}{}{}", decoded[0], decoded[1], decoded[2]);
    let version_id = usize::from_str_radix(&version_id_binary, 2).unwrap();
    let type_id_binary = format!("{}{}{}", decoded[3], decoded[4], decoded[5]);
    let type_id = usize::from_str_radix(&type_id_binary, 2).unwrap();

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
        return version_id + parse(i, remain);
    }

    // operator
    /*
    let length_type_id_char = chars_vec[6];
    if length_type_id_char == '1' {
        let num_binary: String = chars_vec.get(7..18).unwrap().iter().collect();
        let num = usize::from_str_radix(&num_binary, 2).unwrap();
        let chunks = chars_vec.get(19..).unwrap().to_vec().chunks(11);
        let chunks_vec: Vec<Vec<char>> = chunks.collect();
    }
    */
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
        assert_eq!(super::solve("D2FE28"), 6);
        /*
        assert_eq!(super::parse("8A004A801A8002F478"), 16);
        assert_eq!(super::parse("620080001611562C8802118E34"), 12);
        assert_eq!(super::parse("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::parse("A0016C880162017C3686B18A3D4780"), 31);
        */
    }
}
