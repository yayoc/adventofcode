use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_input<P, T>(filename: P) -> Result<Vec<T>, T::Err>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let file = File::open(filename).expect("can't open file");
    let lines = io::BufReader::new(file).lines();
    lines.map(|l| l.unwrap().parse::<T>()).collect()
}

pub fn counter<T, I>(it: I) -> HashMap<T, usize>
where
    T: Eq + Hash,
    I: Iterator<Item = T>,
{
    let mut count_by_element = HashMap::new();
    for e in it {
        *count_by_element.entry(e).or_insert(0) += 1;
    }
    count_by_element
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
