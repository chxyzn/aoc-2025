use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").expect("Failed to read file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut data: Vec<Vec<&str>> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        let x = split_three(line);
        data.push(x);
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

    //println!("operations - {:?}", operations);
    println!("{}", part1(&problems, &operations))
}

fn split_three(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < s.len() {
        let start = i;
        let end = (i + 3).min(s.len());
        result.push(&s[start..end]);
        i += 4;
    }

    result
}

//fn part2(data: Vec<Vec<&str>>, operations: &[u8]) -> i64 {
//    let mut results: i64 = 0;
//    for i in 0..operations.len() {
//        let mut column: Vec<Vec<u8>> = Vec::new();
//
//        for row in &data {
//            column.push(row[i].as_bytes().to_vec());
//        }
//
//        //for item in column {
//        //    for byte in item {
//        //        print!("{}", byte);
//        //    }
//        //    println!();
//        //}
//
//        let mut res: i64 = operations[i] as i64;
//        for j in 0..3 {
//            let mut num: u64 = 0;
//
//            for row in &column {
//                let byte = row[j];
//                if byte == b' ' {
//                    continue;
//                }
//                let d = byte - b'0';
//                num = num * 10 + d as u64;
//            }
//
//            print!("{} ", num);
//
//            if operations[i] == 0 {
//                res += num as i64;
//            } else {
//                res *= num as i64;
//            }
//        }
//
//        print!("{} {}", res, i);
//        results += res;
//        println!();
//    }
//
//    results
//}

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
