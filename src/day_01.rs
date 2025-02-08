use std::fs::read_to_string;

fn get_inputs() -> (Vec<i32>, Vec<i32>) {
    let mut left_inputs: Vec<i32> = Vec::new();
    let mut right_inputs: Vec<i32> = Vec::new();

    for line in read_to_string("./src/inputs/day_01.txt").unwrap().lines() {
        if let Some((left, right)) = line.split_once("   ") {
            left_inputs.push(left.parse::<i32>().unwrap());
            right_inputs.push(right.parse::<i32>().unwrap());
        }
    }

    (left_inputs, right_inputs)
}

pub fn part_1() {
    let (mut left_inputs, mut right_inputs) = get_inputs();

    left_inputs.sort();
    right_inputs.sort();

    let mut sum = 0;
    for (l, r) in left_inputs.iter().zip(right_inputs.iter()) {
        sum = sum + (l - r).abs()
    }

    println!("Puzzle answer of day 1 part 1 is {sum}")
}

pub fn part_2() {
    let (left_inputs, right_inputs) = get_inputs();

    let mut sum = 0;
    for l in left_inputs.iter() {
        for r in right_inputs.iter() {
            if l == r {
                sum = sum + l
            }
        }
    }

    println!("Puzzle answer of day 1 part 2 is {sum}")
}
