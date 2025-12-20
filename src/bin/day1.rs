use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").expect("Failed to read input file");

    part_1(&contents);
    println!();
    part_2(&contents);
}

fn part_2(contents: &str) {
    let mut dial: i32 = 50;
    let mut password = 0;
    for line in contents.lines() {
        let turn: i32 = String::from(&line[1..]).parse().expect("Invalid Input");

        if &line[..1] == "R" {
            dial += turn % 100;
        } else if &line[..1] == "L" {
            if dial == 0 {
                password -= 1;
            }
            dial -= turn % 100;
        }

        password += turn / 100;

        if dial == 0 {
            password += 1;
        }

        if dial >= 100 {
            dial -= 100;
            password += 1;
        } else if dial < 0 {
            dial += 100;
            password += 1;
        }
    }

    print!("{} {}", dial, password);
}

fn part_1(contents: &str) {
    let mut dial: i32 = 50;
    let mut password = 0;
    for line in contents.lines() {
        let turn: i32 = String::from(&line[1..]).parse().expect("Invalid Input");

        if &line[..1] == "R" {
            dial += turn % 100;
        } else {
            dial -= turn % 100;
        }

        if dial >= 100 {
            dial -= 100;
        }

        if dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            password += 1;
        }
    }

    print!("{} {}", dial, password);
}
