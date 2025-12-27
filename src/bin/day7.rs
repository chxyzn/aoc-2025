use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day7.txt").expect("Failed to read file");

    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}

fn part2(content: &str) -> u64 {
    let mut grid: Vec<Vec<u8>> = vec![];

    for line in content.lines() {
        let row: Vec<u8> = line.bytes().collect();
        grid.push(row);
    }

    let mut index = 0;
    for (i, &byte) in grid[0].iter().enumerate() {
        if byte == b'S' {
            index = i;
            break;
        }
    }

    let mut timelines: Vec<Vec<u64>> = vec![vec![1; grid[0].len()]; grid.iter().len()];
    for i in (0..grid.len() - 1).rev() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'.' {
                timelines[i][j] = timelines[i + 1][j];
            } else {
                timelines[i][j] = 0;
                if i > 0 {
                    timelines[i][j] += timelines[i + 1][j - 1];
                }
                if i + 1 < timelines.len() {
                    timelines[i][j] += timelines[i + 1][j + 1];
                }
            }
        }
    }

    timelines[1][index]
}

fn part1(content: &str) -> u32 {
    let mut lines = content.lines();

    let init = lines.next().unwrap();
    let mut active_row = vec![0u32; init.len()];

    let start = init.find('S').unwrap();
    active_row[start] = 1;

    let mut split_count = 0;
    for (i, line) in lines.enumerate() {
        let i = i as u32;
        let line = line.as_bytes();
        let mut changes = vec![0u32; init.len()];
        for (j, cell) in active_row.iter().enumerate() {
            if *cell == i + 1 {
                if line[j] == b'.' {
                    changes[j] = i + 2;
                } else {
                    split_count += 1;
                    changes[j] = 0;
                    changes[j + 1] = i + 2;
                    changes[j - 1] = i + 2;
                }
            }
        }

        active_row = changes;
    }
    split_count
}

