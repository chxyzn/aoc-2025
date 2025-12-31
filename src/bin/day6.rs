use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").expect("Failed to read file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut data: Vec<Vec<char>> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        data.push(line.chars().collect::<Vec<char>>());
    }

    let problems: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let operations: Vec<u8> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| if x == "+" { 0 } else { 1 })
        .collect();

    println!("Part 1 : {}", part1(&problems, &operations));
    println!("Part 2 : {}", part2(data, &operations));
}

fn part2(data: Vec<Vec<char>>, operations: &[u8]) -> i64 {
    let mut results: i64 = 0;

    let mut j = 0;
    for &op in operations {
        let mut res = op as i64;

        loop {
            let mut only_spaces = true;

            for line in &data {
                if j >= line.len() {
                    break;
                }
                if line[j] != ' ' {
                    only_spaces = false;
                    break;
                }
            }

            if only_spaces {
                j += 1;
                break;
            }

            let mut num = 0;
            for line in &data {
                let d = line[j] as u8;
                if d == b' ' {
                    continue;
                }
                num = num * 10 + (d - b'0') as u64;
            }
            if op == 0 {
                res += num as i64;
            } else {
                res *= num as i64;
            }

            j += 1;
        }

        results += res;
    }

    results
}

fn part1(problems: &[Vec<i64>], operations: &[u8]) -> i64 {
    let mut results = problems[0].clone();

    for row in problems.iter().skip(1) {
        for (i, number) in row.iter().enumerate() {
            if operations[i] == 0 {
                results[i] += number;
            } else {
                results[i] *= number;
            }
        }
    }

    results.iter().sum::<i64>()
}
