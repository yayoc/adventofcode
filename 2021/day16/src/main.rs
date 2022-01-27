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

fn get_version(bits: &str) -> usize {
    let chars_vec: Vec<char> = bits.chars().collect();
    let version_id_binary = format!("{}{}{}", chars_vec[0], chars_vec[1], chars_vec[2]);
    usize::from_str_radix(&version_id_binary, 2).unwrap()
}

fn get_sub_packets(bits: &str) -> Vec<String> {
    let chars_vec: Vec<char> = bits.chars().collect();
    let type_id_binary = format!("{}{}{}", chars_vec[3], chars_vec[4], chars_vec[5]);
    let type_id = usize::from_str_radix(&type_id_binary, 2).unwrap();

    if type_id == 4 {
        return chars_vec
            .get(6..)
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .collect();
    }

    let length_type_id_char = chars_vec[6];
    if length_type_id_char == '1' {
        let num_binary: String = chars_vec.get(7..18).unwrap().iter().collect();
        let num = usize::from_str_radix(&num_binary, 2).unwrap();

        let remain = chars_vec.get(19).unwrap();
    }

    // If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
    let num_binary: String = chars_vec.get(7..22).unwrap().iter().collect();
    let num = usize::from_str_radix(&num_binary, 2).unwrap();

    vec![]
}

fn parse(bits: &str) -> usize {
    let chars_vec: Vec<char> = bits.chars().collect();

    let version_id_binary = format!("{}{}{}", chars_vec[0], chars_vec[1], chars_vec[2]);
    let version_id = usize::from_str_radix(&version_id_binary, 2).unwrap();
    let type_id_binary = format!("{}{}{}", chars_vec[3], chars_vec[4], chars_vec[5]);
    let type_id = usize::from_str_radix(&type_id_binary, 2).unwrap();

    // literal
    if type_id == 4 {
        let remain: String = chars_vec.get(6..).unwrap().iter().collect();
        return version_id + parse(&remain);
    }

    // operator
    let length_type_id_char = chars_vec[6];
    if length_type_id_char == '1' {
        let num_binary: String = chars_vec.get(7..18).unwrap().iter().collect();
        let num = usize::from_str_radix(&num_binary, 2).unwrap();
        let chunks = chars_vec.get(19..).unwrap().to_vec().chunks(11);
        let chunks_vec: Vec<Vec<char>> = chunks.collect();
    }
    0
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let ans = parse(&input[0]);
    println!("{}", ans);
}
