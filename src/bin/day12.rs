use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day12.txt").expect("Failed to read file");

    let mut shapes: Vec<u32> = Vec::new();
    let mut itr = content.lines();

    let mut count = 0;

    while let Some(line) = itr.next() {
        if line.trim().is_empty() {
            continue;
        }

        let split = line.trim().split(':').collect::<Vec<&str>>();
        if !split[0].contains('x') {
            let mut shape_size = 0;
            for _ in 0..3 {
                let next = itr.next().unwrap().trim();

                for ch in next.chars() {
                    if ch == '#' {
                        shape_size += 1;
                    }
                }
            }
            shapes.push(shape_size);
        } else {
            let grid = split[0]
                .split('x')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let requirements = split[1]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            count += solve(grid, requirements, &shapes);
        }
    }
    println!("Part 1: {}", count);
}

fn solve(grid: Vec<u32>, requirements: Vec<u32>, shapes: &[u32]) -> u32 {
    let area = grid.iter().product::<u32>();
    let mut occupied = 0;
    shapes.iter().enumerate().fold(occupied, |_, (i, shape)| {
        let req = requirements[i];
        let shape_area = shape * req;
        occupied += shape_area;
        occupied
    });

    if occupied >= area { 0 } else { 1 }
}
