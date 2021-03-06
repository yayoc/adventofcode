use std::{collections::HashMap, fs};

type Path<'a> = Vec<&'a str>;

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn get_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut nodes = line.split('-');
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();
        graph.entry(l).or_insert_with(Vec::new).push(r);
        graph.entry(r).or_insert_with(Vec::new).push(l);
    }
    graph
}

fn count_paths<'a>(
    graph: HashMap<&str, Path<'a>>,
    start: &'a str,
    end: &'a str,
    visited: &mut Vec<&'a str>,
    count: &mut usize,
    can_duplicate: bool,
) {
    visited.push(start);

    if start == end {
        *count += 1;
    } else {
        for adj in graph.get(start).unwrap() {
            if is_lowercase(adj) {
                if !visited.contains(adj) {
                    count_paths(graph.clone(), adj, end, visited, count, can_duplicate);
                } else if can_duplicate && *adj != "start" && *adj != "end" {
                    count_paths(graph.clone(), adj, end, visited, count, false);
                }
            } else {
                count_paths(graph.clone(), adj, end, visited, count, can_duplicate);
            }
        }
    }
    visited.truncate(visited.len() - 1);
}

fn main() {
    let content: String = fs::read_to_string("input.txt").expect("can't read file");
    let graph: HashMap<&str, Vec<&str>> = get_graph(content.as_str());
    let mut visited = vec![];
    let mut count = 0;
    count_paths(graph, "start", "end", &mut visited, &mut count, true);
    println!("{}", count);
}

pub fn solve1(input: &str) -> usize {
    let graph: HashMap<&str, Vec<&str>> = get_graph(input);
    let mut visited = vec![];
    let mut count = 0;
    count_paths(graph, "start", "end", &mut visited, &mut count, false);
    count
}

pub fn solve2(input: &str) -> usize {
    let graph: HashMap<&str, Vec<&str>> = get_graph(input);
    let mut visited = vec![];
    let mut count = 0;
    count_paths(graph, "start", "end", &mut visited, &mut count, true);
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let test1 = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(super::solve1(test1), 10);

        let test2 = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(super::solve1(test2), 19);

        let test3 = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(super::solve1(test3), 226);
    }

    #[test]
    fn part2() {
        let test1 = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(super::solve2(test1), 36);

        let test2 = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(super::solve2(test2), 103);

        let test3 = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(super::solve2(test3), 3509);
    }
}
