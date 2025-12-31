use std::cmp::{max, min};
use std::fs;

struct Point(i64, i64);

fn main() {
    let content = fs::read_to_string("inputs/temp.txt").expect("Failed to read file");
    let points: Vec<Point> = content.lines().map(to_point).collect();

    let mut xs: Vec<i64> = points.iter().map(|p| p.0).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.1).collect();

    xs.sort_unstable();
    ys.sort_unstable();
    xs.dedup();
    ys.dedup();

    let rows = ys.len() - 1;
    let cols = xs.len() - 1;

    let mut grid = vec![vec![0; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let mid_x = (xs[c] as f64 + xs[c + 1] as f64) / 2.0;
            let mid_y = (ys[r] as f64 + ys[r + 1] as f64) / 2.0;

            if !point_in_polygon(mid_x, mid_y, &points) {
                grid[r][c] = 1;
            }
        }
    }

    let mut psum = vec![vec![0; cols + 1]; rows + 1];
    for r in 0..rows {
        for c in 0..cols {
            psum[r + 1][c + 1] = psum[r][c + 1] + psum[r + 1][c] - psum[r][c] + grid[r][c];
        }
    }

    let mut max_area: i64 = 0;

    for i in 0..points.len() {
        for j in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            let x1_idx = xs.binary_search(&min(p1.0, p2.0)).unwrap();
            let x2_idx = xs.binary_search(&max(p1.0, p2.0)).unwrap();
            let y1_idx = ys.binary_search(&min(p1.1, p2.1)).unwrap();
            let y2_idx = ys.binary_search(&max(p1.1, p2.1)).unwrap();

            if x1_idx == x2_idx || y1_idx == y2_idx {
                continue;
            }

            let bad_count = query_psum(&psum, y1_idx, x1_idx, y2_idx, x2_idx);

            if bad_count == 0 {
                let width = (p1.0 - p2.0).abs() + 1;
                let height = (p1.1 - p2.1).abs() + 1;
                let area = width * height;
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    println!("Part 1: {}", part1(&points));
    println!("Part 2: {}", max_area);
}

fn to_point(s: &str) -> Point {
    let p: Vec<i64> = s.split(',').map(|x| x.parse().unwrap()).collect();
    Point(p[0], p[1])
}

fn area(p: &Point, q: &Point) -> i64 {
    let width = (p.0 - q.0).abs() + 1;
    let height = (p.1 - q.1).abs() + 1;
    width * height
}

fn part1(points: &[Point]) -> i64 {
    let mut max_area: i64 = 0;
    for p in points {
        for q in points {
            let a = area(p, q);
            if a > max_area {
                max_area = a;
            }
        }
    }
    max_area
}

fn query_psum(psum: &[Vec<i32>], r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    psum[r2][c2] - psum[r1][c2] - psum[r2][c1] + psum[r1][c1]
}

fn point_in_polygon(x: f64, y: f64, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;

    for i in 0..n {
        let xi = polygon[i].0 as f64;
        let yi = polygon[i].1 as f64;
        let xj = polygon[j].0 as f64;
        let yj = polygon[j].1 as f64;

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}
