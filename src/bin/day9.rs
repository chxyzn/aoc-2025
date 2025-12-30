use std::fs;

struct Point(i64, i64);

fn to_point(s: &str) -> Point {
    let p: Vec<i64> = s.split(',').map(|x| x.parse().unwrap()).collect();
    Point(p[0], p[1])
}

fn area(p: &Point, q: &Point) -> i64 {
    let width = (p.0 - q.0).abs() + 1;
    let height = (p.1 - q.1).abs() + 1;
    width * height
}

fn main() {
    let content = fs::read_to_string("inputs/day9.txt").expect("Failed to read file");
    let lines = content.lines();
    let points: Vec<Point> = lines.map(to_point).collect();

    println!("Part 1: {}", part1(&points));
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
