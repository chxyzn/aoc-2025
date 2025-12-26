use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").expect("failed to read input file");

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

fn part2(contents: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in contents.lines() {
        let mut selected_batteries = [0u8; 12];
        let batteries: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut index = 0;

        for i in 0..12 {
            let (imaxb, maxb) = batteries[index..batteries.len() - (12 - i - 1)]
                .iter()
                .copied()
                .enumerate()
                .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
                .unwrap();

            //        println!(
            //            "i: {}, imaxb: {},imaxb + index {}, maxb: {}",
            //            i,
            //            imaxb,
            //            imaxb + index,
            //            maxb
            //        );

            selected_batteries[i] = maxb as u8;
            index += imaxb + 1;
        }

        let mut jolts: u64 = 0;
        for (i, b) in selected_batteries.iter().enumerate() {
            jolts += (*b as u64) * 10u64.pow(11 - i as u32);
        }
        sum += jolts;
        println!("{}", jolts);
    }
    sum
}

fn part1(contents: &str) -> u32 {
    let mut sum = 0;

    for line in contents.lines() {
        let batteries: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

        let (imaxb, maxb) = batteries
            .iter()
            .copied()
            .enumerate()
            .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
            .unwrap();

        let b2 = if imaxb == line.len() - 1 {
            let (_, b2) = batteries[..imaxb]
                .iter()
                .copied()
                .enumerate()
                .max_by_key(|(_, v)| *v)
                .unwrap();
            b2
        } else {
            let (_, b2) = batteries[imaxb + 1..]
                .iter()
                .copied()
                .enumerate()
                .max_by_key(|(_, v)| *v)
                .unwrap();
            b2
        };

        if imaxb == line.len() - 1 {
            println!("{}", maxb + b2 * 10);
            sum += maxb + b2 * 10;
        } else {
            println!("{}", maxb * 10 + b2);
            sum += maxb * 10 + b2;
        }
    }
    sum
}
