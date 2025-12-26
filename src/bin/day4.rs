use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day4.txt").expect("Failed to read file");

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let dirs = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];

    println!("Part 1: {}", part1(&grid, &dirs));
    println!("Part 2: {}", part2(&grid, &dirs));
}

fn part2(grid: &[Vec<char>], dirs: &[(isize, isize)]) -> usize {
    let mut current_grid = grid.to_vec();
    let mut paperrolls = 0;

    loop {
        let mut to_remove = Vec::new();
        let rows = current_grid.len();
        let cols = current_grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if current_grid[r][c] == '@' {
                    let mut count = 0;
                    for (dx, dy) in dirs {
                        let nx = r as isize + dx;
                        let ny = c as isize + dy;

                        if nx >= 0
                            && nx < rows as isize
                            && ny >= 0
                            && ny < cols as isize
                            && current_grid[nx as usize][ny as usize] == '@'
                        {
                            count += 1;
                        }
                    }

                    if count < 4 {
                        to_remove.push((r, c));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        paperrolls += to_remove.len();

        for (r, c) in to_remove {
            current_grid[r][c] = '.';
        }
    }

    paperrolls
}

fn part1(grid: &[Vec<char>], dirs: &[(isize, isize)]) -> u32 {
    let mut paperrolls = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut count = 0;
            for (dx, dy) in dirs {
                let x = i as isize + dx;
                let y = j as isize + dy;
                if x >= 0
                    && x < grid.len() as isize
                    && y >= 0
                    && y < grid[i].len() as isize
                    && grid[x as usize][y as usize] == '@'
                {
                    count += 1;
                }
            }
            if count < 4 && grid[i][j] == '@' {
                paperrolls += 1;
            }
        }
    }

    paperrolls
}

