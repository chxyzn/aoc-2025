use std::fs;
use std::mem::swap;

struct Point(i32, i32, i32);

fn main() {
    let content = fs::read_to_string("inputs/day8.txt").expect("Failed to read file");
    let points: Vec<Point> = content.lines().map(to_point).collect();

    let mut distances: Vec<(f32, usize, usize)> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        for (j, q) in points.iter().enumerate() {
            if j <= i {
                continue;
            }
            distances.push((distance(p, q), i, j));
        }
    }
    distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut dsu: Vec<usize> = (0..points.len()).collect();
    let mut dsu_size = vec![1u32; points.len()];

    let p1 = part1(&distances, &mut dsu, &mut dsu_size);
    let p2 = part2(&distances, &mut dsu, &mut dsu_size, &points);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part2(
    distances: &[(f32, usize, usize)],
    dsu: &mut [usize],
    dsu_size: &mut [u32],
    points: &[Point],
) -> u64 {
    let n = points.len() as u32;
    for &(_, x, y) in distances.iter() {
        dsu_unite(dsu, dsu_size, x, y);
        let px = dsu_parent(dsu, x);
        let py = dsu_parent(dsu, y);
        if dsu_size[px] == n || dsu_size[py] == n {
            return (points[x].0 as u64) * (points[y].0 as u64);
        }
    }
    0
}

fn part1(distances: &[(f32, usize, usize)], dsu: &mut [usize], dsu_size: &mut [u32]) -> u32 {
    for &(_, x, y) in distances.iter().take(1000) {
        dsu_unite(dsu, dsu_size, x, y);
    }
    let mut copy: Vec<u32> = dsu_size.to_vec();
    copy.sort_unstable();
    copy.reverse();
    copy[0] * copy[1] * copy[2]
}

fn dsu_parent(dsu: &mut [usize], x: usize) -> usize {
    if dsu[x] != x {
        dsu[x] = dsu_parent(dsu, dsu[x]);
    }
    dsu[x]
}

fn dsu_unite(dsu: &mut [usize], dsu_size: &mut [u32], x: usize, y: usize) -> bool {
    let mut x = dsu_parent(dsu, x);
    let mut y = dsu_parent(dsu, y);
    if x == y {
        false
    } else {
        if dsu_size[y] > dsu_size[x] {
            swap(&mut x, &mut y);
        }
        dsu[y] = x;
        dsu_size[x] += dsu_size[y];
        dsu_size[y] = 0;
        true
    }
}

fn to_point(s: &str) -> Point {
    let parts: Vec<i32> = s.split(',').map(|x| x.trim().parse().unwrap()).collect();
    Point(parts[0], parts[1], parts[2])
}

fn distance(p: &Point, q: &Point) -> f32 {
    let dx = (p.0 - q.0) as f32;
    let dy = (p.1 - q.1) as f32;
    let dz = (p.2 - q.2) as f32;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
