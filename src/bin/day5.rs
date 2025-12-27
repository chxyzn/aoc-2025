use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day5.txt").expect("Failed to read file");
    let mut lines = contents.lines();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let parts: Vec<u64> = line.split('-').map(|x| x.parse().unwrap()).collect();
        ranges.push((parts[0], parts[1]));
    }

    for line in lines {
        ingredients.push(line.parse().unwrap());
    }

    ranges.sort_unstable();

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    let mut curr = ranges[0];

    for range in ranges.iter().skip(1) {
        if curr.1 >= range.0 {
            curr.1 = curr.1.max(range.1);
        } else {
            merged_ranges.push(curr);
            curr = *range;
        }
    }
    merged_ranges.push(curr);
    println!("{}", part1(&merged_ranges, &ingredients));
    println!("{}", part2(&merged_ranges));
}

fn part2(ranges: &[(u64, u64)]) -> u64 {
    ranges.iter().map(|(l, r)| r - l + 1).sum()
}

fn part1(ranges: &[(u64, u64)], ingredients: &Vec<u64>) -> u32 {
    let mut fresh_ingredients = 0;

    for ingredient in ingredients {
        if binary_search(*ingredient, ranges) {
            fresh_ingredients += 1;
        }
    }

    fresh_ingredients
}

fn binary_search(x: u64, ranges: &[(u64, u64)]) -> bool {
    let mut low = 0i64;
    let mut high = (ranges.len() as i64) - 1;

    while low <= high {
        let mid = (low + (high - low) / 2) as usize;
        let candidate = ranges[mid];

        if candidate.0 <= x && x <= candidate.1 {
            return true;
        } else if candidate.0 > x {
            high = mid as i64 - 1;
        } else {
            low = mid as i64 + 1;
        }
    }
    false
}
