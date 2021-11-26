use util;

#[derive(Debug)]
struct Rule {
    min: i32,
    max: i32,
    c: char,
}

#[derive(Debug)]
struct Password {
    rule: Rule,
    data: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let mut cnt = 0;
        let chars: Vec<char> = self.data.chars().collect();
        for c in chars {
            if c == self.rule.c {
                cnt += 1
            }
        }

        self.rule.min <= cnt && cnt <= self.rule.max
    }
}

fn main() {
    if let Ok(lines) = util::read_lines("input.txt") {
        let l: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let passwords = parse(&l);

        println!("{}", passwords.into_iter().filter(|x| x.is_valid()).count());
    }
}

fn parse(lines: &Vec<String>) -> Vec<Password> {
    let mut v = Vec::new();
    for line in lines {
        let mut iter = line.split_whitespace();
        let mm: Vec<i32> = iter
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let min = mm[0];
        let max = mm[1];

        let c = iter
            .next()
            .unwrap()
            .replace(":", "")
            .chars()
            .next()
            .unwrap();
        let data = iter.next().unwrap();
        let mut rule = Rule { min, max, c };
        let p = Password {
            rule,
            data: data.to_string(),
        };
        v.push(p);
    }
    v
}
