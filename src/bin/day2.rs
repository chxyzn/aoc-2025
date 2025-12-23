use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("Failed to read input");
    println!("{}", contents);

    let ranges: Vec<&str> = contents.split(",").collect();
    let mut sum: u64 = 0;

    for r in ranges {
        let mut iter = r.split('-');

        let a = iter.next().unwrap_or("").trim().as_bytes();
        let b = iter.next().unwrap_or("").trim().as_bytes();

        let mut v: Vec<u8> = vec![b'0'; b.len() - a.len()];
        v.extend(a);

        let mut invalid: Vec<Vec<u8>> = Vec::new();
        let v_copy = v.to_vec();

        func(&mut v, &v_copy, b, 0, &mut invalid, part2);

        for x in invalid {
            let y = String::from_utf8(x)
                .expect("failed")
                .parse::<u64>()
                .expect("failed to convert string to u32");
            sum += y;

            //println!("{}", y);
        }
    }

    println!("{}", sum);
}

fn part1(a_slice: &[u8]) -> bool {
    if a_slice.len() % 2 == 1 {
        return false;
    }
    let half = a_slice.len() / 2;

    for j in 0..half {
        if a_slice[j] != a_slice[j + half] {
            return false;
        }
    }

    true
}

fn part2(a_slice: &[u8]) -> bool {
    let n = a_slice.len();

    if n < 2 {
        return false;
    }

    let mut v = a_slice.to_vec();
    v.extend_from_slice(a_slice);

    let t = &v[1..v.len() - 1];

    for i in 0..=t.len() - a_slice.len() {
        let mut found = true;
        for j in 0..a_slice.len() {
            if a_slice[j] != t[i + j] {
                found = false;
                break;
            }
        }

        if found {
            return true;
        }
    }

    false
}

fn func(
    a: &mut [u8],
    aa: &[u8],
    b: &[u8],
    i: usize,
    invalid: &mut Vec<Vec<u8>>,
    invalidator: fn(&[u8]) -> bool,
) {
    if i >= a.len() {
        let mut zi = 0;

        for x in a.iter() {
            if *x != b'0' {
                break;
            }
            zi += 1;
        }

        let a_slice = &a[zi..];

        let inv = invalidator(a_slice);

        if inv {
            invalid.push(a_slice.to_vec());
        }

        return;
    }

    for d in 0..10 {
        a[i] = d + b'0';

        if cmp(a, aa) < 0 {
            continue;
        }

        if cmp(a, b) > 0 {
            break;
        }

        func(a, aa, b, i + 1, invalid, invalidator);
    }

    a[i] = b'0';
}

fn cmp(a: &[u8], b: &[u8]) -> i8 {
    for (x, y) in a.iter().zip(b.iter()) {
        if x > y {
            return 1;
        }
        if x < y {
            return -1;
        }
    }

    0
}
