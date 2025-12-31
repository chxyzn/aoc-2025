use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day11.txt").expect("Failed to read file");
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in content.lines() {
        let mut itr = line.split(':');
        let node = itr.next().unwrap();
        let neigh: Vec<&str> = itr.next().unwrap().split_whitespace().collect();
        graph.insert(node, neigh);
    }

    let mut count = 0;
    dfs("you", "", &graph, &mut count);
    println!("{:?}", count);

    let mut memo: HashMap<(&str, u8), u64> = HashMap::new();
    let count: u64 = dfs_2("svr", 0, &graph, &mut memo);
    println!("{:?}", count);
}

fn dfs_2<'a>(
    curr: &'a str,
    mut state: u8,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, u8), u64>,
) -> u64 {
    if curr == "dac" {
        state |= 1;
    } else if curr == "fft" {
        state |= 2;
    }

    if curr == "out" {
        return if state == 3 { 1 } else { 0 };
    }

    if let Some(&cached_count) = memo.get(&(curr, state)) {
        return cached_count;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(curr) {
        for &next in neighbors {
            total_paths += dfs_2(next, state, graph, memo);
        }
    }

    memo.insert((curr, state), total_paths);
    total_paths
}

fn dfs(curr: &str, parent: &str, graph: &HashMap<&str, Vec<&str>>, count: &mut u32) {
    for &next in &graph[curr] {
        if next == parent {
            continue;
        }
        if next == "out" {
            *count += 1;
            continue;
        }
        dfs(next, curr, graph, count);
    }
}

