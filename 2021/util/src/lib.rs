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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
