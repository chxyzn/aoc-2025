use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day10.txt").expect("Failed to read file");
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}

fn part2(content: &str) -> usize {
    let mut total = 0;
    for line in content.lines() {
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

        let mut memo = HashMap::new();
        let res = solve_recursive(dst, &buttons, &mut memo);
        //println!("Result for line: {}", res);
        if res != usize::MAX {
            total += res;
        }
    }
    total
}

fn get_valid_buttons(
    btn_list: &[Vec<u32>],
    valid_list: &mut Vec<Vec<u32>>,
    valid_set: &mut Vec<Vec<Vec<u32>>>,
    dst: &mut Vec<u32>,
    i: usize,
) {
    if i == btn_list.len() {
        if dst.iter().all(|&x| x == 0) {
            valid_set.push(valid_list.clone());
        }
        return;
    }
    let btn = &btn_list[i];
    get_valid_buttons(btn_list, valid_list, valid_set, dst, i + 1);
    for (idx, &b_val) in btn.iter().enumerate() {
        dst[idx] ^= b_val;
    }
    valid_list.push(btn.clone());
    get_valid_buttons(btn_list, valid_list, valid_set, dst, i + 1);
    valid_list.pop();
    for (idx, &b_val) in btn.iter().enumerate() {
        dst[idx] ^= b_val;
    }
}

fn solve_recursive(
    target: Vec<u32>,
    buttons: &[Vec<u32>],
    memo: &mut HashMap<Vec<u32>, usize>,
) -> usize {
    if target.iter().all(|&x| x == 0) {
        return 0;
    }
    if let Some(&res) = memo.get(&target) {
        return res;
    }
    let mut parity_target: Vec<u32> = target.iter().map(|&x| x % 2).collect();
    let mut valid_sets = Vec::new();
    let mut current_subset = Vec::new();
    get_valid_buttons(
        buttons,
        &mut current_subset,
        &mut valid_sets,
        &mut parity_target,
        0,
    );
    let mut min_presses = usize::MAX;
    for subset in valid_sets {
        let mut new_target = target.clone();
        let mut possible = true;
        for btn in &subset {
            for (val, &b_val) in new_target.iter_mut().zip(btn.iter()) {
                if *val < b_val {
                    possible = false;
                }
                *val = val.saturating_sub(b_val);
            }
        }
        if possible {
            for val in new_target.iter_mut() {
                *val /= 2;
            }
            let res = solve_recursive(new_target, buttons, memo);
            if res != usize::MAX {
                let cost = subset.len() + 2 * res;
                if cost < min_presses {
                    min_presses = cost;
                }
            }
        }
    }
    memo.insert(target, min_presses);
    min_presses
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
