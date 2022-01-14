use std::collections::HashMap;
use util;

fn bfs(graph: HashMap<&str, Vec<&str>>, start: &str, paths: &mut Vec<Vec<&str>>) {
    let mut queue = vec![];

    while queue.len() > 0 {
        let node = queue.pop();
        if node == "end" {
            break;
        }
    }
}

fn main() {
    let input: Vec<String> = util::parse_input("input.txt").expect("can't parse input");
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for path in &input {
        let mut nodes = path.split("-");
        let l = nodes.nth(0).unwrap();
        let r = nodes.nth(0).unwrap();
        graph.entry(l).or_insert_with(|| vec![]).push(r);
        graph.entry(r).or_insert_with(|| vec![]).push(l);
    }

    let mut paths = vec![];
    for n in graph.get("start") {
        bfs(graph, n, &mut paths);
    }

    println!("{:?}", paths);
    println!("{:?}", paths.len());
}
