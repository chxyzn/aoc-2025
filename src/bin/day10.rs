use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day10.txt").expect("Failed to read file");

    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}

fn part2(content: &str) -> usize {
    let mut total = 0;
    for (kk, line) in content.lines().enumerate() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let dst = tokens.last().unwrap()[1..tokens.last().unwrap().len() - 1]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let n = dst.len();
        let mut buttons: Vec<Vec<u32>> = Vec::new();

        for s in tokens[1..tokens.len() - 1].iter() {
            let indicies = s[1..s.len() - 1]
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let mut button = vec![0; n];
            for i in indicies {
                button[i as usize] = 1;
            }
            buttons.push(button);
        }

        let mut curr = vec![0; n];
        let mut min_steps = usize::MAX;

        dfs(0, &mut curr, &dst, &buttons, 0, &mut min_steps);

        if min_steps != usize::MAX {
            total += min_steps;
        }
        println!("Line {}: min_steps = {}", kk + 1, min_steps);
    }
    total
}

fn dfs(
    btn_idx: usize,
    curr: &mut Vec<u32>,
    dst: &[u32],
    buttons: &[Vec<u32>],
    steps: usize,
    min_steps: &mut usize,
) {
    if steps >= *min_steps {
        return;
    }

    if btn_idx == buttons.len() {
        if curr == dst {
            *min_steps = steps;
        }
        return;
    }

    let button = &buttons[btn_idx];

    let mut max_k = usize::MAX;
    for (i, &b_val) in button.iter().enumerate() {
        if b_val > 0 {
            let remaining = dst[i].saturating_sub(curr[i]);
            let limit = remaining / b_val;
            if (limit as usize) < max_k {
                max_k = limit as usize;
            }
        }
    }

    for k in 0..=max_k {
        if k > 0 {
            for (i, val) in button.iter().enumerate() {
                curr[i] += val;
            }
        }

        dfs(btn_idx + 1, curr, dst, buttons, steps + k, min_steps);
    }

    for _ in 0..max_k {
        for (i, val) in button.iter().enumerate() {
            curr[i] -= val;
        }
    }
}

fn part1(content: &str) -> usize {
    let mut min_steps = 0;
    for line in content.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let indicator = tokens[0][1..tokens[0].len() - 1]
            .chars()
            .collect::<Vec<char>>();

        let mut dst = 0u16;
        for (i, &c) in indicator.iter().enumerate() {
            if c == '#' {
                dst |= 1 << (i);
            }
        }

        let mut buttons: Vec<u16> = Vec::new();
        for button in tokens[1..tokens.len() - 1].iter() {
            let button = button[1..button.len() - 1]
                .split(',')
                .map(|s| s.parse::<u16>().unwrap())
                .fold(0u16, |acc, x| acc | 1 << x);
            buttons.push(button);
        }

        min_steps += bfs(dst, &buttons).unwrap_or(0);
    }
    min_steps
}

fn bfs(dst: u16, buttons: &Vec<u16>) -> Option<usize> {
    let mut queue: VecDeque<(u16, usize)> = VecDeque::new();
    let mut visited: HashSet<u16> = HashSet::new();

    queue.push_back((0, 0));
    visited.insert(0);

    while let Some((current, steps)) = queue.pop_front() {
        if current == dst {
            return Some(steps);
        }

        for &button in buttons {
            let next = current ^ button;
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            }
        }
    }

    None
}
